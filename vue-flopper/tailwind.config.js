/** @type {import('tailwindcss').Config} */
export default {
    content: ["src/**/*.ts", "src/**/*.vue"],
    theme: {
        extend: {},
    },
    daisyui: {
        themes: ["retro"],
    },
    plugins: [require("daisyui")],
}

