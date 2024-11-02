import { defineConfig } from "tsup";
import fs from "fs";
import path from "path";

const production = process.argv.includes("--production");
const watch = process.argv.includes("--watch");

export default defineConfig({
    entry: ["client/extension.ts"],
    format: ["cjs"],
    minify: production,
    sourcemap: !production,
    outDir: "dist",
    dts: false,
    clean: true,
    watch,
    platform: "node",
    external: ["vscode", "vscode-languageclient"],
    onSuccess: async () => {
        // Copy the server binary to the dist folder
        const file = path.join(
            __dirname,
            "server",
            "target",
            "release",
            "roan-language-server.exe",
        );

        if (!fs.existsSync(file)) {
            console.error("Server binary not found.");
            process.exit(1);
        }

        fs.copyFileSync(
            file,
            path.join(__dirname, "dist", "roan-language-server.exe"),
        );
    },
});
