
<script setup lang="ts">
    import { computed, onUnmounted, ref, toRefs, watch } from 'vue';
    import { useConfigStore } from '../store';
    import { ChanceNode, HoverContent, Results, ResultsOpts, PlayerNode } from '../typing';
    import { cardPairCellIndex, rgbToString, toFixed, toFixed1 } from '../util';
    import { ActionNode } from '../typing';

    const config = useConfigStore();

    const yellow500 = "#eab308";
    const colorGradient = [
      { r: 0xef, g: 0x44, b: 0x44 }, // red-500
      { r: 0xf9, g: 0x73, b: 0x16 }, // orange-500
      { r: 0xf5, g: 0x9e, b: 0x0b }, // amber-500
      { r: 0xea, g: 0xb3, b: 0x08 }, // yellow-500
      { r: 0x84, g: 0xcc, b: 0x16 }, // lime-500
      { r: 0x22, g: 0xc5, b: 0x5e }, // green-500
      { r: 0x10, g: 0xb9, b: 0x81 }, // emerald-500
    ];

    const props = defineProps<{
        cards: number[][];
        selectedNode: ActionNode;
        selectedChance: ChanceNode | null;
        currentBoard: number[];
        results: Results;
        totalBetAmount: number[];
        resultsOpts: ResultsOpts;
        displayPlayer: "oop" | "ip";
        isCompareMode: boolean;
    }>();

    const { selectedNode, displayPlayer } = toRefs(props);
    watch([selectedNode, displayPlayer], () => {
        clickedCellIndex.value = -1;
        emit("update-hover-content", null);
    });

    const emit = defineEmits<{
        (event: "update-hover-content", content: HoverContent | null): void;
    }>();

    const clickedCellIndex = ref(-1);
    
    const getColor = (
        value: number,
        lowest: number,
        middle: number,
        highest: number
    ) => {
        if (value <= lowest) return rgbToString(colorGradient[0]);
        if (value > middle && value >= highest) return rgbToString(colorGradient[6]);

        let colorRate;
        let gradientRate;

        if (value <= middle) {
            colorRate = (value - lowest) / (middle - lowest);
            gradientRate = colorRate * 3;
        } else {
            colorRate = (value - middle) / (highest - middle);
            gradientRate = colorRate * 3 + 3;
        }

        const gradientIndex = Math.floor(gradientRate);
        const r = gradientRate - gradientIndex;

        const color1 = colorGradient[gradientIndex];
        const color2 = colorGradient[gradientIndex + 1];

        const retColor = { r: 0, g: 0, b: 0 };
        for (const primary of ["r", "g", "b"] as const) {
            const primary1 = color1[primary];
            const primary2 = color2[primary];
            retColor[primary] = Math.floor(primary1 * (1 - r) + primary2 * r);
        }

        return rgbToString(retColor);
    };

    const onClickCell = (row: number, col: number) => {
        if (props.isCompareMode) return;
        const index = cellIndex(row, col);
        if (!hasWeight(row, col)) {
            clickedCellIndex.value = -1;
            emit("update-hover-content", null);
        } else if (clickedCellIndex.value === index) {
            clickedCellIndex.value = -1;
        } else {
            clickedCellIndex.value = -1;
            onMouseEnterCell(row, col);
            clickedCellIndex.value = index;
        }
    };

    const onMouseEnterCell = (row: number, col: number) => {
        if (props.isCompareMode || clickedCellIndex.value !== -1) return;
        if (hasWeight(row, col) && cellData.value) {
            const idx = cellIndex(row, col);
            const idxs = cellData.value[idx].flatMap((suit) => suit.idxs);
            emit("update-hover-content", {
                name: cellText(row, col),
                idxs,
            });
        } else {
            emit("update-hover-content", null);
        }
    };

    const onMouseLeaveTable = () => {
        if (props.isCompareMode || clickedCellIndex.value !== -1) return;
        emit("update-hover-content", null);
    };

    onUnmounted(() => {
        if (props.isCompareMode || clickedCellIndex.value === -1) return;
        emit("update-hover-content", null);
    });

    const numActions = computed(() => {
        return (
            (props.resultsOpts.strategy === "show" &&
            props.displayPlayer === props.selectedNode.player &&
            !props.selectedChance &&
            props.results.numActions) || 0
        );
    });

    const cellData = computed(() => {
        const results = props.results;
        const opts = props.resultsOpts;
        const player = props.displayPlayer;
        const suitsIndividual = opts.suit === "individual";

        const data = Array.from({ length: 13 * 13 }, (_, i) => {
            const row = Math.floor(i / 13);
            const col = i % 13;

            let len = 1;
            if (suitsIndividual) {
                len = row === col ? 6 : row < col ? 4 : 12;
            }

            return Array.from({ length: len }, () => ({
                idxs: [] as number[],
                weight: 0,
                normaliser: 0,
                equity: 0,
                ev: 0,
                strategy: Array.from({ length: numActions.value }, () => 0),
            }));
        });

        const playerIdx = player === "oop" ? 0 : 1;
        const cardsLength = props.cards[playerIdx].length;
        const empty = props.results.empty;

        for (let i = 0; i < cardsLength; ++i) {
            const weight = results.weights[playerIdx][i];
            const normaliser = results.normaliser[playerIdx][i];
            if (weight === 0 || normaliser === 0) continue;
            const pair = props.cards[playerIdx][i];
            const c1 = pair & 0xff;
            const c2 = pair >> 8;
            const { row, col, index } = cardPairCellIndex(c1, c2);
            const cellIdx = row * 13 + col;
            const suitIdx = suitsIndividual ? index : 0;
            const target = data[cellIdx][suitIdx];

            target.idxs.push(i);
            target.weight += weight;
            target.normaliser += normaliser;

            if (!empty) {
                target.equity += results.equity[playerIdx][i] * normaliser;
                target.ev += results.ev[playerIdx][i] * normaliser;
            }

            if (numActions.value > 0) {
                for (let j = 0; j < numActions.value; ++j) {
                    const k = j * cardsLength + i;
                    target.strategy[j] += results.strategy[k] * normaliser;
                }
            }
        }

        for (const cell of data) {
            const hasWeight = cell.some((x) => x.weight > 0);
            if (!hasWeight) cell.length = 0;
        }

        return data;
    });

    const cellDenominator = computed(() => {
        const out = Array.from({ length: 13 * 13 }, () => 0);
        for (let c1 = 0; c1 < 52; ++c1) {
            if (props.currentBoard.includes(c1)) continue;
            for (let c2 = c1 + 1; c2 < 52; ++c2) {
                if (props.currentBoard.includes(c2)) continue;
                const r1 = c1 >>> 2;
                const r2 = c2 >>> 2;
                const suited = (c1 & 3) === (c2 & 3);
                if (suited) {
                    const idx = (12 - r2) * 13 + (12 - r1);
                    ++out[idx];
                } else {
                    const idx = (12 - r1) * 13 + (12 - r2);
                    ++out[idx];
                }
            }
        }

        return out;
    });

    const cellValueText = computed(() => {
        return Array.from({ length: 13 * 13 }, (_, idx) => {
            
            const resultsOpts = props.resultsOpts;
            if (
                resultsOpts.strategy === "show" &&
                resultsOpts.contentBasics === "default"
            ) return "";

            const data = cellData.value[idx];
            if (data.length === 0) return "";

            let weightSum = 0;
            let normaliserSum = 0;
            let equitySum = 0;
            let evSum = 0;

            for (const suit of data) {
                weightSum += suit.weight;
                normaliserSum += suit.normaliser;
                equitySum += suit.equity;
                evSum += suit.ev;
            }

            let value;
            if (resultsOpts.contentBasics === "default") {
                value = weightSum / cellDenominator.value[idx];
            } else if (resultsOpts.contentBasics === "eq") {
                value = equitySum / normaliserSum;
            } else if (resultsOpts.contentBasics === "ev") {
                value = evSum / normaliserSum;
            } else {
                const playerIdx = props.displayPlayer === "oop" ? 0 : 1;
                const eqrBase = props.results.eqrBase[playerIdx];
                value = evSum / (eqrBase * equitySum);
            }

            if (resultsOpts.contentBasics !== "default" && props.results.empty) {
                return "-";
            } else if (resultsOpts.contentBasics === "ev") {
                return Math.abs(value) >= 999.95
                    ? value.toFixed(0)
                    : toFixedEv.value(value);
            } else {
                return toFixed1(value * 100);
            }
        })
    });

    const evDigits = computed(() => {
        const results = props.results;
        if (results.empty) return 3;
        const playerIndex = props.displayPlayer === "oop" ? 0 : 1;
        const maxEv = Math.max(...results.ev[playerIndex]);
        return maxEv < 9.9995 ? 3 : maxEv < 99.995 ? 2 : 1;
    });

    const toFixedEv = computed(() => {
        return toFixed[evDigits.value - 1];
    });


    const maxWeight = computed(() => {
        let out = 0;
        cellData.value.forEach((cell, i) => {
            const demoninator = cell.length > 1 ? 1 : cellDenominator.value[i];
            for (const suit of cell) {
                const weight = suit.weight / demoninator;
                if (weight > out) out = weight;
            }
        });

        return out;
    })

    const cellContent = computed(() => {
        const results = props.results;
        const opts = props.resultsOpts;
        const playerIdx = props.displayPlayer === "oop" ? 0 : 1;
        const empty = results.empty;
        const eqrBase = results.eqrBase[playerIdx];
        const barHeight = opts.barHeight;
        const suitsIndividual = opts.suit === "individual";

        let lowest = 0;
        let middle = 0;
        let highest = 0;

        if (numActions.value === 0) {
            if (opts.contentBasics === "eq") {
                lowest = 0;
                middle = 0.5;
                highest = 1;
            } else if (opts.contentBasics === "ev") {
                const amounts = props.totalBetAmount;
                const amountSum = Math.min(...amounts) + amounts[playerIdx];
                const pot = config.startingPot + amountSum;
                lowest = 0;
                middle = pot / 2;
                highest = pot;
            } else if (opts.contentBasics === "eqr") {
                lowest = 0;
                middle = 1;
                highest = 2;
            }
        }

        return cellData.value.map((cell, i) => {
            const denominator = cell.length > 1 ? 1 : cellDenominator.value[i];

            return cell.map((suit) => {
                const weight = suit.weight;
                const normaliser = suit.normaliser;
                if (weight === 0) return null;

                let height;
                if (barHeight === "normalised") {
                    height = weight / (denominator * maxWeight.value);
                } else if (barHeight === "absolute") {
                    height = weight / denominator;
                } else {
                    height = 1;
                }

                if (numActions.value === 0) {
                    let colour;
                    if (empty || opts.contentBasics === "default") {
                        colour = yellow500;
                    } else {
                        let value: number;
                        if (opts.contentBasics === "eq") {
                            value = suit.equity / normaliser;
                        } else if (opts.contentBasics === "ev") {
                            value = suit.ev / normaliser;
                        } else {
                            value = suit.ev / (eqrBase * suit.equity);
                        }
                        colour = getColor(value, lowest, middle, highest);
                    }

                    const bgImage = `linear-gradient(${colour} 0% 100%)`;
                    const bgSize = `100% ${height * 100}%`;
                    return { bgImage, bgSize };
                }

                const node = props.selectedNode as PlayerNode;
                const colours = node.actions.map((action) => action.colour)
                
                let bgImage = `linear-gradient(to ${suitsIndividual ? "top" : "right"}`;
                const bgSize = `100% ${height * 100}%`;

                let prevPosition = 0;
                for (let i = suit.strategy.length - 1; i >= 0; --i) {
                    const position = prevPosition + suit.strategy[i] / normaliser;
                    bgImage += `, ${colours[i]} ${prevPosition * 100}% ${position * 100}%`;
                    prevPosition = position;
                }

                bgImage += ")";
                return { bgImage, bgSize };
            });
        });
    });
 
    const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
    const suits = ["♣", "♦", "♥", "♠"];
    
    const cellText = (row: number, col: number) => {
        const r1 = 13 - Math.min(row, col);
        const r2 = 13 - Math.max(row, col);
        return ranks[r1] + ranks[r2] + ["s", "", "o"][Math.sign(row - col) + 1];
    };

    const cellIndex = (row: number, col: number) => {
        return (row - 1) * 13 + col - 1;
    };

    const columns = (row: number, col: number) => {
        return cellContent.value[cellIndex(row, col)];
    };
    
    const hasWeight = (row: number, col: number) => {
        return cellData.value[cellIndex(row, col)].length > 0;
    };

    let strTemp = "";
