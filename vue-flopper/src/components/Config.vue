
<script setup lang="ts">
    import { useStore, useConfigStore } from '../store';
    import MiniRange from './MiniRange.vue';
    import Range from './Range.vue';
    import Board from './Board.vue';
    import PreviewTree from './PreviewTree.vue';
    import BetSizes from './BetSizes.vue';
    import { computed, ref } from 'vue';
    import * as rust from '../rust_funcs';

    const app = useStore();
    const config = useConfigStore();

    type BuildState = 'idle' | 'building' | 'built';
    const buildState = ref<BuildState>('idle');
    const gameTreeBuilt = ref(false);
    const status = ref(["Not Built", "warning"]);

    const numThreads = ref(navigator.hardwareConcurrency || 1);
    const targetDelta = ref(0.5);
    const maxIters = ref(1000);
    
    const memUncompressed = ref(0);
    const memCompressed = ref(0);
    const compression = ref(false);
    
    const memUsageCompressedString = computed(() => {
        return memCompressed.value >= 1023.5 * 1024 * 1024 
        ? (memCompressed.value / (1024 * 1024 * 1024)).toFixed(2) + " GB" 
        : (memCompressed.value / (1024 * 1024)).toFixed(0) + " MB";
    });

    const memUsageUncompressedString = computed(() => {
        return memUncompressed.value >= 1023.5 * 1024 * 1024 
        ? (memUncompressed.value / (1024 * 1024 * 1024)).toFixed(2) + " GB" 
        : (memUncompressed.value / (1024 * 1024)).toFixed(0) + " MB";
    });

    const osName = ref<Awaited<ReturnType<typeof rust.osName>> | null>(null);
    const availableMem = ref(0);
    const totalMem = ref(0);
    const maxMemUsage = ref(0);

    const buildGameTree = async () => {
        gameTreeBuilt.value = false;
        buildState.value = 'building';

        const err = await rust.buidGameTree(
            config.board,
            config.oopRange,
            config.ipRange,
            config.startingPot,
            config.effectiveStack,
            config.rake / 100,
            config.rakeCap,
            config.addAllInThreshold / 100,
            config.forceAllInThreshold / 100,
            config.betSizes[0][0][0],
            config.betSizes[0][0][1],
            config.betSizes[0][1][0],
            config.betSizes[0][1][1],
            config.betSizes[0][2][0],
            config.betSizes[0][2][1],
            config.betSizes[1][0][0],
            config.betSizes[1][0][1],
            config.betSizes[1][1][0],
            config.betSizes[1][1][1],
            config.betSizes[1][2][0],
            config.betSizes[1][2][1],
        );

        if (err) {
            buildState.value = 'idle';
            status.value = ["Error: " + err, "error"];
            return;
        }

        [memUncompressed.value, memCompressed.value] = await rust.memoryUsageGame();
        osName.value = await rust.osName();
        [availableMem.value, totalMem.value] = await rust.memory();
        
        if (osName.value === 'macos') {
            maxMemUsage.value = totalMem.value * 0.7;
        } else {
            maxMemUsage.value = availableMem.value;
        }

        if (memUncompressed.value > maxMemUsage.value && memCompressed.value <= maxMemUsage.value) {
            compression.value = true;
        } else {
            compression.value = true;
        }

        gameTreeBuilt.value = true;
        buildState.value = 'built';
        status.value = ["Built Successfully", "success"];
        app.treeHash = config.configHash;

        app.solverRunning = false;
        app.solverPaused = false;
        app.solverFinished = false;
        app.solverError = false;
    }

    const terminateFlag = ref(false);
    const pauseFlag = ref(false);
    const currentIter = ref(-1);
    const currentDelta = ref(Number.POSITIVE_INFINITY);
    const timeElapsed = ref(-1);

    let timeStart = 0;
    let deltaUpdated = false;

    const runSolver = async () => {

        terminateFlag.value = false;
        pauseFlag.value = false;
        
        currentIter.value = -1;
        currentDelta.value = Number.POSITIVE_INFINITY;
        timeElapsed.value = -1;
        app.solverRunning = true;
        timeStart = performance.now();

        await rust.setNumThreads(numThreads.value);
        await rust.allocateMemoryGame(compression.value);

        currentIter.value = 0;
        currentDelta.value = Math.max(await rust.exploitabilityGame(), 0);
        deltaUpdated = true;

        await resumeSolver();
    };

    const resumeSolver = async () => {
        app.solverRunning = true;
        app.solverPaused = false;

        if (timeStart === 0) {
            timeStart = performance.now();
            await rust.setNumThreads(numThreads.value);
        }

        const target = (config.startingPot * targetDelta.value) / 100;

        while (!terminateFlag.value && currentIter.value < maxIters.value && currentDelta.value > target) {

            if (pauseFlag.value) {
                const timePaused = performance.now();
                timeElapsed.value += timePaused - timeStart;
                timeStart = 0;
                pauseFlag.value = false;
                app.solverRunning = false;
                app.solverPaused = true;
                return;
            }

            await rust.solveStepGame(currentIter.value);
            ++currentIter.value;
            deltaUpdated = false;

            if (currentIter.value % 10 === 0) {
                currentDelta.value = Math.max(await rust.exploitabilityGame(), 0);
                deltaUpdated = true;
            }
        }

        if (!deltaUpdated) {
            currentDelta.value = Math.max(await rust.exploitabilityGame(), 0);
        }

        app.solverRunning = false;
        await rust.finaliseGame();
        app.solverFinished = true;

        const timeEnd = performance.now();
        timeElapsed.value = timeEnd - timeStart;
    };

    const exploitabilityText = computed(() => {
        if (!Number.isFinite(currentDelta.value)) {
            return "null";
        } else {
            const percent = ((currentDelta.value * 100) / config.startingPot).toFixed(2);
            return `${currentDelta.value.toFixed(2)} (${percent}%)`
        }
    });

    const previewTreeKey = ref(0);
    const showModal = (modal: string) => {
        switch (modal) {
            case 'range_oop':
                // @ts-ignore
                window.range_oop_modal.showModal();
                break;
            case 'range_ip':
                // @ts-ignore
                window.range_ip_modal.showModal();
                break;
            case 'preview_tree':
                // This is a hack to force the preview tree to re-render.
                previewTreeKey.value += 1;
                // @ts-ignore
                window.preview_tree.showModal();
                break;
            default:
                break;
        }
    };

