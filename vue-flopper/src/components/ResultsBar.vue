
<script setup lang="ts">
    import { reactive, ref, toRefs, watch } from "vue";
    import { DisplayMode, DisplayOpts } from "../typing";
    import * as Type from "../typing";

    const props = defineProps<{
        displayMode: DisplayMode;
        chanceMode: string;
        autoPlayerBasics: "oop" | "ip";
        autoPlayerChance: "oop" | "ip";
    }>();

    const emit = defineEmits<{
        (event: "update:display-mode", displayMode: DisplayMode): void;
        (event: "update:display-opts", displayOpts: DisplayOpts): void;
    }>();

    const { chanceMode } = toRefs(props);
    let oldDisplayMode = "basics" as DisplayMode;

    watch(chanceMode, (newVal, oldVal) => {
        if (newVal && !oldVal) {
            oldDisplayMode = props.displayMode;
            emit("update:display-mode", "chance");
        } else if (!newVal && oldVal) {
            emit("update:display-mode", oldDisplayMode);
        }
    });

    const displayOpts = reactive<DisplayOpts>({
        playerBasics: "auto",
        playerChance: "auto",
        barHeight: "normalised",
        strategy: "show",
        contentBasics: "default",
        contentGraphs: "eq",
        chartChance: "strategy-combos",
    });

    const strategyContentPair = ref("show,default");
    const savedDisplayOpts = localStorage.getItem("display-opts");

    if (savedDisplayOpts) {
        const saved: DisplayOpts = JSON.parse(savedDisplayOpts);

        if (Type.barHeightList.includes(saved?.barHeight)) {
            displayOpts.barHeight = saved.barHeight;
        }
        if (Type.strategyList.includes(saved?.strategy)) {
            displayOpts.strategy = saved.strategy;
        }
        if (Type.contentBasicsList.includes(saved?.contentBasics)) {
            displayOpts.contentBasics = saved.contentBasics;
        }
        if (Type.contentGraphsList.includes(saved?.contentGraphs)) {
            displayOpts.contentGraphs = saved.contentGraphs;
        }
        if (Type.chartChanceList.includes(saved?.chartChance)) {
            displayOpts.chartChance = saved.chartChance;
        }

        strategyContentPair.value = [
            displayOpts.strategy,
            displayOpts.contentBasics,
        ].join(",");
        
        emit("update:display-opts", displayOpts);
    }

    const updateDisplayMode = (displayMode: DisplayMode) => {
        if (displayMode !== "chance") {
            oldDisplayMode = displayMode;
        }
        emit("update:display-mode", displayMode);
    };

    const updateDisplayOpts = () => {
        const opts = displayOpts;
        const [strat, content] = strategyContentPair.value.split(",");
        opts.strategy = strat as DisplayOpts["strategy"];
        opts.contentBasics = content as DisplayOpts["contentBasics"];
        localStorage.setItem("display-opts", JSON.stringify(opts));
        emit("update:display-opts", opts);
    };

</script>

<template>
    <div class="flex shrink-0 py-2.5 px-2 items-center">

        <div class="join">
            <button
                v-for="m in ['basics', 'graphs', 'compare'] as const"
                :key="m"
                :class="'btn join-item ' + (displayMode === m ? 'btn-primary' : '')" 
                @click="updateDisplayMode(m)"
            >
                {{ m }}
            </button>

            <button
                :class="'btn join-item ' + (displayMode === 'chance' ? 'btn-primary' : '')" 
                v-show="chanceMode !== ''"
                @click="updateDisplayMode('chance')"
            >
                {{ chanceMode }}
            </button>
        </div>

        <div class="flex ml-auto shrink-0 h-full px-4 items-center justify-start gap-2 snug">

            <div v-if="['basics', 'graphs'].includes(displayMode)" class="form-control">
                <label class="label p-0">
                    <span class="label-text">Player:</span>
                </label>
                <select 
                    class="select select-sm select-bordered select-primary"
                    v-model="displayOpts.playerBasics"
                    @change="updateDisplayOpts"    
                >
                    <option value="auto">
                        Auto {{ autoPlayerBasics.toUpperCase() }}
                    </option>
                    <option value="oop">OOP</option>
                    <option value="ip">IP</option>
                </select>
            </div>

            <div v-if="displayMode === 'chance'" class="form-control">
                <label class="label p-0">
                    <span class="label-text">Player:</span>
                </label>
                <select
                    class="select select-sm select-bordered select-primary"
                    v-model="displayOpts.playerChance"
                    @change="updateDisplayOpts"
                >
                    <option value="auto">
                        Auto {{ autoPlayerChance.toUpperCase() }}
                    </option>
                    <option value="oop">OOP</option>
                    <option value="ip">IP</option>
                </select>
            </div>

            <div
                v-if="['basics', 'compare'].includes(displayMode)"
                class="form-control"
            >
                <label class="label p-0">
                    <span class="label-text">Bar Height:</span>
                </label>
                <select
                    class="select select-sm select-bordered select-primary"
                    v-model="displayOpts.barHeight"
                    @change="updateDisplayOpts"
                >
                    <option value="normalised">Normalised</option>
                    <option value="absolute">Absolute</option>
                    <option value="full">Full</option>
                </select>
            </div>

            <div
                v-if="['basics', 'compare'].includes(displayMode)"
                class="form-control"
            >
                <div class="label p-0 text-sm">
                    <span>Display:</span>
                </div>
                <select
                    class="select select-sm select-bordered select-primary"
                    v-model="strategyContentPair"
                    @change="updateDisplayOpts"
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
                v-if="displayMode === 'graphs'"
                class="form-control"
            >
                <div class="label p-0">
                    <span class="text-sm">Display:</span>
                </div>
                <select
                    class="select select-sm select-bordered select-primary"
                    v-model="displayOpts.contentGraphs"
                    @change="updateDisplayOpts"
                >
                    <option value="eq">EQ</option>
                    <option value="ev">EV</option>
                    <option value="eqr">EQR</option>
                </select>
            </div>

            <div
                v-if="displayMode === 'chance'"
                class="form-control"
            >
                <div class="label p-0">
                    <span>Chart:</span>
                </div>
                <select
                    class="select select-sm select-bordered select-primary"
                    v-model="displayOpts.chartChance"
                    @change="updateDisplayOpts"
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