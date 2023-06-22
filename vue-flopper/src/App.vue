
<script setup lang="ts">
    import NavBar from './components/NavBar.vue'
    import Config from './components/Config.vue'
    import { useStore } from './store';
    const store = useStore();
</script>

<template>
    <NavBar />
     
    <div v-show="store.mainPanel === 'config'">
        <Config />
    </div>

</template>
