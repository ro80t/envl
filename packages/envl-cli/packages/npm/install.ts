const repo = "https://github.com/ro80t/envl";

type Platform = {
    os: string;
    arch: string;
    bin: string;
    installFile: string;
};

const supportPlatform: Platform[] = [
    {
        os: "darwin",
        arch: "arm64",
        bin: "envl",
        installFile: "envl-aarch64-apple-darwin.zip"
    },
    {
        os: "linux",
        arch: "arm64",
        bin: "envl",
        installFile: "envl-aarch64-unknown-linux-gnu.zip"
    },
    {
        os: "darwin",
        arch: "x64",
        bin: "envl",
        installFile: "envl-x86_64-apple-darwin.zip"
    },
    {
        os: "win32",
        arch: "x64",
        bin: "envl.exe",
        installFile: "envl-x86_64-pc-windows-msvc.zip"
    },
    {
        os: "linux",
        arch: "x64",
        bin: "envl",
        installFile: "envl-x86_64-unknown-linux-gnu.zip"
    }
];

export async function installEnvl(version: string): Promise<ArrayBuffer> {
    const os = process.platform;
    const arch = process.arch;

    for (const platform of supportPlatform) {
        if (platform.os === os && platform.arch === arch) {
            return await downloadEnvl(version, platform);
        }
    }

    throw new Error(`${os}:${arch} is unsupported.`);
}

async function downloadEnvl(version: string, platform: Platform): Promise<ArrayBuffer> {
    const res = await fetch(`${repo}/releases/download/v${version}/${platform.installFile}`);
    const rawData = await res.arrayBuffer();
    return rawData;
}
