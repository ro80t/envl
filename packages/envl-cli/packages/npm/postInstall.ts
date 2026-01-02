import pkgJson from "../../package.json";
import { installEnvl } from "./install";
import { Entry, open } from "yauzl";
import { createWriteStream, unlinkSync, writeFileSync } from "fs";
import { join } from "path";

const version = process.env["ENVL_VERSION"] ?? pkgJson.version;
const dirname = import.meta.dirname;
const tmpZipPath = join(dirname, "./envl.zip");
const unzipedPath = join(dirname, "./envl.exe");

await installEnvl(version)
    .then(async (data) => {
        writeFileSync(tmpZipPath, Buffer.from(data));

        open(tmpZipPath, { lazyEntries: true }, (err, zipfile) => {
            if (err) throw new Error("Cloud not unzip file.");

            zipfile.readEntry();
            zipfile.on("entry", (entry: Entry) => {
                if (/\/$/.test(entry.fileName)) {
                    zipfile.readEntry();
                } else {
                    zipfile.openReadStream(entry, (err, readStream) => {
                        if (err) throw err;

                        readStream.on("end", () => zipfile.readEntry());
                        readStream.pipe(createWriteStream(unzipedPath));
                    });
                }
            });
            zipfile.on("close", () => {
                unlinkSync(tmpZipPath);
            });
        });
    })
    .catch((err) => {
        console.error(err);
        process.exit(1);
    });
