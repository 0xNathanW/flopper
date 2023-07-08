
<script setup lang="ts">
    import { computed, ref, toRefs, watch } from 'vue';
    import { ActionNode, ChanceNode, ChanceReport, PlayerNode, Results, RootNode } from '../typing';
    import { average, rgbToString, idxToCard } from '../util';
    import { useConfigStore } from '../store';
    import * as rust from '../rust_funcs';

    const props = defineProps<{
        handlerUpdated: boolean;
        locked: boolean;
        cards: number[][];
        dealtCard: number;
    }>();

    const emit = defineEmits<{
        (event: "update:is-handler-updated", value: boolean): void;
        (event: "update:is-locked", value: boolean): void;
        (
            event: "trigger-update",
            selectedNode: ActionNode | null,
            selectedChance: ChanceNode | null,
            currentBoard: number[],
            results: Results,
            chanceReports: ChanceReport | null,
            totalBetAmount: number[],
        ): void;
    }>();

    const navDiv = ref<HTMLDivElement | null>(null);
    const config = useConfigStore();
    const { handlerUpdated, dealtCard } = toRefs(props);
    const nodes = ref<ActionNode[]>([]);
    const rates = ref<number[] | null>(null);
    const selectedNodeIdx = ref(-1);
    const selectedChanceIdx = ref(-1);
    const isDealing = ref(false);
    const canChanceReport = ref(false);

    let selectedNodeIdxTemp = -1;
    let selectedChanceIdxTemp = -1;
    let results: Results | null = null;
    let chanceReport: ChanceReport | null = null;
    let totalBetAmount: number[] = [0, 0];
    let totalBetAmountAppended = [0, 0];

    const selectedNode = computed(() => 
        selectedNodeIdx.value === -1 || selectedNodeIdx.value >= nodes.value.length
            ? null
            : nodes.value[selectedNodeIdx.value]
    );

    const selectedChance = computed(() =>
        selectedChanceIdx.value === -1 
            ? null 
            : (nodes.value[selectedChanceIdx.value] as ChanceNode)
    );

    const isSelectedChanceSkipped = computed(() => {
        return selectedChance.value?.selectedIdx === -1;
    });

    const currentBoard = computed(() => {
        const board = [...config.board];
        const endIdx = selectedChance.value ? selectedChance.value.selectedIdx : selectedNodeIdx.value;
        
        for (let i = 3; i < endIdx; i++) {
            const node = nodes.value[i];
            
            if (node.type === "chance") {
                const card = node.selectedIdx;
                if (card !== -1) {
                    board.push(card);
                } else {
                    return board;
                }
            }
        }

        return board;
    });

    watch(handlerUpdated, async () => {

        if (!handlerUpdated.value) return;
        const len = config.board.length;
        const node: RootNode = {
            type: "root",
            idx: 0,
            player: len === 3 ? "flop" : len === 4 ? "turn" : "river",
            selectedIdx: -1,
            board: config.board,
            pot: config.startingPot,
            stack: config.effectiveStack,
        };
        nodes.value = [node];

        await selectNode(1, true);
        emit("update:is-handler-updated", false);
    });

    const selectNode = async (
        nodeIdx: number,
        splice: boolean,
        fromDeal = false,
    ) => {

        if (props.locked || 
        (!splice && ((nodeIdx === selectedNodeIdx.value && !fromDeal) ||
        nodeIdx === selectedChanceIdx.value ||
        (nodes.value[nodeIdx].type === "chance" &&
        isSelectedChanceSkipped.value)))
        ) return;

        if (nodeIdx === 0) {
            await selectNode(1, true);
            return;
        }

        emit("update:is-locked", true);

        selectedNodeIdxTemp = selectedNodeIdx.value;
        selectedChanceIdxTemp = selectedChanceIdx.value;

        if (fromDeal) {
            const findRiverIdx = nodes.value
                .slice(selectedChanceIdxTemp + 3)
                .findIndex((node) => node.type === "chance");
            
            let riverIdx = -1;
            if (findRiverIdx !== -1) {
                riverIdx = findRiverIdx + selectedChanceIdxTemp + 3;
            }
            const riverNode = riverIdx === -1 ? null : (nodes.value[riverIdx] as ChanceNode);
            selectedChanceIdxTemp = -1;

            if (riverNode) {
                
                const history = nodes.value.slice(1, riverIdx).map((node) => node.selectedIdx);
                await rust.applyHistoryGame(history);
                const possibleCards = await rust.possibleCardsGame();
                
                for (let i = 0; i < 52; ++i) {
                    const dead = !(possibleCards & (1n << BigInt(i)));
                    riverNode.cards[i].dead = dead;

                    if (riverNode.selectedIdx === i && dead) {
                        riverNode.cards[i].selected = false;
                        riverNode.selectedIdx = -1;
                    }
                }
            }

            const riverSkipped = riverNode?.selectedIdx === -1;
            const lastNode = nodes.value[nodes.value.length - 1];

            if (
                !riverSkipped &&
                lastNode.type === "terminal" &&
                lastNode.equityOOP !== 0 &&
                lastNode.equityOOP !== 1
            ) {
                
                const history = nodes.value.slice(1, -1).map((node) => node.selectedIdx);
                await rust.applyHistoryGame(history);
                const results = await rust.resultsGame();
                
                if (!results.empty) {
                    lastNode.equityOOP = average(results.equity[0], results.normaliser[0]);
                } else {
                    lastNode.equityOOP = -1;
                }
            }
        }

        if (!splice && nodes.value[nodeIdx].type === "chance") {
            selectedChanceIdxTemp = nodeIdx;
            if (selectedNodeIdxTemp < nodeIdx + 1) {
                selectedNodeIdxTemp = nodeIdx + 1;
            }
        
        } else {
            selectedNodeIdxTemp = nodeIdx;
            if (nodeIdx <= selectedChanceIdxTemp) {
                selectedChanceIdxTemp = -1;
            } else if (selectedChanceIdxTemp === -1) {
                selectedChanceIdxTemp = nodes.value
                    .slice(0, nodeIdx)
                    .findIndex((node) => node.type === "chance" && node.selectedIdx === -1);
            }
        }

        let endIdx: number;
        if (selectedChanceIdxTemp === -1) {
            endIdx = selectedNodeIdxTemp;
        } else {
            endIdx = selectedChanceIdxTemp;
        }

        const history = nodes.value.slice(1, endIdx).map((node) => node.selectedIdx);
        await rust.applyHistoryGame(history);
        
        results = await rust.resultsGame();

        let append: number[] = [];
        if (selectedChanceIdxTemp !== -1) {
            append = nodes.value
                .slice(selectedChanceIdxTemp, selectedNodeIdxTemp)
                .map((node) => node.selectedIdx);
        }

        const nxtActions = await rust.actionsAfterGame(append);
        canChanceReport.value = 
            selectedChanceIdxTemp !== -1 &&
            nodes.value.slice(selectedChanceIdxTemp + 3, selectedNodeIdxTemp)
                .every((node) => node.type !== "chance") &&
            nxtActions[0] !== "chance";
        
        if (canChanceReport.value) {
            let player: "oop" | "ip" | "terminal";
            let numActions: number;
            
            if (nxtActions[0] === "terminal") {
                player = "terminal";
                numActions = 0;
            } else {
                player = append.length % 2 === 1 ? "oop" : "ip";
                numActions = nxtActions.length;
            }

            chanceReport = await rust.chanceReportGame(append, player, numActions);
        } else {
            chanceReport = null;
        }

        totalBetAmount = await rust.totalBetAmountGame([]);
        totalBetAmountAppended = await rust.totalBetAmountGame(append);

        if (splice) {
            if (nxtActions[0] === "terminal") {
                spliceNodesTerminal(nodeIdx);
            } else if (nxtActions[0] === "chance") {
                await spliceNodesChance(nodeIdx);
            } else {
                spliceNodesPlayer(nodeIdx, nxtActions);
            }
        }

        // Update rates.
        const node = nodes.value[selectedNodeIdxTemp];
        if (node.type === "player" && selectedChanceIdxTemp === -1) {
            const playerIdx = node.player === "oop" ? 0 : 1;
            if (results.empty & (1 << playerIdx)) {
                rates.value = null;
            } else {
                const n = props.cards[playerIdx].length;
                rates.value = Array.from({ length: node.actions.length }, (_, i) => {
                    if (!results) throw new Error("Results are null");
                    const rates = results.strategy.slice(i * n, (i + 1) * n);
                    return average(rates, results.normaliser[playerIdx]);
                });
            }
        } else {
            rates.value = null;
        }

        selectedNodeIdx.value = selectedNodeIdxTemp;
        selectedChanceIdx.value = selectedChanceIdxTemp;
        isDealing.value = false;

        emit(
            "trigger-update",
            selectedNode.value,
            selectedChance.value,
            currentBoard.value,
            results,
            chanceReport,
            totalBetAmount,
        );
    }

    const spliceNodesTerminal = (nodeIdx: number) => {
        
        if (!results) throw new Error("Results are null");
        const prevNode = nodes.value[nodeIdx - 1] as PlayerNode;
        const prevAction = prevNode.actions[prevNode.selectedIdx];
        const chanceIdx = selectedChanceIdxTemp;
        const chanceSkipped = chanceIdx !== -1 && (nodes.value[chanceIdx] as ChanceNode).selectedIdx === -1;

        let equityOOP;
        if (prevAction.name === "Fold") {
            equityOOP = prevNode.player === "oop" ? 0 : 1;
        } else if (chanceSkipped || results.empty) {
            equityOOP = -1;
        } else {
            equityOOP = average(results.equity[0], results.normaliser[0]);
        }

        const betSum = totalBetAmountAppended[0] + totalBetAmountAppended[1];
        nodes.value.splice(nodeIdx, nodes.value.length, {
            type: "terminal",
            idx: nodeIdx,
            player: "end",
            selectedIdx: -1,
            prevPlayer: prevNode.player,
            equityOOP,
            pot: config.startingPot + betSum,
        });
    };

    const spliceNodesChance = async (nodeIdx: number) => {
        type TurnNode = RootNode | ChanceNode;
        const prevNode = nodes.value[nodeIdx - 1] as PlayerNode;
        const turnNode = nodes.value.slice(0, nodeIdx).find((node) => node.player === "turn") as TurnNode | undefined;
        let append: number[] = [];

        if (selectedChanceIdxTemp !== -1) {
            append = nodes.value
                .slice(selectedChanceIdxTemp, nodeIdx)
                .map((node) => node.selectedIdx);
        }

        let possibleCards = 0n;
        if (!(turnNode?.type === "chance" && turnNode.selectedIdx === -1)) {
            possibleCards = await rust.possibleCardsGame();
        }

        append.push(-1);
        const nxtActions = await rust.actionsAfterGame(append);

        let numBetActions = nxtActions.length;
        while (numBetActions > 0 && nxtActions[nxtActions.length - numBetActions].split(":")[1] === "0") {
            --numBetActions;
        }

        if (selectedChanceIdxTemp === -1) {
            canChanceReport.value = true;
            const numActions = nxtActions.length;
            chanceReport = await rust.chanceReportGame(append, "oop", numActions);
        }

        nodes.value.splice(nodeIdx, nodes.value.length, 
        {
            type: "chance",
            idx: nodeIdx,
            player: turnNode ? "river" : "turn",
            selectedIdx: -1,
            prevPlayer: prevNode.player,
            cards: Array.from({ length: 52 }, (_, i) => ({
                card: i,
                selected: false,
                dead: !(possibleCards & (1n << BigInt(i))),
            })),
            pot: config.startingPot + 2 * totalBetAmountAppended[0],
            stack: config.effectiveStack - totalBetAmountAppended[0],
        },
        {
            type: "player",
            idx: nodeIdx + 1,
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
                    colour: actionColour(name, i, nxtActions.length, numBetActions),
                };
            }),
        });

        ++selectedNodeIdxTemp;
        if (selectedChanceIdxTemp === -1) {
            selectedChanceIdxTemp = nodeIdx;
        }
    };

    const spliceNodesPlayer = (nodeIdx: number, actions: string[]) => {

        const prevNode = nodes.value[nodeIdx - 1];
        const player = prevNode.player === "oop" ? "ip" : "oop";
        let numBetActions = actions.length;

        if (actions[0].split(":")[1] === "0") --numBetActions;
        if (actions[1].split(":")[1] === "0") --numBetActions;

        nodes.value.splice(nodeIdx, nodes.value.length, {
            type: "player",
            idx: nodeIdx,
            player,
            selectedIdx: -1,
            actions: actions.map((action, i) => {
                const [name, amount] = action.split(":");
                return {
                    idx: i,
                    name, 
                    amount,
                    selected: false,
                    colour: actionColour(name, i, actions.length, numBetActions),
                };
            }),
        });
    };

    const play = async (nodeIdx: number, actionIdx: number) => {
        const node = nodes.value[nodeIdx] as PlayerNode;
        if (node.selectedIdx !== -1) {
            node.actions[node.selectedIdx].selected = false;
        }
        node.actions[actionIdx].selected = true;
        node.selectedIdx = actionIdx;
        await selectNode(nodeIdx + 1, true);
    };

    const deal = async (card: number) => {
        const node = selectedChance.value;
        if (!node) throw new Error("Node is null");
        isDealing.value = true;
        if (node.selectedIdx !== -1) {
            node.cards[node.selectedIdx].selected = false;
        }
        node.cards[card].selected = true;
        node.selectedIdx = card;
        await selectNode(selectedNodeIdx.value, false, true);
    };

    const dealArrow = async (node: ChanceNode, rankDir: number, suitDir: number) => {
        const offset = rankDir ? 4 * rankDir : suitDir;
        let card = node.selectedIdx + offset;

        for (; 0 <= card && card < 52; card += offset) {
            if (!node.cards[card].dead) break;
        }

        if (card < 0 || 52 <= card) {
            throw new Error("Card invalid");
        }

        isDealing.value = true;
        node.cards[node.selectedIdx].selected = false;
        node.cards[card].selected = true;
        node.selectedIdx = card;

        const selectedChanceIdxBak = selectedChanceIdx.value;
        selectedChanceIdx.value = node.idx;

        await selectNode(selectedNodeIdx.value, false, true);
        if (selectedChanceIdx.value === -1 && selectedChanceIdxBak !== node.idx) {
            selectedChanceIdx.value = selectedChanceIdxBak;
        }
    };

    const isCardAvailable = (node: ChanceNode, rankDir: number, suitDir: number) => {
        let card = node.selectedIdx;
        if (rankDir) {
            card += 4 * rankDir;
            for(; 0 <= card && card < 52; card += 4 * rankDir) {
                if (!node.cards[card].dead) return true;
            }
        } else {
            const rank = card >>> 2;
            card += suitDir;
            for (; card >>> 2 === rank; card += suitDir) {
                if (!node.cards[card].dead) return true;
            }
        }

        return false;
    };

    watch(dealtCard, async (card) => {
        if (card === -1) return;
        await deal(card);
    });

    const nodeCards = (node: RootNode | ChanceNode) => {
        if (node.type === "root") {
            return node.board.map((card) => idxToCard(card));
        } else if (node.selectedIdx === -1) {
            return [{ rank: "?", suit: "", colorClass: "text-black"}]
        } else {
            return [idxToCard(node.selectedIdx)];
        }
    };

    const foldColour = { r: 0x3b, g: 0x82, b: 0xf6 }; // blue-500
    const checkColour = { r: 0x22, g: 0xc5, b: 0x5e }; // green-500
    const callColour = { r: 0x22, g: 0xc5, b: 0x5e }; // green-500
    const betColourGradient = [
        { r: 0xf5, g: 0x9e, b: 0x0b }, // amber-500
        { r: 0xf9, g: 0x73, b: 0x16 }, // orange-500
        { r: 0xef, g: 0x44, b: 0x44 }, // red-500
        { r: 0xec, g: 0x48, b: 0x99 }, // pink-500
        { r: 0xd9, g: 0x46, b: 0xef }, // fuchsia-500
        { r: 0xa8, g: 0x55, b: 0xf7 }, // purple-500
        { r: 0x8b, g: 0x5c, b: 0xf6 }, // violet-500
    ];

    const actionColour = (
        name: string, 
        idx: number, 
        numActions: number,
        numBetActions: number,    
    ) => {
        
        if (name === "Fold") return rgbToString(foldColour);
        if (name === "Check") return rgbToString(checkColour);
        if (name === "Call") return rgbToString(callColour);

        if (numBetActions === 1) return rgbToString(betColourGradient[0]);
        if (idx === numActions - 1) {
            const denominator = numBetActions === 2 ? 2 : 1;
            return rgbToString(betColourGradient[(betColourGradient.length - 1) / denominator]);
        }

        const betIdx = idx - (numActions - numBetActions);
        const colourRate = betIdx / (numBetActions - 1);

        const gradientRate = colourRate * (betColourGradient.length - 1);
        const gradientIdx = Math.floor(gradientRate);
        const r = gradientRate - gradientIdx;

        const colour1 = betColourGradient[gradientIdx];
        const colour2 = betColourGradient[gradientIdx + 1];

        const outColour = {r: 0, g: 0, b: 0};
        for (const primary of ["r", "g", "b"] as const) {
            const primary1 = colour1[primary];
            const primary2 = colour2[primary];
            outColour[primary] = Math.floor(primary1 * (1 - r) + primary2 * r);
        }

        return rgbToString(outColour);
    };

