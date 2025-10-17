module.exports = {
    extends: [
        "airbnb-base",
        "prettier",
        "eslint:recommended",
        "plugin:import/recommended",
        "plugin:@typescript-eslint/recommended",
        "eslint-config-prettier",
    ],
    plugins: ["prettier"],
    rules: {
        "prettier/prettier": "error",
    },
};
