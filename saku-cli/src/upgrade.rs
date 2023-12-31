use saku_lib::exec;
use saku_lib::prelude::*;

use saku_lib::pkg::data;

pub fn upgrade(name: &str) -> Result<()> {
    let pkg = data::get_pkg(name)?;

    exec::pull(&pkg.name)?;

    if pkg.update.len() > 0 {
        exec::run_in_repo(&name, pkg.update.clone())?;
    } else {
        crate::install::run_install(&pkg)?;
    }

    pkg.install_root()?;

    Ok(())
}
