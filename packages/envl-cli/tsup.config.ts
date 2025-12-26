import { defineConfig } from "tsup";

export default defineConfig({
    entry: ["packages/npm/install.ts"],
    dts: true,
    format: ["esm"],
    tsconfig: "tsconfig.json",
    clean: true,
    outDir: "dist"
});
