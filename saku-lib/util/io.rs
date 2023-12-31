use std::fs;

use crate::prelude::*;

use super::path;

pub fn mkdir(path: String) -> Result<bool> {
    if path::exists(&path) {
        return Ok(false);
    }

    fs::create_dir(path)?;

    Ok(true)
}

pub fn link(target: &str, path: &str) -> Result<()> {
    std::os::unix::fs::symlink(target, path)?;

    Ok(())
}
