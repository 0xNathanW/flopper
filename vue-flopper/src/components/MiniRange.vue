
<script setup lang="ts">
    import { useConfigStore } from '../store';
    
    const config = useConfigStore();
    const props = defineProps<{oop: boolean}>();

    const getWeight = (i: number, j: number): number => {
        return props.oop ? config.oopRange[(i - 1) * 13 + (j - 1)] : config.ipRange[(i - 1) * 13 + (j - 1)];
    }

</script>

<template>
    <div class="flex flex-col items-start">
            
            <div class="flex flex-row items-center justify-between w-full">
                <h2>{{oop ? "OOP" : "IP"}} Range</h2>
                <span 
                    v-if="props.oop ? config.rangeEmptyOOP : config.rangeEmptyIP " 
                    class="badge badge-warning badge-md">
                    Empty
                </span>
            </div>
            
            <table class="table-fixed">
                <tbody>
                    <tr v-for="row in 13" :key="row" class="h-4">
                        <td v-for="col in 13" :key="col" class="relative w-4 px-0 py-0 border-[0.5px] border-accent">
                            <div class="absolute w-full h-full left-0 top-0 bg-base-200">
                                <div 
                                    class="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
                                    :style="{
                                        'background-image': 'linear-gradient(hsl(var(--pf)) 0% 100%)',
                                        'background-size': `100% ${getWeight(row, col)}%`,
                                    }"
                                ></div>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        
        </div>
</template>