import { defineConfig } from 'cypress'

export default defineConfig({
    chromeWebSecurity: false,
    video: true,
    videoCompression: 32,
    e2e: {
        setupNodeEvents(on, config) {
            // implement node event listeners here
        },
    },
})
