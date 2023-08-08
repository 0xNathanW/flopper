
<script setup lang="ts">
    import { useConfigStore } from '../store';
    import { cardText } from '../util';
    import Card from './Card.vue';

    const config = useConfigStore();

    const boardText = () => {
        return config.board.map((n) => {
            const card = cardText(n);
            return card.rank + card.suit;
        }).join(' ');
    };

</script>

<template>
    <div class="flex flex-col h-full p-3">
        <div class="flex justify-between items-center px-1 mb-2">
            <h1 class="lg:text-2xl xl:text-3xl font-bold">Board: {{ boardText() }}</h1>
            <div class="flex gap-2">
                <button class="btn btn-primary sm:btn-xs xl:btn-sm" @click="config.setRandomBoard(3)">Random</button>
                <button class="btn btn-primary sm:btn-xs xl:btn-sm" @click="config.clearBoard">Clear</button>
            </div>    
        </div>
        <div v-for="suit in 4" :key="suit" class="flex flex-row grow">
            <Card
                v-for="rank in 13"
                :key="rank"
                class="lg:m-0.5 xl:m-1"
                :id="56 - 4 * rank - suit"
                :selected="config.board.includes(56 - 4 * rank - suit)"
                @click="config.addToBoard(56 - 4 * rank - suit)"
            />
        </div>
    </div>
</template>