use std::{env::current_dir, fs::read_to_string};

use clap::{Parser, Subcommand};
use envl::{
    generator::{generate_file, GenerateOptions},
    load_envl_core,
    misc::filesystem::write_file,
};
use napi_derive::napi;

#[derive(Parser, Debug, Clone)]
#[clap(name = "envl", author = "ro80t")]
#[command(version, about, flatten_help = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Build {
        output: String,

        #[arg(short, long, long_help = "set language")]
        language: Option<String>,

        #[arg(long)]
        cjs: Option<bool>,
    },
}

fn get_config_file() -> String {
    let current_dir = current_dir().unwrap();
    let config_path = current_dir.join(".envlconf").display().to_string();

    read_to_string(config_path).unwrap()
}

#[napi]
pub fn cli() {
    let args = Args::parse();
    let current_dir = current_dir().unwrap();
    let config_path = current_dir.join(".envlconf").display().to_string();
    let config_code = get_config_file();

    match args.command {
        Command::Build {
            output,
            language,
            cjs,
        } => {
            let options = GenerateOptions { language, cjs };
            let data = load_envl_core(current_dir.clone(), config_path, config_code).unwrap();

            let f = generate_file(data, output.clone(), options).unwrap();
            write_file(current_dir.join(output).display().to_string(), f).unwrap();
        }
    }
}
