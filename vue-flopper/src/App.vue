
<script setup lang="ts">
    import NavBar from './components/NavBar.vue';
    import Config from './components/Config.vue';
    import Results from './components/Results.vue';
    import { useStore } from './store';
    const store = useStore();
</script>

<template>
    <div class="min-w-[1080px] min-h-[720px]">
        <NavBar />
        <div v-show="store.mainPanel === 'config'" class="">
            <Config />
        </div>
        <div v-show="store.mainPanel === 'results'">
            <Results />
        </div>
        <div v-show="store.mainPanel === 'settings'">
            <h1 class="text-2xl">SETTINGS</h1>
        </div>
    </div>
</template>
