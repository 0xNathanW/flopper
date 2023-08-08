
<script setup lang="ts">
    import { cardText } from '../util';

    interface Props {
        id: number;
        selected?: boolean;
        fontSize?: string;
    };

    const props = withDefaults(defineProps<Props>(), {
        selected: false,
        fontSize: "1rem",
    });

    const { rank, suit, colourClass } = cardText(props.id);
</script>

<template>
    <button
        class="grow relative rounded-lg select-none border-2 border-accent"
        :style="{
            '--font-size': fontSize,
            backgroundColor: selected ? 'gold' : 'white'
        }"
    >
        <span
            :class="'absolute top-0 font-bolder' + colourClass"
            :style="{
                left: '15%',          
                'font-size': 'calc(var(--font-size) * 1.25)',
                'color': 'black',
            }"    
        >
            {{ rank }}
        </span>
        <span
            :class="'absolute ' + colourClass"
            :style="{
                bottom: '5%',
                right: '10%',
                'font-size': 'var(--font-size)',
            }"
        >
            {{ suit }}
        </span>
    </button>
</template>