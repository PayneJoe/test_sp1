fn main() {
    let docker = std::env::var("SP1_DOCKER")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    sp1_build::build_program_with_args(
        "../pegout_test_program",
        sp1_build::BuildArgs {
            warning_level: sp1_build::WarningLevel::Minimal,
            docker: true,
            ..Default::default()
        },
    );
}
