
<script setup lang="ts">
    import { useConfigStore } from '../store';
    import * as rust from '../rust_funcs';
    import { ActionNode, RootNode, ChanceNode, PlayerNode, TerminalNode } from '../typing';
    import { computed, ref } from 'vue';

    const config = useConfigStore();
    let locked = false;
    const navDiv = ref(null as HTMLDivElement | null);

    const treeError = !(await rust.buildActionTree(
        config.board,
        config.startingPot,
        config.effectiveStack,
        config.rake / 100,
        config.rakeCap,
        config.addAllInThreshold / 100,
        config.forceAllInThreshold / 100,
        
        config.betSizes[0][0][0],
        config.betSizes[0][0][1],
        config.betSizes[0][1][0],
        config.betSizes[0][1][1],
        config.betSizes[0][2][0],
        config.betSizes[0][2][1],

        config.betSizes[1][0][0],
        config.betSizes[1][0][1],
        config.betSizes[1][1][0],
        config.betSizes[1][1][1],
        config.betSizes[1][2][0],
        config.betSizes[1][2][1],
    ));
    
    const rootNode: RootNode = {
        type: "root",
        idx: 0,
        player: config.board.length === 3 ? "flop" : config.board.length === 4 ? "turn" : "river",
        selectedIdx: -1,
        board: config.board,
        pot: config.startingPot,
        stack: config.effectiveStack,
    };

    const nodes = ref<ActionNode[]>([rootNode]);
    const selectedNodeIdx = ref(-1);
    const betAmount = ref(0);
    const prevBetAmount = ref(0);
    const totalBetAmount = ref([0, 0]);

    const isSelectedTerminal = computed(() => {
        if (locked || selectedNodeIdx.value === -1) return false;
        const selectedNode = nodes.value[selectedNodeIdx.value];
        return selectedNode.type === "terminal";
    });

    const afterAllIn = computed(() => {
        const maxTotalBetAmount = Math.max(...totalBetAmount.value);
        return maxTotalBetAmount === config.effectiveStack;
    });

    const maxAmount = computed(() => {
        if (isSelectedTerminal.value) return 0;
        const maxTotalBetAmount = Math.max(...totalBetAmount.value);
        return config.effectiveStack - (maxTotalBetAmount - prevBetAmount.value);
    });
    
    const minAmount = computed(() => {
        const betMinus = config.effectiveStack - maxAmount.value;
        const min = Math.min(...totalBetAmount.value) - betMinus;
        const max = Math.max(...totalBetAmount.value) - betMinus;
        return Math.min(Math.max(2 * max - min, 1), maxAmount.value);
    });
    
    const amountRate = computed(() => {
        const pot = config.startingPot + 2 * Math.max(...totalBetAmount.value);
        const amount = betAmount.value - prevBetAmount.value;
        return amount / pot;
    });

    const encodeLine = (spotIndex: number) => {
        const ret: string[] = [];
        for (let i = 1; i < spotIndex; ++i) {
            const spot = nodes.value[i];
            if (spot.type === "player") {
            const action = spot.actions[spot.selectedIdx];
            if (action.name === "Fold") {
                ret.push("F");
            } else if (action.name === "Check") {
                ret.push("X");
            } else if (action.name === "Call") {
                ret.push("C");
            } else if (action.name === "Bet") {
                ret.push("B" + action.amount);
            } else if (action.name === "Raise") {
                ret.push("R" + action.amount);
            } else if (action.name === "AllIn") {
                ret.push("A" + action.amount);
            }
            }
        }
        return ret;
    };

    const pushResultsTerminal = () => {
        const prevNode = nodes.value[selectedNodeIdx.value - 1] as PlayerNode;
        const prevAction = prevNode.actions[prevNode.selectedIdx];
        let equityOOP = -1;
        if (prevAction.name === "Fold") {
            equityOOP = prevNode.player === "oop" ? 0 : 1;
        }

        nodes.value.push({
            type: "terminal",
            idx: selectedNodeIdx.value,
            player: "end",
            selectedIdx: -1,
            prevPlayer: prevNode.player,
            equityOOP,
            pot: config.startingPot + totalBetAmount.value[0] + totalBetAmount.value[1],
        });
    };

    const pushResultsChance = async () => {
        type TurnNode = RootNode | ChanceNode;
        const prevNode = nodes.value[selectedNodeIdx.value - 1] as PlayerNode;
        const turnNode = nodes.value.find((node) => node.player === "turn") as TurnNode | undefined;
        const nxtActions = await rust.getActionsActionTree();

        nodes.value.push(
            {
                type: "chance",
                idx: selectedNodeIdx.value,
                player: turnNode ? "river" : "turn",
                selectedIdx: -1,
                prevPlayer: prevNode.player,
                cards: Array.from({ length: 52 }, (_, i) => ({
                    card: i,
                    selected: false,
                    dead: true,
                })),
                pot: config.startingPot + 2 * totalBetAmount.value[0],
                stack: config.effectiveStack - totalBetAmount.value[0],
            },
            {
                type: "player",
                idx: selectedNodeIdx.value + 1,
                player: "oop",
                selectedIdx: -1,
                actions: nxtActions.map((action, i) => {
                    const [name, amount] = action.split(":");
                    return {
                        idx: i,
                        name,
                        amount,
                        rate: -1,
                        selected: false,
                        colour: "#000",
                    };
                }),
            }
        );
    };

    const pushResultsPlayer = async () => {
        const prevNode = nodes.value[selectedNodeIdx.value - 1];
        const player = prevNode.player === "oop" ? "ip" : "oop";
        const actions = await rust.getActionsActionTree();

        nodes.value.push({
            type: "player",
            idx: selectedNodeIdx.value,
            player,
            selectedIdx: -1,
            actions: actions.map((action, i) => {
                const [name, amount] = action.split(":");
                return {
                    idx: i,
                    name,
                    amount,
                    selected: false,
                    colour: "#000",
                };
            }),
        });
    };

    const selectNode = async (nodeIdx: number, splice: boolean, rebuild: boolean, updateAmount: boolean) => {

        if (!splice && !rebuild && nodeIdx === selectedNodeIdx.value) return;

        if (nodeIdx === 0) {
            await selectNode(1, true, false, selectedNodeIdx.value !== -1);
            return;
        }

        if (!splice && nodes.value[nodeIdx]?.type === "chance") {
            await selectNode(nodeIdx + 1, false, true, false);
            return;
        }

        locked = true;

        if (rebuild) {
            
            const selectedNodeIdxTemp = selectedNodeIdx.value;
            const line = encodeLine(nodes.value.length - 1);
            nodes.value = [rootNode];

            selectedNodeIdx.value = 1;
            totalBetAmount.value = [0, 0];

            await rust.toRootActionTree();
            await pushResultsPlayer();

            for (let i = 0; i < line.length; ++i) {
                
                const idx = await rust.playActionTree(line[i]);
                if (idx === -1) {
                    updateAmount = false;
                    break;
                }

                const node = nodes.value[selectedNodeIdx.value] as PlayerNode;
                const action = node.actions[idx];
                node.selectedIdx = idx;
                action.selected = true;
                ++selectedNodeIdx.value;
                totalBetAmount.value = await rust.totalBetAmountActionTree();

                if (await rust.isTerminalNodeActionTree()) {
                    pushResultsTerminal();
                } else if (await rust.isChanceNodeActionTree()) {
                    await pushResultsChance();
                    ++selectedNodeIdx.value;
                } else {
                    await pushResultsPlayer();
                }
            }
            
            if (selectedNodeIdxTemp < selectedNodeIdx.value) {
                selectedNodeIdx.value = selectedNodeIdxTemp;
            }
        
        } else {
            selectedNodeIdx.value = nodeIdx;
        }

        const line = encodeLine(selectedNodeIdx.value);
        await rust.applyHistoryActionTree(line);
        totalBetAmount.value = await rust.totalBetAmountActionTree();

        if (splice) {
            nodes.value.splice(selectedNodeIdx.value);
            if (await rust.isTerminalNodeActionTree()) {
                pushResultsTerminal();
            } else if (await rust.isChanceNodeActionTree()) {
                await pushResultsChance();
                ++selectedNodeIdx.value;
            } else {
                await pushResultsPlayer();
            }
        }

        const prev = nodes.value[selectedNodeIdx.value - 1];
        if (prev.type === "player") {
            prevBetAmount.value = Number(prev.actions[prev.selectedIdx].amount);
        } else {
            prevBetAmount.value = 0;
        }

        if (updateAmount) {
            betAmount.value = minAmount.value;
        }

        locked = false;
        // navScroll();
    };

    const play = async (nodeIdx: number, actionIdx: number) => {
        const node = nodes.value[nodeIdx] as PlayerNode;

        if (node.selectedIdx !== -1) {
            node.actions[node.selectedIdx].selected = false;
        }

        node.actions[actionIdx].selected = true;
        node.selectedIdx = actionIdx;
        await selectNode(nodeIdx + 1, true, false, true);
    };

    await selectNode(0, true, false, true);
