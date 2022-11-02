use anyhow::{Ok, Result};
use build_common::generate_envs;

fn main() -> Result<()> {
    // Generate the default 'cargo:' instruction output
    generate_envs();
    Ok(())
}
