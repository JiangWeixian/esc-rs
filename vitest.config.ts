import { defineConfig } from 'vitest/config'

const config = defineConfig({
  test: {
    watch: !process.env.CI_TEST,
  },
})

export default config
