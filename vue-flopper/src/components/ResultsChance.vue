
<script setup lang="ts">
    import { ActionNode, ChanceNode, ChanceReport, DisplayOpts } from '../typing';
    import Card from './Card.vue';
    import ResultsTable from './ResultsTable.vue';

    const props = defineProps<{
        selectedNode: ActionNode;
        selectedChance: ChanceNode;
        chanceReport: ChanceReport | null;
        displayOpts: DisplayOpts;
        displayPlayer: "oop" | "ip";
    }>();

    const emit = defineEmits<{
        (emit: "deal-card", card: number): void;
    }>();

    const deal = (card: number) => {
        emit("deal-card", card);
    };

</script>

<template>
    <!-- 2 equal sized  -->
    <div class="grid grid-cols-2 w-full h-full">
        <div class="flex flex-col h-[75%] items-center just p-10">

            <div v-for="suit in 4" :key="suit" class="flex flex-row grow w-full">
                <Card v-for="rank in 13"
                    class="disabled:opacity-75 disabled:brightness-75 m-1"
                    :key="rank"
                    font-size="max(1.2vw, 13px)"
                    :id="56 - 4 * rank - suit"
                    :disabled="selectedChance.cards[56 - 4 * rank - suit].dead"
                    :selected="selectedChance.selectedIdx === 56 - 4 * rank - suit"
                    @click="deal(56 - 4 * rank - suit)"
                />
            </div>
        </div>
        
        <ResultsTable
            style="flex: 3"
            table-mode="chance"
            :chance-type="selectedChance.player"
            :selected-node="selectedNode"
            :chance-report="chanceReport"
            :display-player="displayPlayer"
        />
    </div>
</template>