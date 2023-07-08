
<script setup lang="ts">
    import { useStore, useConfigStore } from '../store';
    import { suitColour, idxToCard } from '../util';
    import MiniRange from './MiniRange.vue';
    import Range from './Range.vue';
    import Board from './Board.vue';
    import TreeConfig from './TreeConfig.vue';
    import PreviewTree from './PreviewTree.vue';
    import RunSolver from './RunSolver.vue';

    const app = useStore();
    const config = useConfigStore();
</script>

<template>
    <div class="drawer absolute lg:drawer-open">
        <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
        
        <!-- config panel content -->
        <div class="drawer-content mt-7 px-5 justify-center">
            
            <Range v-show="app.configPanel === 'rangeOOP'" :oop="true" />

            <Range v-show="app.configPanel === 'rangeIP'" :oop="false" />

            <Board v-show="app.configPanel === 'board'" />

            <TreeConfig v-show="app.configPanel === 'treeConfig'" />

            <div v-if="app.configPanel === 'preview'">
                <h1 class="text-2xl font-bold">Preview Tree:</h1>
                <div class="divider my-0"></div>
                <Suspense>
                    <template #default>
                        <PreviewTree />
                    </template>
                    <template #fallback>
                        <div>Loading...</div>
                    </template>
                </Suspense>
                <div class="divider my-0"></div>                
            </div>

            <RunSolver v-show="app.configPanel === 'run'" />

            <label htmlFor="my-drawer-2" class="btn btn-base-200 drawer-button lg:hidden self-start ml-3 fixed top-16 left-2">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-6 h-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" />
                </svg>
            </label>
        </div>

        <!-- config side panel -->
        <div className="drawer-side top-auto">
            <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
            <ul className="menu p-4 w-75 h-full bg-base-200 text-base-content text-xl">
                <!-- side panel contents -->
                <li @click="app.configPanel = 'rangeOOP'"><MiniRange :oop="true" /></li>

                <li @click="app.configPanel = 'rangeIP'"><MiniRange :oop="false" /></li>

                <li @click="app.configPanel = 'board'">
                    <a>
                        <div class="flex flex-col gap-1">
                            Board
                            
                            <div v-if="config.board.length >= 3">
                                <h1>
                                    <span 
                                        v-for="card in config.board" 
                                        :key="card"
                                        :class="'text-2xl font-bold ml-1 ' + suitColour(idxToCard(card)[1])"
                                    >
                                        {{ idxToCard(card) }}
                                    </span>
                                </h1>
                            </div>

                            <div v-else class="badge badge-warning badge-md mt-1">3 cards minimum required!</div>
                        </div>
                    </a>
                </li>

                <li @click="app.configPanel = 'treeConfig'"><a>Tree Config</a></li>

                <div class="divider"></div>

                <button
                    class="btn btn-outline mb-4"
                    :disabled="config.configInvalid"
                    @click="app.configPanel = 'preview'"
                >Preview Tree</button>

                <button 
                    class="btn btn-outline"
                    :disabled="config.configInvalid"
                    @click="app.configPanel = 'run'"
                >Build & Run</button>
            </ul>
        </div>
    </div>
</template>