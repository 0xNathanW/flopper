
<script setup lang="ts">
    import { ref } from 'vue';
    import { useConfigStore } from '../store';
    import { RANKS, textToRange } from "../util";

    const config = useConfigStore();
    const props = defineProps<{oop: boolean}>();
    
    // Is the mouse down.
    type DragStatus = "none" | "enabled" | "disabled";
    let dragStatus: DragStatus = "none";

    const handleMouseDown = (i: number, j: number) => {
        config.setWeight(cellIdx(i, j), weight.value, props.oop);
        dragStatus = "enabled";
    };

    const handleMouseOver = (i: number, j: number) => {
        if (dragStatus == "enabled") {
            config.setWeight(cellIdx(i, j), weight.value, props.oop);
        }
    };

    window.addEventListener("mouseup", () => {
        dragStatus = "none";
    });

    let weight = ref(100);

    // eg. AA, AKo, AKs etc.
    const cellText = (i: number, j: number): string => {
        const rank1 = RANKS[i - 1];
        const rank2 = RANKS[j - 1];
        if (i == j) {
            return rank1 + rank2;
        } else if (i < j) {
            return rank1 + rank2 + "s";
        } else {
            return rank2 + rank1 + "o";
        }
    };

    // Index of cell in range array.
    const cellIdx = (i: number, j: number): number => {
        return (i - 1) * 13 + (j - 1);
    };

    // Get corresponding weight from range array.
    const getWeight = (i: number, j: number): number => {
        return props.oop ? config.oopRange[cellIdx(i, j)] : config.ipRange[cellIdx(i, j)];
    };

    // String of weight to display in bottom right of cell if not 0 or 100.
    const weightText = (i: number, j: number): (string | null) =>  {
        const weight = getWeight(i, j).toString();
        if (weight === "0" || weight === "100") {
            return null;
        } else {
            return weight.toString() + "%";
        }
    };

    // Load a range from a string change in future to load from a json file with all saved ranges.
    const loadRange = (s: string) => {
        const range = textToRange(s);
        if (props.oop) {
            config.oopRange = range;
        } else {
            config.ipRange = range;
        }
    };

    // Temp values, eventually moved into json file.
    const positions = ["MP2", "MP3", "CO", "BTN", "SB"];

    const openRaiseStandard: string[] = [
        "66+,AJs+,AQo+",
        "66+,AJs+,KQs,AJo+,KQo",
        "22+,A7s+,K9s+,Q9s+,J9s+,T8s+,A9o+,K9o+,QTo+,J9o+",
        "22+,A2s+,K8s+,Q8s+,J7s+,T9s,98s,87s,76s,65s,54s,A9o+,K9o+,Q9o+,J8o+,T8o+,97o+,87o,76o,65o,54o",
        "22+,ATs+,KTs+,QTs+,JTs,ATo+,KTo+,QTo+,JTo",
    ];

    const openRaiseTight: string[] = [
        "66+,AJs+,AQo+",
        "66+,AJs+,KQs,AJo+,KQo",
        "22+,A7s+,K9s+,Q9s+,J9s+,T8s+,A9o+,K9o+,QTo+,J9o+",
        "22+,A2s+,K8s+,Q8s+,J7s+,T9s,98s,87s,76s,65s,54s,A9o+,K9o+,Q9o+,J8o+,T8o+,97o+,87o,76o,65o,54o",
        "22+,ATs+,KTs+,QTs+,JTs,ATo+,KTo+,QTo+,JTo",
    ];

    const openRaiseLoose: string[] = [
        "22+,ATs+,KJs+,QJs,JTs,T9s,98s,87s,76s,65s,ATo+,KJo+",
        "22+,A9s+,KJs+,QTs+,JTs,T9s,98s,87s,76s,65s,A9o+,KJo+,QJo",
        "22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o",
        "22+,A2s+,K2s+,Q2s+,J5s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K2o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o",
        "22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o",
    ];
</script>

<template>
    <div class="flex flex-row items-start justify-center gap-5">
        <div class="flex flex-col items-start w-[625px] min-w-[625px]">
            
            <!-- range grid header -->
            <div class="flex flex-row mb-3 w-full justify-between items-center">
                <h2 class="font-bold text-3xl">{{ props.oop ? "OOP" : "IP" }} Range</h2>
                <button
                    class="btn btn-primary"
                    id="btn-clear"
                    @click="config.clearRange(props.oop)"
                >
                Clear
                </button>
            </div>

            <!-- range grid -->
            <table class="table-fixed select-none">
                <tr v-for="row in 13" :key="row" class="h-12">
                    <td 
                        v-for="col in 13" 
                        :key="col" 
                        class="relative border-2 border-accent w-12"
                        @mousedown="handleMouseDown(row, col)"
                        @mouseover="handleMouseOver(row, col)"
                        >

                        <div class="absolute top-0 left-0 w-full h-full">
                            <div
                                class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat bg-base-200"
                                :style="{
                                    'background-image': `linear-gradient(hsl(var(--pf)) 0% 100%)`,
                                    'background-size': `100% ${getWeight(row, col)}%`,
                                }"
                            ></div>
                        </div>

                        <div class="absolute top-0 left-0.5">{{ cellText(row, col) }}</div>
                        <div
                            class="absolute bottom-0 right-0.5"
                        >
                            {{ weightText(row, col)  }}
                        </div>
                    </td>
                </tr>
            </table>

            <div class="flex flex-row mt-4 w-full items-center gap-5">
                <p>Weight: </p>
                <input
                    v-model.number="weight"
                    class="range range-primary"
                    type="range"
                    min="0"
                    max="100"
                    step="5"
                />
                <input
                    v-model.number="weight"
                    class="input input-primary input-bordered input-sm"
                    type="number"
                    min="0"
                    max="100"
                    step="5"
                />
                <p>%</p>
            </div>
        </div>

        <div class="w-fit">
            <ul class="menu menu-sm bg-base-200 rounded-lg w-[300px] h-[625px] overflow-y-auto mt-16">
                <li>
                    <details>
                        <summary>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-4 h-4">
                                <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                            </svg>
                            6-Max PokerStategy.com
                        </summary>
                    
                        <ul class="ml-4 pl-2">
                            
                            <li>
                                <details>
                                    <summary>
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-4 h-4">
                                            <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                                        </svg>
                                        Open Raise - Standard
                                    </summary>
                                    <ul class="ml-4 pl-2">
                                        <li v-for="(range, idx) in openRaiseStandard" :key="idx">
                                            <a @click="loadRange(range)">{{ positions[idx] }}</a>
                                        </li>
                                    </ul>
                                </details>
                            </li>

                            <li>
                                <details>
                                    <summary>
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-4 h-4">
                                            <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                                        </svg>
                                        Open Raise - Tight
                                    </summary>
                                    <ul class="ml-4 pl-2">
                                        <li v-for="(range, idx) in openRaiseTight" :key="idx">
                                            <a @click="loadRange(range)">{{ positions[idx] }}</a>
                                        </li>
                                    </ul>
                                </details>
                            </li>

                            <li>
                                <details>
                                    <summary>
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-4 h-4">
                                            <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                                        </svg>
                                        Open Raise - Loose
                                    </summary>
                                    <ul class="ml-4 pl-2">
                                        <li v-for="(range, idx) in openRaiseLoose" :key="idx">
                                            <a @click="loadRange(range)">{{ positions[idx] }}</a>
                                        </li>
                                    </ul>
                                </details>
                            </li>
                        </ul>
                    </details>
                </li>
            </ul>
        </div>
    </div>
</template>

