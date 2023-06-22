
<script setup lang="ts">
    import { RANKS, cardToIdx, idxToCard, suitColour } from '../util';
    import { useConfigStore } from '../store';

    const config = useConfigStore();
    const suits = ["♦", "♥", "♠", "♣"];
    const cards = suits.flatMap((suit) => RANKS.map((rank) => rank + suit));

    const backgroundColour = (card: string): string => {
        
        if (config.board.includes(cardToIdx(card))) {
            return "bg-yellow-200";
        }

        if (card[1] === "♦") {
            return "bg-blue-300";
        } else if (card[1] === "♥") {
            return "bg-red-300";
        } else if (card[1] === "♠") {
            return "bg-gray-300";
        } else {
            return "bg-green-300";
        }
    }

</script>

<template>
    <div class="flex flex-col items-center">
        <div class="flex flex-col items-center w-fit-content gap-4">
            
            <div class="flex flex-row items-center gap-4 self-start">
                <h1 class="text-2xl font-bold">Board:</h1>
                <h1>
                    <span 
                        v-for="card in config.board" 
                        :key="card"
                        :class="'text-2xl font-bold ml-2 ' + suitColour(idxToCard(card)[1])"
                    >
                        {{ idxToCard(card) }}
                    </span>
                </h1>
            </div>
            
            <div class="grid grid-cols-[repeat(13,50px)] gap-2">
                <button
                    v-for="card in cards"
                    :class="
                    'h-12 w-12 outline-none border-2 border-neutral rounded-lg text-xl select-none ' +
                    backgroundColour(card)
                    "
                    :key="card"
                    @click="config.addToBoard(cardToIdx(card))"
                >
                    {{ card }}
                </button>
            </div>
            
            <div class="flex flex-row justify-between w-full">
                <button class="btn btn-primary" @click="config.clearBoard">Clear</button>
                <div class="flex flex-row items-center gap-3">
                    <p class="text-xl">Randomise:</p>
                    <div class="btn-group">
                        <button class="btn btn-primary" @click="config.setRandomBoard(3)">Flop</button>
                        <button class="btn btn-primary" @click="config.setRandomBoard(4)">Turn</button>
                        <button class="btn btn-primary" @click="config.setRandomBoard(5)">River</button>
                    </div>
                </div>
            </div>
            
        </div>
    </div>

</template>