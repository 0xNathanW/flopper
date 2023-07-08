
<script setup lang="ts">
    import { useConfigStore } from '../store';

    const config = useConfigStore();
    const streetText = ["Flop", "Turn", "River"];

</script>

<template>

    <div class="flex flex-col items-center">
        <div class="flex flex-col items-center w-fit-content gap-5">
            
            <div v-for="player in 2">

                <div class="flex flex-row items-center gap-3 self-start mb-10">
                    <h1 class="text-2xl self-start font-bold">OOP Bet Sizes:</h1>
                    <!-- oop has tooltip -->
                    <div v-if="player === 1" class="tooltip tooltip-right tooltip-info" data-tip={syntaxTip}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-6 h-6">
                            <path strokeLinecap="round" strokeLinejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
                        </svg>
                    </div>
                    <!-- ip has copy -->
                    <button v-else class="btn btn-sm btn-secondary" @click="config.copyBets">Copy</button>
                </div>

                <div class="flex flex-row items-center gap-5 mb-5">
                    <div v-for="street in 3">
                        <div class="indicator">
                            <span class="indicator-item indicator-center badge badge-secondary badge-lg z-0">{{ streetText[(street - 1)] }}</span>
                            <!-- street bet sizes -->
                            <div class="grid grid-cols-[3rem,10rem,3rem] gap-3 p-4 pt-5 border-2 border-secondary rounded-lg items-center">
                                <label>Bet: </label>
                                <input
                                    v-model="config.betSizes[player - 1][street - 1][0]"
                                    type="text"
                                    class="input input-bordered input-secondary input-sm w-full"
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
                                    v-model="config.betSizes[player - 1][street - 1][1]"
                                    type="text"
                                    class="input input-bordered input-secondary input-sm w-full"
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
                </div>
                <div class="divider my-0"></div>
            </div>
            
            <div class="grid grid-cols-[7rem,5rem,7rem,5rem] items-center gap-10">
                
                <p>Starting Pot:</p>
                <input
                    v-model="config.startingPot"
                    type="number"
                    min="1"
                    class="input input-bordered input-secondary input-sm w-full"
                />
                
                <p>Effective Stack:</p>
                <input
                    v-model="config.effectiveStack"
                    type="number"
                    min={1}
                    class="input input-bordered input-secondary input-sm w-full"
                />

                <p>Rake %:</p>
                <input
                    v-model="config.rake"
                    type="number"
                    min={0}
                    max={100}
                    step={0.5}
                    class="input input-bordered input-secondary input-sm w-full"
                />

                <p>Rake Cap:</p>
                <input
                    v-model="config.rakeCap"
                    type="number"
                    min={0}
                    class="input input-bordered input-secondary input-sm w-full"
                />

                <p>Add All-In %:</p>
                <input
                    v-model="config.addAllInThreshold"
                    type="number"
                    min={0}
                    step={5}
                    class="input input-bordered input-secondary input-sm w-full"
                />

                <p>Force All-In %:</p>
                <input
                    v-model="config.forceAllInThreshold"
                    type = "number"
                    min={0}
                    step={5}
                    class="input input-bordered input-secondary input-sm w-full"
                />
            </div>
        </div>
    </div>
    
</template>