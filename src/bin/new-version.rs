use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

const ACTUALLY_DELETE: bool = true;

fn do_delete(p: &Path) -> Result<()> {
    if ACTUALLY_DELETE {
        fs::remove_file(p)?;
    } 
    eprintln!("delete: {}", p.display());

    Ok(())
}

fn delete_old_bin() -> Result<()> {
    let old_bin = PathBuf::from("replace.exe.old");
    if old_bin.is_file() {
        do_delete(&old_bin)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    eprintln!("Version: new");
    if let Err(e) = delete_old_bin() {
        eprintln!("{}", e);
    }
    Ok(())
}
