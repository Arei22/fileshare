import eslint from "@eslint/js";
import prettier from "eslint-plugin-prettier/recommended";
import { defineConfig } from "eslint/config";
import typescript from "typescript-eslint";

import tsconfig from "./tsconfig.json";

export default defineConfig([
    {
        extends: [
            eslint.configs.recommended,
            typescript.configs.strictTypeChecked,
            typescript.configs.stylisticTypeChecked,
            prettier,
        ],
        files: tsconfig.include,

        languageOptions: {
            parserOptions: {
                projectService: true,
            },
        },

        rules: {
            "no-restricted-syntax": [
                "error",
                {
                    selector:
                        ':matches(PropertyDefinition, MethodDefinition)[accessibility="private"]',
                    message: "Use `#private` members instead.",
                },
            ],
            "object-shorthand": "error",
            "prefer-arrow-callback": "error",
            "@typescript-eslint/naming-convention": [
                "error",
                {
                    selector: ["interface", "typeAlias", "enum", "class"],
                    format: ["PascalCase"],
                },
                {
                    selector: ["function"],
                    format: ["camelCase"],
                },
                {
                    selector: ["variable"],
                    format: ["camelCase", "UPPER_CASE"],
                },
            ],
            "@typescript-eslint/restrict-template-expressions": [
                "error",
                {
                    allowNumber: true,
                },
            ],
        },
    }
]);