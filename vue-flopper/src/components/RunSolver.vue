
<script setup lang="ts">
    import { computed, ref } from 'vue';
    import { useConfigStore, useStore } from '../store';
    import * as rust from '../rust_funcs';
    
    const config = useConfigStore();
    const app = useStore();

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
            return "";
        } else {
            const percent = ((currentDelta.value * 100) / config.startingPot).toFixed(2);
            return `Exploitability: ${currentDelta.value.toFixed(2)} (${percent}%)`
        }
    });

</script>

<template>

    <div class="flex w-full justify-center">
        <div class="flex flex-col gap-5">

            <div class="grid grid-cols-[7rem,10rem] gap-5">
                <button
                    class="btn btn-primary btn-sm col-span-2"
                    :disabled="buildState === 'building'"
                    @click="buildGameTree"
                >
                    Build Game Tree
                </button>

                <p>Status: </p>
                <div :class="'badge badge-' + status[1]">
                    {{ status[0] }}
                </div>

                <p v-if="gameTreeBuilt">ID: </p>
                <div v-if="gameTreeBuilt" class="badge badge-info">
                    {{ app.treeHash }}
                </div>
            </div>

            <div v-if="gameTreeBuilt" class="flex flex-col gap-5">
                <div class="divider my-0"></div>
                <h1 class="text-2xl font-bold">Run Solver</h1>
            
                <div class="flex flex-row gap-4 items-center">
                    <p>Number of threads: </p>
                    <input
                        class="input input-bordered w-20 input-sm text-center"
                        v-model="numThreads"
                        type="number"
                        min="1"
                        max="64"
                    />
                </div>

                <div>
                    <p class="mb-2">Select precision mode: </p>
                    <div class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text">32-bit: {{ memUsageUncompressedString }}</span>
                            <input
                                class="radio radio-primary"
                                type="radio"
                                name="radio-10"
                                v-model="compression"
                                :value="false"
                            />
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text">16-bit: {{ memUsageCompressedString }}</span>
                            <input
                                class="radio radio-primary"
                                type="radio"
                                name="radio-10"
                                v-model="compression"
                                :value="true"
                            />
                        </label>
                    </div>
                </div>

                <div class="flex flex-row gap-4 items-center">
                    <p>Target delta: </p>
                    <input
                        class="input input-bordered w-20 input-sm text-center"
                        v-model="targetDelta"
                        type="number"
                        min="0.05"
                        step="0.05"
                    />
                    <p>%</p>
                </div>

                <div class="flex flex-row gap-4 items-center">
                    <p>Max iterations: </p>
                    <input
                        class="input input-bordered w-30 input-sm text-center"
                        v-model="maxIters"
                        type="number"
                        min="1"
                    />
                </div>

                <div class="flex flex-row gap-4 items-center">

                    <button 
                        class="btn btn-primary"
                        @click="runSolver"
                        :disabled="app.solverRunning || app.solverRun"
                    >
                        Run Solver
                    </button>

                    <button
                        class="btn btn-primary"
                        :disabled="!app.solverRunning"
                        @click="() => (pauseFlag = true)"
                    >
                        Pause
                    </button>

                    <button
                        class="btn btn-primary"
                        :disabled="!app.solverPaused"
                        @click="resumeSolver"
                    >
                        Resume
                    </button>
                </div>

            </div>

            <div v-if="app.solverRun">
                <div class="flex flex-row items-center">
                    <span v-if="app.solverRunning" className="loading loading-spinner loading-xs"></span>
                    {{ 
                        app.solverRunning 
                        ? "Running Solver" 
                        : app.solverPaused 
                        ? "Solver Paused"
                        : app.solverError 
                        ? "Solver Error"
                        : "Solver Finished" 
                    }}
                </div>

                {{ currentIter === -1 ? "Allocationg Memory" : `Iteration: ${currentIter}` }}
                
                {{ exploitabilityText }}

                {{ (timeElapsed === -1 || !app.solverFinished) ? "" : `Time: ${(timeElapsed / 1000).toFixed(2)}s`}}

            </div>
        </div>
    </div>
</template>