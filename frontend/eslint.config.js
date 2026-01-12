import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import prettier from 'eslint-config-prettier';
import svelte from 'eslint-plugin-svelte';

export default tseslint.config(
  { ignores: ['build/**', '.svelte-kit/**'] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  prettier,
  {
    languageOptions: {
      globals: globals.browser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: 'module'
      }
    },
    plugins: {
      svelte
    },
    rules: {
      // Svelte rules
      'svelte/require-styles-at-layer': 'error',
      'svelte/no-at-html-tags': 'error',

      // JS/TS
      '@typescript-eslint/no-unused-vars': 'warn',
      'no-console': 'warn'
    }
  }
);
