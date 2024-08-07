export default [ {
  root: true,
  env: { browser: true, es2020: true },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:react-hooks/recommended',
  ],
  ignorePatterns: ['dist'],
  parser: '@typescript-eslint/parser',
  plugins: ['react-refresh'],
  rules: {
    'react-refresh/only-export-components': [
      'warn',
      { allowConstantExport: true },
    ],
    'react-hooks/rules-of-hooks': 'off', // Disable the rule globally,
    'react-hooks/exhaustive-deps': 'off',
    '@typescript-eslint/no-explicit-any': "off",
    'react-refresh/only-export-components': 'off',
    '@typescript-eslint/no-unused-vars': 'off',
  },
}]
