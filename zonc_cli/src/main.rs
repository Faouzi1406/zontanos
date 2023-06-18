use std::{env::current_dir, error::Error, fs::read_to_string, path::PathBuf, process::Command};

use clap::Parser;
use zontanos::compile;

#[derive(Parser)]
struct Cli {
    compile: PathBuf,

    #[arg(short, long)]
    out: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Ok(parse) = Cli::try_parse() else {
        return Err("Pleass give a path...".into());
    };

    if parse.compile.exists() {
        let Some(ends_with_zon) = parse.compile.extension() else {
            return Err("A zontanos file must end with .zon".into());
        };

        if ends_with_zon != "zon" {
            return Err("A zontanos file must end with .zon".into());
        }

        let read = read_to_string(parse.compile)?;
        compile(read)?;

        let current_path = current_dir()?;

        let mut llvm_l_path = current_path.clone();
        llvm_l_path.push("main.l");

        let llvm = Command::new("llvm-as").arg(&llvm_l_path).spawn();
        if llvm.is_err() {
            return Err("couldn't run llvm-as, perhaps you don't have llvm installed.".into());
        }

        let mut clang_path = current_path.clone();
        clang_path.push("main.l.bc");

        let clang = Command::new("clang").arg(&clang_path).arg("-o").arg(parse.out.unwrap_or("a.out".into())).spawn();
        if clang.is_err() {
            return Err("couldn't run llvm-as, perhaps you don't have llvm installed.".into());
        }

        let _ = clang.unwrap().wait_with_output();

        std::fs::remove_file(llvm_l_path)?;
        std::fs::remove_file(clang_path)?;
    } else {
        return Err(format!("path: {}, doesn't exist", parse.compile.display()).into());
    };

    Ok(())
}
