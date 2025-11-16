import { readFile } from "fs/promises";
import { join } from "path";
import { parse } from "smol-toml";

import pkgJson from "../packages/envl-cli/package.json";

const __dirname = import.meta.dirname;

const cargoToml = parse(
    await readFile(join(__dirname, "../packages/envl-cli/Cargo.toml"), "utf-8")
) as {
    package?: {
        version?: string;
    };
};

if (!pkgJson.version) {
    throw new Error("Version is not found in package.json");
}

if (cargoToml.package && cargoToml.package.version) {
    if (pkgJson.version === cargoToml.package.version) {
        console.log("Versions in Cargo.toml and package.json are identical");
    } else {
        throw new Error("Versions in Cargo.toml and package.json aren't identical");
    }
} else {
    throw new Error("Version is not found in Cargo.toml");
}
