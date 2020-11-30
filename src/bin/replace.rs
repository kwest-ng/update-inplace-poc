use std::fs;
use std::path::PathBuf;

use anyhow::Result;

fn rename() -> Result<()> {
    let old = PathBuf::from("replace.exe.old");
    let middle = PathBuf::from("replace.exe");
    let new = PathBuf::from("replace.exe.new");

    fs::rename(&middle, &old)?;
    fs::rename(&new, &middle)?;
    Ok(())
}

fn detect_new_version() -> bool {
    PathBuf::from("replace.exe.new").exists()
}

fn main() -> Result<()> {
    let replace = detect_new_version();
    if replace {
        rename()?;
        println!("Replaced bins");
    };
    Ok(())
}

