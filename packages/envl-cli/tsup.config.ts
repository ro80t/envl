import { defineConfig } from "tsup";

export default defineConfig({
    entry: ["nodejs/index.ts"],
    dts: true,
    format: ["esm"],
    tsconfig: "tsconfig.json",
    clean: true,
    outDir: "dist"
});
