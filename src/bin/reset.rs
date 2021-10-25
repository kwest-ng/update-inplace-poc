use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let old_replace = PathBuf::from("replace.exe.old");
    let real_replace = PathBuf::from("replace.exe");
    let new_replace = PathBuf::from("replace.exe.new");
    let stub = PathBuf::from("stub.exe");
    let replace_backup = PathBuf::from("replace.new.bak");

    if old_replace.is_file() {
        fs::remove_file(&old_replace)?;
    }
    if !(stub.is_file() && replace_backup.is_file()) {
        bail!("Error: both '{}' and '{}' must be existing files to reset safely", stub.display(), replace_backup.display());
    }
    fs::copy(replace_backup, real_replace)?;
    fs::copy(stub, new_replace)?;
    Ok(())
}
