
<script setup lang="ts">

    import { reactive, ref, toRefs, watch } from "vue";
    import { ResultsPanel, ResultsOpts } from "../typing";

    const props = defineProps<{
        resultsPanel: string;
        chanceMode: string;
        autoPlayerBasics: "oop" | "ip";
        autoPlayerChance: "oop" | "ip";
    }>();

    const emit = defineEmits<{
        (event: "update:resultsPanel", resultsPanel: ResultsPanel): void;
        (event: "update:resultsOpts", resultsOpts: ResultsOpts): void;
    }>();

    const { chanceMode } = toRefs(props);
    let oldResultsPanel = "basics" as ResultsPanel;

    watch(chanceMode, (newVal, oldVal) => {
        if (newVal && !oldVal) {
            oldResultsPanel = props.resultsPanel;
            emit("update:resultsPanel", "chance");
        } else if (!newVal && oldVal) {
            emit("update:resultsPanel", oldResultsPanel);
        }
    });

    const resultsOpts = reactive<ResultsOpts>({
        playerBasics: "auto",
        playerChance: "auto",
        barHeight: "normalised",
        suit: "grouped",
        strategy: "show",
        contentBasics: "default",
        contentGraphs: "eq",
        chartChance: "strategy-combos",
    });

    const strategyContentPair = ref("show,default");

    const updateResultsPanel = (resultsPanel: ResultsPanel) => {
        if (resultsPanel !== "chance") {
            oldResultsPanel = resultsPanel;
        }
        emit("update:resultsPanel", resultsPanel);
    };

    const updateResultsOpts = () => {
        const opts = resultsOpts;
        const [strat, content] = strategyContentPair.value.split(",");
        opts.strategy = strat as ResultsOpts["strategy"];
        opts.contentBasics = content as ResultsOpts["contentBasics"];
        emit("update:resultsOpts", opts);
    };

</script>

<template>
    <div class="flex shrink-0 h-12 border-y">

        <button
            v-for="m in (['basics', 'graphs', 'compare'] as const)"
            :key="m"
            class="flex w-[10%] h-full items-center justify-center font-semibold text-lg"
            @click="updateResultsPanel(m)"
        >
            {{ m }}
        </button>

        <button
            class="flex w-[10%] h-full items-center justify-center font-semibold text-lg"
            :disabled="chanceMode === ''"    
            @click="updateResultsPanel('chance')"
        >
            chanceMode
        </button>

        <div class="flex ml-auto shrink-0 h-full px-4 items-center justify-start gap-2 snug">

            <div 
                v-if="['basics', 'graphs'].includes(resultsPanel)" 
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Player: </div>
                <select
                    v-model="resultsOpts.playerBasics"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="auto">
                        Auto {{ autoPlayerBasics.toUpperCase() }}
                    </option>
                    <option value="oop">OOP</option>
                    <option value="ip">IP</option>
                </select>
            </div>

            <div
                v-if="resultsPanel === 'chance'"
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Player: </div>
                <select
                    v-model="resultsOpts.playerChance"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="auto">
                        Auto {{ autoPlayerChance.toUpperCase() }}
                    </option>
                    <option value="oop">OOP</option>
                    <option value="ip">IP</option>
                </select>
            </div>

            <div
                v-if="['basics', 'compare'].includes(resultsPanel)"
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Bar height: </div>
                <select
                    v-model="resultsOpts.barHeight"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="normalised">Normalised</option>
                    <option value="absolute">Absolute</option>
                    <option value="full">Full</option>
                </select>
            </div>

            <div
                v-if="['basics', 'compare'].includes(resultsPanel)"
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Suit: </div>
                <select
                    v-model="resultsOpts.suit"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="grouped">Grouped</option>
                    <option value="separate">Separate</option>
                </select>
            </div>

            <div
                v-if="['basics', 'compare'].includes(resultsPanel)"
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Display: </div>
                <select
                    v-model="strategyContentPair"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="show,default">Strategy</option>
                    <option value="show,eq">Strategy + EQ</option>
                    <option value="show,ev">Strategy + EV</option>
                    <option value="show,eqr">Strategy + EQR</option>
                    <option value="none,default">Weight</option>
                    <option value="none,eq">EQ</option>
                    <option value="none,ev">EV</option>
                    <option value="none,eqr">EQR</option>
                </select>
            </div>

            <div
                v-if="resultsPanel === 'graphs'"
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Display: </div>
                <select
                    v-model="resultsOpts.contentGraphs"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="eq">EQ</option>
                    <option value="ev">EV</option>
                    <option value="eqr">EQR</option>
                </select>
            </div>

            <div
                v-if="resultsPanel === 'chance'"
                class="flex flex-col items-start justify-center h-full"
            >
                <div class="text-sm">Chart: </div>
                <select
                    v-model="resultsOpts.chartChance"
                    class="w-28 px-1 py-0.5 rounded-lg cursor-pointer"
                    @chance="updateResultsOpts"
                >
                    <option value="strategy-combos">Strategy combos</option>
                    <option value="strategy">Strategy</option>
                    <option value="eq">Equity</option>
                    <option value="ev">EV</option>
                    <option value="eqr">EQR</option>
                </select>
            </div>
        </div>
    </div>
</template>