</script>

<template>
    <!-- v-if="!treeError" -->
    <div 
        ref="navDiv"
        class="flex flex-row h-40 gap-2 p-1 whitespace-nowrap overflow-x-auto snug justify-center"
    >
        <div
            v-for="node in nodes"
            :key="node.idx"
            :class="
                'flex flex-col h-full p-1 justify-start border-2 rounded-md border-neutral '
                + (node.type === 'chance' || node.type === 'root' ? 'border-secondary' : 'border-neutral')
            "
            @click="selectNode(node.idx, false, false, true)"
        >

            <!-- Root/Chance -->
            <template v-if="node.type === 'root' || node.type === 'chance'" class="border-secondary">
                <div class="p-1 font-semibold">
                    {{ node.player.toUpperCase() }}
                </div>
                <div class="flex flex-col flex-grow px-3 items-center justify-evenly font-semibold">
                    <div class="group-hover:opacity-100 opacity-70">
                        <div>Pot: {{ node.pot }}</div>
                        <div>Stack: {{ node.stack }}</div>
                    </div>
                </div>
            </template>

            <!-- Player -->
            <template v-else-if="node.type === 'player'">
                <div class="px-1 py-1 font-semibold">
                    {{ node.player.toUpperCase() }}
                </div>
                <div class="flex-grow overflow-y-auto">
                    <button
                        v-for="action in node.actions"
                        :key="action.idx"
                        :class="'items-center flex w-full px-1 rounded-md transition-colors hover:bg-base-100' +
                            (action.selected ? ' bg-base-200' : '')"
                        @click.stop="play(node.idx, action.idx)"
                    >
                        <span class="inline-block relative w-4 mr-0.5">
                            <span v-if="action.selected">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                                    <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                                </svg>
                            </span>
                        </span>
                        {{ action.name }}
                        {{ action.amount === "0" ? "" : action.amount }}
                    
                    </button>
                    <div
                        v-if="node.actions.length === 0"
                        :class="'flex w-full px-1 font-semibold group-hover:opacity-100 ' 
                        + (node.idx === selectedNodeIdx ? '' : 'opacity-70')"
                    >
                        No Actions
                    </div>
                </div>
            </template>

            <!-- Terminal -->
            <template v-else-if="node.type === 'terminal'">
                <div class="px-1 pt-1 pb-0.5 font-semibold group-hover:opacity-100">
                    {{ node.player.toUpperCase() }}
                </div>
                <div class="flex flex-col flex-grow justify-evenly font-semibold group-hover:opacity-100">
                    <div v-if="node.equityOOP === 0 || node.equityOOP === 1" class="px-3">
                        {{ ["IP", "OOP"][node.equityOOP] }} Wins
                    </div>
                    <div class="px-3">Pot: {{ node.pot }}</div>
                </div>
            </template>
        </div>
    </div>
</template>