</script>

<template>
    <div ref="navDiv" class="flex shrink-0 h-44 gap-1 p-1 overflow-x-auto whitespace-nowrap">
        <div
            v-for="node in nodes"
            :key="node.idx"
            class="flex flex-col relative h-full p-1 justify-start rounded-lg shadow-md border-2 "
            @click="selectNode(node.idx, false)"    
        >
            <!-- Root or Chance -->
            <template v-if="node.type === 'root' || node.type === 'chance'">
                <div class="p-1 font-semibold">
                    {{ node.player.toUpperCase() }}
                </div>
                <div class="flex flex-col flex-grow px-3 items-center justify-evenly font-semibold">
                    <div class="relative">
                        <span 
                            v-for="card of nodeCards(node)"
                            :key="card.toString()"
                        >
                            {{ card.toString() }}
                        </span>
                        <template v-if="node.type === 'chance' && node.selectedIdx !== -1">

                            <button 
                                class="absolute -top-[1.375rem] left-1/2 -ml-3 w-6 h-6"
                                @click.stop="isCardAvailable(node, 0, 1) && dealArrow(node, 0, 1)"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 15.75l7.5-7.5 7.5 7.5" />
                                </svg>
                            </button>

                            <button
                                class="absolute -left-[1.375rem] top-1/2 -mt-3 w-6 h-6"
                                @click.stop="isCardAvailable(node, 1, 0) && dealArrow(node, 1, 0)"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                                </svg>
                            </button>

                            <button
                                class="absolute -right-[1.375rem] top-1/2 -mt-3 w-6 h-6"
                                @click.stop="isCardAvailable(node, -1, 0) && dealArrow(node, -1, 0)"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                                </svg>
                            </button>

                            <button
                                class="absolute -bottom-[1.375rem] left-1/2 -ml-3 w-6 h-6"
                                @click.stop="isCardAvailable(node, 0, -1) && dealArrow(node, 0, -1)"    
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
                                </svg>
                            </button>
                        </template>
                    </div>

                    <div>
                        <div>Pot {{ node.pot }}</div>
                        <div>Stack {{ node.stack }}</div>
                    </div>
                </div>

                <button
                    v-if="
                        node.idx === selectedChanceIdx &&
                        node.selectedIdx !== -1 && 
                        !isDealing
                    "
                    class="absolute top-1.5 right-1.5 "
                    @click="deal(node.selectedIdx)"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </template>

            <!-- Player -->
            <template v-if="node.type === 'player'">
                <div class="p-1 font-semibold">
                    {{ node.player.toUpperCase() }}
                </div>
                <div class="flex-grow overflow-y-auto">
                    <button
                        v-for="action of node.actions"
                        :key="action.idx"
                        class="flex w-full px-1.5 rounded-md"
                        @click.stop="play(node.idx, action.idx)"    
                    >
                        <span class="inline-block relative w-4 mr-0.5">
                            <span 
                                v-if="node.idx === selectedNodeIdx && !(selectedChanceIdx !== -1 && !canChanceReport)"
                                class="absolute top-[0.3125rem] left-0 w-3 h-3 rounded-sm"
                            ></span>
                            <span v-if="action.selected">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                                </svg>
                            </span>
                        </span>
                        <span class="pr-0.5 font-semibold">
                            {{ action.name }}
                            {{ action.amount === "0" ? "" : action.amount }}
                        </span>
                        <span 
                            v-if="node.idx === selectedNodeIdx && rates != null"
                            class="ml-auto pl-1.5"
                        >
                            [{{ (rates[action.idx] * 100).toFixed(1) }}%]
                        </span>
                    </button>
                </div>
            </template>

            <!-- Terminal -->
            <template v-if="node.type === 'terminal'">
                <div class="p-1 font-semibold">
                    {{ node.player.toUpperCase() }}
                </div>
                <div class="flex flex-col flex-grow items-center justify-evenly font-semibold">
                    <div v-if="node.equityOOP === 0 || node.equityOOP === 1" class="px-3">
                        {{ ["IP", "OOP"][node.equityOOP] }} Wins
                    </div>
                    <div v-else-if="node.equityOOP !== -1" class="px-1.5">
                        <div class="mb-0.5">Equity</div>
                        <div class="flex w-full px-1.5">
                            <span>OOP</span>
                            <span class="ml-auto pl-2">
                                {{ (node.equityOOP * 100).toFixed(1) }}%
                            </span>
                        </div>
                        <div class="flex w-full px-1.5">
                            <span>IP</span>
                            <span class="ml-auto pl-2">
                                {{ ((1 - node.equityOOP) * 100).toFixed(1) }}%
                            </span>
                        </div>
                    </div>
                    <div class="px-3">Pot {{  }}</div>
                </div>
            </template>           
        </div>
    </div>
</template>