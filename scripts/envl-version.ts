import { readFile } from "fs/promises";
import { join } from "path";
import { parse } from "smol-toml";

import pkgJson from "../package.json";

const __dirname = import.meta.dirname;

const cargoToml = parse(await readFile(join(__dirname, "../Cargo.toml"), "utf-8")) as {
    workspace?: {
        package?: {
            version?: string;
        };
    };
};

if (!pkgJson.version) {
    throw new Error("Version is not found in package.json");
}

if (cargoToml.workspace && cargoToml.workspace.package && cargoToml.workspace.package.version) {
    if (pkgJson.version === cargoToml.workspace.package.version) {
        console.log("Versions in Cargo.toml and package.json are identical");
    } else {
        throw new Error("Versions in Cargo.toml and package.json aren't identical");
    }
} else {
    throw new Error("Version is not found in Cargo.toml");
}
