use std::fs;
use std::path::Path;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let old_replace = "replace.exe.old";
    let real_replace = "replace.exe";
    let new_replace = "replace.exe.new";
    let new_version = "new-version.exe";
    let replace_backup = "replace.exe.bak";

    if is_file(old_replace) {
        fs::remove_file(&old_replace)?;
        eprintln!("Delete: {}", old_replace);
    }
    if !(is_file(new_version) && is_file(replace_backup)) {
        bail!(
            "Error: both '{}' and '{}' must exist to reset safely",
            new_version,
            replace_backup
        );
    }
    fs::copy(replace_backup, real_replace)?;
    eprintln!("Copy: {} -> {}", replace_backup, real_replace);
    fs::copy(new_version, new_replace)?;
    eprintln!("Copy: {} -> {}", new_version, new_replace);
    Ok(())
}

// This makes str -> path conversion trivial.
fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}
