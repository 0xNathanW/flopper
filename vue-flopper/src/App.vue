
<script setup lang="ts">
    import NavBar from './components/NavBar.vue';
    import Config from './components/Config.vue';
    import Results from './components/Results.vue';
    import Settings from './components/Settings.vue';
    import { useStore } from './store';
    const store = useStore();
</script>

<template>
    <div class="w-full" :data-theme="store.theme">
        <NavBar />
        <div v-show="store.mainPanel === 'config'" class="flex w-full h-[calc(100vh-5rem)] justify-center">
            <Config />
        </div>
        <div v-show="store.mainPanel === 'results'">
            <Results />
        </div>
        <div v-show="store.mainPanel === 'settings'" class="flex w-full h-[calc(100vh-5rem)] justify-center">
            <Settings />
        </div>
    </div>
</template>
