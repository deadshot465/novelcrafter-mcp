import pluginVue from 'eslint-plugin-vue';

export default [
    pluginVue.configs["flat/recommended"],
    {
        files: ["src/**/*.js", "src/**/*.ts", "src/**/*.vue"],
        ignores: ["**/*.config.js"],
        rules: {
            semi: "error"
        }
    }
]