use anyhow::{Ok, Result};
use env_file_reader::read_file;

pub fn generate_envs() -> Result<()> {
    // Generate the default 'cargo:' instruction output

    // generate git info
    build_data::set_BUILD_TIMESTAMP();
    build_data::set_GIT_BRANCH();
    build_data::set_GIT_COMMIT();
    build_data::set_GIT_DIRTY();
    build_data::set_SOURCE_TIMESTAMP();
    build_data::no_debug_rebuilds();
    build_data::set_RUSTC_VERSION();
    //
    // println!("rerun-if-env-changed=COMMON_CANISTER_ENV");
    // let env = if let Some(env) = option_env!("COMMON_CANISTER_ENV") {
    //     env
    // } else {
    //     "dev"
    // };
    // println!("load env: {}", env);
    // println!("warning={}", env);
    //
    // // enable feature dev_env if env is dev
    // if env == "dev" {
    //     println!("cargo:rustc-cfg=feature=\"dev_env\"");
    // }
    //
    // // load env files
    // let env_parts = vec!["canister_ids", "config", "principals"];
    // for env_part in env_parts {
    //     let env_variables = read_file(format!("../../env_configs/{}.{}.env", env, env_part))?;
    //     for (key, value) in env_variables {
    //         println!("cargo:rustc-env={}={}", key, value.replace("\n", "||||"));
    //     }
    // }

    Ok(())
}
