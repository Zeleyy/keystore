import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import { ViteMinifyPlugin } from "vite-plugin-minify";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
    plugins: [
        react(),
        ViteMinifyPlugin({}),
    ],
    
    build: {
        target: "esnext",
        cssMinify: true,
        chunkSizeWarningLimit: 1000,

        rolldownOptions: {
            output: {
                codeSplitting: {
                    minSize: 10000,
                    groups: [
                        {
                            name: "vendor-react",
                            test: /node_modules[\\/]react/,
                            priority: 20,
                        },
                        {
                            name: "vendor-others",
                            test: /node_modules/,
                            priority: 10,
                        },
                    ]
                }
            }
        }
    },

    resolve: {
        alias: [
            { find: "@", replacement: "/src" },
        ],
    },
    
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
})
