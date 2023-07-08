import devtools from "@vue/devtools";
import { createApp } from "vue";
import { createPinia } from "pinia";
import "./styles.css";
import App from "./App.vue";

createApp(App).use(createPinia()).mount("#app");

if (process.env.NODE_ENV === "development") devtools.connect();