</script>

<template>
    <div class="w-full h-full">
        <table
            class="w-[500px] h-[500px] tabled-fixed select-none"
            @mouseleave="onMouseLeaveTable"
        >
            <tr v-for="row in 13" :key="row">
                <td
                    v-for="col in 13"
                    :key="col"
                    class="relative border border-neutral"
                    @click="onClickCell(row, col)"    
                    @mouseenter="onMouseEnterCell(row, col)"    
                >
                    <div class="flex absolute w-full h-full left-0 top-0 bg-base-100">
                        <div
                            v-for="(column, k) in columns(row, col)"
                            :key="k"
                            class="flex-grow h-full bg-left-bottom bg-no-repeat"
                            :style="{
                                'background-image': column?.bgImage ?? 'none',
                                'background-size': column?.bgSize ?? 'auto',
                            }"
                        ></div>
                    </div>
                    <div class="absolute -top-px left-[0.1875rem] z-10 text-shadow">
                        {{ cellText(row, col) }}
                    </div>
                    <div
                        class="absolute bottom-px right-1 z-10 text-shadow text-white overflow-hidden"
                        style="max-width: calc(100% - 0.25rem); font-size: var(--value-font-size); line-height: var(--value-line-height);"                   
                        :data-set="(strTemp = cellValueText[cellIndex(row, col)])"    
                    >
                        {{ strTemp.split(".")[0] }}
                        <span v-if="strTemp.includes('.')">.
                            <span style="font-size: 85%">
                                {{ strTemp.split(".")[1] }}
                            </span>
                        </span>
                    </div>
                </td>
            </tr>
        </table>
    </div>
</template>

