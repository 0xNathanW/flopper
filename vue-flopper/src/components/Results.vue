
<script setup lang="ts">
    import { ref } from '@vue/runtime-dom';
    import { useStore } from '../store';
    import { ActionNode, ChanceNode, Results, ChanceReport, DisplayMode, DisplayOpts, PlayerNode, HoverContent } from '../typing';
    import * as rust from '../rust_funcs';
    import ResultsNavigator from './ResultsNavigator.vue';
    import ResultsBar from './ResultsBar.vue';
    import { computed } from 'vue';
    import ResultsBasics from './ResultsBasics.vue';
    import ResultsTable from './ResultsTable.vue';
    import ResultsChance from './ResultsChance.vue';
    import ResultsGraphs from './ResultsGraphs.vue';
import ResultsCompare from './ResultsCompare.vue';

    const app = useStore();
    const solverFinished = ref(false);
    app.$subscribe(async (_, store) => {
        if (solverFinished.value !== store.solverFinished) {
            if ((solverFinished.value = store.solverFinished)) {
                await init();
            } else {
                clear();
            }
        }
    });

    const handlerUpdated = ref(false);
    const locked = ref(false);
    const cards = ref<number[][]>([[], []]);
    const dealtCard = ref(-1);
    const selectedNode = ref<ActionNode | null>(null);
    const selectedChance = ref<ChanceNode | null>(null);
    const currentBoard = ref<number[]>([]);
    const results = ref<Results | null>(null);
    const chanceReports = ref<ChanceReport | null>(null);
    const totalBetAmount = ref([0, 0]);

    const init = async () => {
        cards.value = await rust.handsGame();
        handlerUpdated.value = true;
    };

    const clear = () => {
        cards.value = [[], []];
        selectedNode.value = null;
        selectedChance.value = null;
        results.value = null;
        chanceReports.value = null;
    };

    const updateNode = (
        newNode: ActionNode | null,
        newChance: ChanceNode | null,
        newBoard: number[],
        newResults: Results,
        newChanceReports: ChanceReport | null,
        newTotalBetAmount: number[]
    ) => {
        dealtCard.value = -1;
        selectedNode.value = newNode;
        selectedChance.value = newChance;
        currentBoard.value = newBoard;
        results.value = newResults;
        chanceReports.value = newChanceReports;
        totalBetAmount.value = newTotalBetAmount;
        locked.value = false;
        chanceMode.value = newChance?.player ?? "";
    };

    // Bar

    const displayMode = ref<DisplayMode>("basics");
    const chanceMode = ref("");
    const displayOpts = ref<DisplayOpts>({
        playerBasics: "auto",
        playerChance: "auto",
        barHeight: "normalised",
        strategy: "show",
        contentBasics: "default",
        contentGraphs: "eq",
        chartChance: "strategy-combos",
    });

    const copySuccess = ref(0);

    const updateDisplayMode = (newPanel: DisplayMode) => {
        displayMode.value = newPanel;
    };

    const updateDisplayOpts = (newOpts: DisplayOpts) => {
        displayOpts.value = newOpts;
    };

    const autoPlayerBasics = computed(() => {
        const node = selectedNode.value;
        const chance = selectedChance.value;
        if (!node) return "oop";

        if (chance) {
            return chance.prevPlayer;
        } else if (node.type === "terminal") {
            return node.prevPlayer;
        } else {
            return (node as PlayerNode).player;
        }
    });

    const autoPlayerChance = computed(() => {
        const node = selectedNode.value;
        if (!node) return "oop";
        if (node.type === "terminal") {
            return node.prevPlayer;
        } else {
            return (node as PlayerNode).player;
        }
    });

    const displayPlayerBasics = computed(() => {
        const optionPlayer = displayOpts.value.playerBasics;
        if (optionPlayer === "auto") {
            return autoPlayerChance.value;
        } else {
            return optionPlayer;
        }
    });

    const basicsHoverContent = ref<HoverContent | null>(null);

    const onUpdateHoverContent = (content: HoverContent | null) => {
        basicsHoverContent.value = content;
    };

    const onDealCard = (card: number) => {
        dealtCard.value = card;
    };
</script>

<template>
    <div class="flex flex-col h-[calc(100vh-5rem)] -mb-20">

        <ResultsNavigator
            :handler-updated="handlerUpdated"
            :locked="locked"
            :cards="cards"
            :dealt-card="dealtCard"
            @update:is-handler-updated="(value: boolean) => (handlerUpdated = value)"
            @update:locked="(value: boolean) => (locked = value)"
            @trigger-update="updateNode"
        />

        <ResultsBar
            :display-mode="displayMode"
            :chance-mode="chanceMode"
            :auto-player-basics="autoPlayerBasics"
            :auto-player-chance="autoPlayerChance"
            @update:display-mode="updateDisplayMode"
            @update:display-opts="updateDisplayOpts"
        />

        <div v-if="selectedNode && results" class="flex flex-grow min-h-0">
        
            <template v-if="displayMode === 'basics'">
                <ResultsBasics
                    style="flex: 4"
                    :cards="cards"
                    :selected-node="selectedNode"
                    :selected-chance="selectedChance"
                    :current-board="currentBoard"
                    :total-bet-amount="totalBetAmount"
                    :results="results"
                    :display-opts="displayOpts"
                    :display-player="displayPlayerBasics"
                    :is-compare-mode="false"
                    @update-hover-content="onUpdateHoverContent"    
                />

                <ResultsTable
                    style="flex: 3"
                    table-mode="basics"
                    :cards="cards"
                    :selected-node="selectedNode"
                    :results="results"
                    :display-player="displayPlayerBasics"
                    :hover-content="basicsHoverContent"
                />
            </template>

            <template v-else-if="displayMode === 'graphs'">
                <ResultsGraphs
                    :cards="cards"
                    :selected-node="selectedNode"
                    :selected-chance="selectedChance"
                    :results="results"
                    :chance-reports="chanceReports"
                    :display-opts="displayOpts"
                    :display-player="displayPlayerBasics"
                />
            </template>
            
            <template v-else-if="displayMode === 'compare'">
                <ResultsBasics
                    style="flex: 5"
                    :cards="cards"
                    :selected-node="selectedNode"
                    :selected-chance="selectedChance"
                    :current-board="currentBoard"
                    :total-bet-amount="totalBetAmount"
                    :results="results"
                    :display-opts="displayOpts"
                    display-player="oop"
                    :is-compare-mode="true"
                />
                <ResultsCompare
                    style="flex: 2"
                    :selected-node="selectedNode"
                    :selected-chance="selectedChance"
                    :results="results"
                />
                <ResultsBasics
                    style="flex: 5"
                    :cards="cards"
                    :selected-node="selectedNode"
                    :selected-chance="selectedChance"
                    :current-board="currentBoard"
                    :total-bet-amount="totalBetAmount"
                    :results="results"
                    :display-opts="displayOpts"
                    display-player="ip"
                    :is-compare-mode="true"
                />                
            </template>

            <template v-else-if="displayMode === 'chance' && selectedChance">
                <ResultsChance
                    :selected-node="selectedNode"
                    :selected-chance="selectedChance"
                    :chance-report="chanceReports"
                    :display-opts="displayOpts"
                    :display-player="displayPlayerBasics"
                    @deal-card="onDealCard"
                />
            </template>
        </div>
    </div>
</template>