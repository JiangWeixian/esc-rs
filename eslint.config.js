const { aiou } = require('@aiou/eslint-config')

module.exports = aiou({ ssr: false }, [
  {
    ignores: ['index.js', 'index.d.ts', '**/fixtures/**'],
  },
])
