use clap::{Parser, ValueEnum};
use std::process::{Command, ExitCode};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Action {
    Run,
    Check,
    Build,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Chip {
    C2,
    C3,
    C6,
    H2,
    Esp32,
    S2,
    S3,
}

impl Chip {
    fn target(self) -> &'static str {
        match self {
            Chip::C2 | Chip::C3 => "riscv32imc-unknown-none-elf",
            Chip::C6 | Chip::H2 => "riscv32imac-unknown-none-elf",
            Chip::Esp32 => "xtensa-esp32-none-elf",
            Chip::S2 => "xtensa-esp32s2-none-elf",
            Chip::S3 => "xtensa-esp32s3-none-elf",
        }
    }

    fn feature(self) -> &'static str {
        match self {
            Chip::C2 => "esp32c2",
            Chip::C3 => "esp32c3",
            Chip::C6 => "esp32c6",
            Chip::H2 => "esp32h2",
            Chip::Esp32 => "esp32",
            Chip::S2 => "esp32s2",
            Chip::S3 => "esp32s3",
        }
    }

    fn example_chip_segment(self) -> &'static str {
        match self {
            Chip::C2 => "c2",
            Chip::C3 => "c3",
            Chip::C6 => "c6",
            Chip::H2 => "h2",
            Chip::Esp32 => "esp32",
            Chip::S2 => "s2",
            Chip::S3 => "s3",
        }
    }

    fn uses_xtensa_toolchain(self) -> bool {
        matches!(self, Chip::Esp32 | Chip::S2 | Chip::S3)
    }
}

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "Cross-platform chip/example runner for device-envoy-esp-blinky")]
struct Cli {
    #[arg(value_enum)]
    action: Action,

    #[arg(long, value_enum, default_value = "c6")]
    chip: Chip,

    /// Board slug to run/check/build copied board examples, e.g. generic or devkitc1_n8
    #[arg(long)]
    board: Option<String>,

    /// Optional example target name (for test workflows), e.g. blinky_c3_generic
    #[arg(long)]
    example: Option<String>,

    /// Force release profile (run/build already default to release)
    #[arg(long)]
    release: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if cli.board.is_some() && cli.example.is_some() {
        eprintln!("error: pass only one of --board or --example");
        return ExitCode::FAILURE;
    }

    let mut cargo = Command::new("cargo");

    if cli.chip.uses_xtensa_toolchain() {
        cargo.arg("+esp");
    }

    cargo.arg(match cli.action {
        Action::Run => "run",
        Action::Check => "check",
        Action::Build => "build",
    });

    cargo.arg("--target").arg(cli.chip.target());

    let use_release = cli.release || !matches!(cli.action, Action::Check);
    if use_release {
        cargo.arg("--release");
    }

    cargo
        .arg("--no-default-features")
        .arg("--features")
        .arg(cli.chip.feature());

    if let Some(example) = cli.example.as_deref() {
        cargo.arg("--example").arg(example);
    } else if let Some(board) = cli.board.as_deref() {
        let example = format!("blinky_{}_{}", cli.chip.example_chip_segment(), board);
        cargo.arg("--example").arg(example);
    }

    if cli.chip.uses_xtensa_toolchain() {
        cargo.arg("-Zbuild-std=core,alloc");
    }

    let status = cargo.status().unwrap_or_else(|error| {
        panic!("failed to launch cargo: {error}");
    });

    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
