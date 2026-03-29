import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import prettier from "eslint-config-prettier";
import solid from "eslint-plugin-solid";
import globals from "globals";
import tseslint from "typescript-eslint";

const tsconfigRootDir = path.dirname(fileURLToPath(import.meta.url));
const sourceFiles = ["src/**/*.{ts,tsx}"];

const strictTypeCheckedConfigs = [
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
].map((config) => ({
  ...config,
  files: sourceFiles,
}));

export default [
  {
    ignores: ["dist/**", "node_modules/**", "src/wasm/pkg/**", "tsconfig.tsbuildinfo"],
  },
  js.configs.recommended,
  ...strictTypeCheckedConfigs,
  {
    ...solid.configs["flat/typescript"],
    files: sourceFiles,
  },
  {
    files: sourceFiles,
    plugins: {
      "@typescript-eslint": tseslint.plugin,
    },
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.browser,
        ...globals.es2022,
      },
      parserOptions: {
        projectService: true,
        tsconfigRootDir,
      },
    },
    rules: {
      "@typescript-eslint/consistent-type-imports": [
        "error",
        {
          prefer: "type-imports",
          fixStyle: "separate-type-imports",
        },
      ],
      "@typescript-eslint/no-misused-promises": [
        "error",
        {
          checksVoidReturn: {
            attributes: false,
          },
        },
      ],
      "@typescript-eslint/no-unnecessary-condition": "error",
      "@typescript-eslint/switch-exhaustiveness-check": "error",
      "no-console": [
        "error",
        {
          allow: ["warn", "error"],
        },
      ],
      "solid/components-return-once": "error",
      "solid/event-handlers": "error",
      "solid/imports": "error",
      "solid/reactivity": "error",
      "solid/self-closing-comp": "error",
      "solid/style-prop": "error",
    },
  },
  {
    files: ["vite.config.ts"],
    plugins: {
      "@typescript-eslint": tseslint.plugin,
    },
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.node,
      },
      parser: tseslint.parser,
    },
    rules: {
      "@typescript-eslint/consistent-type-imports": [
        "error",
        {
          prefer: "type-imports",
          fixStyle: "separate-type-imports",
        },
      ],
    },
  },
  {
    files: ["scripts/**/*.{js,mjs,cjs}"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.node,
      },
    },
    rules: {
      "no-console": "off",
    },
  },
  prettier,
];
