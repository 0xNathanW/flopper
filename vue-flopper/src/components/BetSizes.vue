
<script setup lang="ts">
    import { useConfigStore } from '../store';
    
    const props = defineProps<{player: number}>();
    const config = useConfigStore();
    const streetText = ["Flop", "Turn", "River"];
</script>

<template>
    <div class="flex flex-row items-center gap-5 xl:gap-10 2xl:gap-14 justify-evenly">
        <div v-for="street in 3" class="indicator">
            <span class="indicator-item indicator-center badge badge-accent badge-lg">{{ streetText[(street - 1)] }}</span>
            <!-- Street Bet Sizes -->
            <div class="grid grid-cols-[3rem,10rem,3rem] gap-3 p-4 pt-5 border-2 border-accent rounded-lg items-center">
                <label>Bet: </label>
                <input
                    v-model="config.betSizes[props.player - 1][street - 1][0]"
                    type="text"
                    class="input input-bordered input-primary input-sm w-full"
                />
                <!-- Empty badge -->
                <div 
                    v-if="config.streetBetValidity(street, player, false).validity === 0"
                    class="badge badge-warning badge-sm"                                
                >Empty</div>
                <!-- Valid badge -->    
                <div 
                    v-else-if="config.streetBetValidity(street, player, false).validity === 1"
                    class="badge badge-success badge-sm"
                >Valid</div>
                <!-- Invalid badge -->
                <div 
                    v-else-if="config.streetBetValidity(street, player, false).validity === 2" 
                    class="badge badge-error badge-sm"
                >Error</div>

                <label>Raise: </label>
                <input
                    v-model="config.betSizes[props.player - 1][street - 1][1]"
                    type="text"
                    class="input input-bordered input-primary input-sm w-full"
                />
                <!-- Empty badge -->
                <div 
                    v-if="config.streetBetValidity(street, player, true).validity === 0"
                    class="badge badge-warning badge-sm"                                
                >Empty</div>
                <!-- Valid badge -->    
                <div 
                    v-else-if="config.streetBetValidity(street, player, true).validity === 1"
                    class="badge badge-success badge-sm"
                >Valid</div>
                <!-- Invalid badge -->
                <div 
                    v-else-if="config.streetBetValidity(street, player, true).validity === 2" 
                    class="badge badge-error badge-sm"
                >Error</div>

            </div>
        </div>
    </div>
</template>