</script>

<template>
    <!-- Config Grid -->
    <div class="grid grid-cols-2 auto-rows-auto gap-x-5 gap-y-3 px-5 my-5 w-fit h-fit justify-items-center">
        
        <!-- Ranges -->
        <div class="flex gap-4 p-3 justify-evenly items-center m-auto">
            <MiniRange :oop="true" class="hover:blur-sm rounded-md" @click="showModal('range_oop')"/>
            <div class="divider divider-horizontal m-0"></div>
            <MiniRange :oop="false" class=" hover:blur-sm rounded-md" @click="showModal('range_ip')"/>
        </div>
        
        <!-- Board -->
        <Board class="w-full" />

        <div class="divider col-span-2 my-0 w-full"></div>

        <!-- Bet Sizes -->
        <div class="flex col-span-2 justify-center p-3">
            <div class="flex flex-col w-fit gap-6">
                <h1 class="text-2xl font-bold -mt-2">OOP Bet Sizes</h1>
                <BetSizes :player="1" />
            </div>
        </div>

        <div class="divider col-span-2 my-0 w-full"></div>

        <div class="flex col-span-2 justify-center p-3">
            <div class="flex flex-col w-fit gap-6">
                <div class="flex flex-row justify-between items-center">
                    <h1 class="text-2xl font-bold -mt-2">IP Bet Sizes</h1>
                    <button class="btn btn-sm btn-primary" @click="config.copyBets">Copy OOP</button>
                </div>
                <BetSizes :player="2" />
            </div>
        </div>
        
        <div class="divider col-span-2 my-0 w-full"></div>

        <!-- Tree Config -->
        <div class="flex flex-col items-center p-3 gap-7 w-fit">
            <h1 class="text-2xl font-bold self-start">Game Tree</h1>
            <div class="grid grid-cols-[7rem,5rem,7rem,5rem] items-center gap-y-9 gap-x-7">

                <p>Starting Pot:</p>
                <input
                    v-model="config.startingPot"
                    type="number"
                    min="1"
                    class="input input-bordered input-primary input-sm w-full"
                />
                
                <p>Effective Stack:</p>
                <input
                    v-model="config.effectiveStack"
                    type="number"
                    min="1"
                    class="input input-bordered input-primary input-sm w-full"
                />

                <p>Rake %:</p>
                <input
                    v-model="config.rake"
                    type="number"
                    min="0"
                    max="100"
                    step="0.5"
                    class="input input-bordered input-primary input-sm w-full"
                />

                <p>Rake Cap:</p>
                <input
                    v-model="config.rakeCap"
                    type="number"
                    min="0"
                    class="input input-bordered input-primary input-sm w-full"
                />

                <p>Add All-In %:</p>
                <input
                    v-model="config.addAllInThreshold"
                    type="number"
                    min="0"
                    step="5"
                    class="input input-bordered input-primary input-sm w-full"
                />

                <p>Force All-In %:</p>
                <input
                    v-model="config.forceAllInThreshold"
                    type = "number"
                    min="0"
                    step="5"
                    class="input input-bordered input-primary input-sm w-full"
                />
            </div>
            
            <div v-if="config.isInvalid !== ''" class="alert alert-error mt-2">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span class="font-bold">{{ config.isInvalid }}</span>
            </div>

            <div v-else class="flex flex-row justify-around w-full mt-2">
                <button class="btn btn-primary" :disabled="config.isInvalid !== ''" @click="showModal('preview_tree')">
                    Preview Action Tree
                </button>
                <button
                    class="btn btn-primary"
                    :disabled="buildState === 'building'"
                    @click="buildGameTree"
                >
                    Build Game Tree
                </button>
            </div>

        </div>
        
        <!-- Solver -->
        <div class="flex flex-col p-3 gap-7">
            <div class="flex flex-row items-center justify-between gap-5">
                <h1 class="text-2xl font-bold">Run Solver</h1>
                <div v-if="gameTreeBuilt" class="badge badge-info badge-lg">Game ID: {{ app.treeHash }}</div>
                <div v-else class="badge badge-error badge-lg">Game Tree Not Built</div>
            </div>

            <div class="grid grid-cols-[7rem,5rem,7rem,5rem] items-center gap-y-9 gap-x-7">
                <p>Threads: </p>
                <input
                    class="input input-bordered input-primary w-20 input-sm text-center"
                    v-model="numThreads"
                    type="number"
                    min="1"
                    max="64"
                />

                <p>Exploitability %: </p>
                <input
                    class="input input-bordered input-primary w-20 input-sm text-center"
                    v-model="targetDelta"
                    type="number"
                    min="0.05"
                    step="0.05"
                />

                <p>Max Iterations: </p>
                <input
                    class="input input-bordered input-primary w-20 input-sm text-center"
                    v-model="maxIters"
                    type="number"
                    min="1"
                />

                <div class="flex flex-row justify-between items-center col-span-2">
                    <p>Compression:</p>
                    <div class="form-control">
                        <label class="cursor-pointer label justify-start gap-2">
                            <input 
                                type="checkbox" 
                                class="toggle toggle-primary" 
                                v-model="compression"
                            >
                            <span v-if="compression" class="label-text">{{ memUsageCompressedString }}</span>
                            <span v-else class="label-text">{{ memUsageUncompressedString }}</span>
                        </label>
                    </div>
                </div>
            </div>

            <div class="flex flex-row items-center justify-around mt-5">
                <div class="join">

                    <button 
                        class="btn btn-primary join-item"
                        @click="runSolver"
                        :disabled="app.solverRunning || app.solverRun || !gameTreeBuilt"
                    >
                        Solve
                    </button>

                    <button
                        class="btn btn-primary join-item"
                        :disabled="!app.solverRunning"
                        @click="() => (pauseFlag = true)"
                    >
                        Pause
                    </button>

                    <button
                        class="btn btn-primary join-item"
                        :disabled="!app.solverPaused"
                        @click="resumeSolver"
                    >
                        Resume
                    </button>

                </div>

                <div class="flex flex-col gap-2">
                    <p>Status: {{ 
                        app.solverRunning 
                        ? "Running" 
                        : app.solverPaused 
                        ? "Solver"
                        : app.solverError 
                        ? "Error"
                        : "Finished" 
                    }}</p>
                
                    <p>Exploitability: {{ exploitabilityText }}</p>

                    <p>Time: {{ (timeElapsed === -1 || !app.solverFinished) ? "null" : `${(timeElapsed / 1000).toFixed(2)}s`}}</p>
                    
                </div>
            
            </div>
        </div>
    </div>

    <dialog id="range_oop_modal" class="modal">
        <form method="dialog" class="modal-box w-fit max-w-none border-2 border-accent xl:p-10">
            <Range :oop="true" />
        </form>
    </dialog>

    <dialog id="range_ip_modal" class="modal">
        <form method="dialog" class="modal-box w-fit max-w-none border-2 border-accent xl:p-10">
            <Range :oop="false" />
        </form>
    </dialog>

    <dialog id="preview_tree" class="modal">
        <form method="dialog" class="modal-box w-fit max-w-none border-2 border-accent xl:p-10">
            <button className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            <Suspense>
                <template #default>
                    <div class="flex flex-col gap-5">
                        <h1 class="text-2xl font-bold mr-5">Action Tree Preview</h1>
                        <PreviewTree :key="previewTreeKey" />
                    </div>
                </template>
                <template #fallback>
                    <div class="flex flex-row items-center justify-center gap-5 p-7">
                        <span className="loading loading-spinner loading-lg"></span>
                        <p class="text-2xl">Loading...</p>
                    </div>
                </template>
            </Suspense>
        </form>
    </dialog>
</template>