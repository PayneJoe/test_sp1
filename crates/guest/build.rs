use sp1_build::{BuildArgs, build_program_with_args};
use std::io::Result;

fn main() -> Result<()> {
    build_program_with_args(
        "./program",
        BuildArgs {
            binaries: vec!["vault_dispute".to_string(), "vault_pegout".to_string()],
            ..Default::default()
        },
    );

    Ok(())
}
