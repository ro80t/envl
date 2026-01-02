import { defineConfig } from "tsup";

export default defineConfig({
    entry: ["packages/npm/postInstall.ts"],
    dts: true,
    format: ["esm"],
    tsconfig: "tsconfig.json",
    clean: true,
    outDir: "dist"
});
