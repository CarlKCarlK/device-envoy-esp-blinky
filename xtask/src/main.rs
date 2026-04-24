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
    C5,
    C6,
    C61,
    H2,
    Esp32,
    S2,
    S3,
}

// Change this once for your project if you do not want to pass --chip every time.
const DEFAULT_CHIP: Chip = Chip::C6;

impl Chip {
    fn target(self) -> &'static str {
        match self {
            Chip::C2 | Chip::C3 => "riscv32imc-unknown-none-elf",
            Chip::C5 | Chip::C6 | Chip::C61 | Chip::H2 => "riscv32imac-unknown-none-elf",
            Chip::Esp32 => "xtensa-esp32-none-elf",
            Chip::S2 => "xtensa-esp32s2-none-elf",
            Chip::S3 => "xtensa-esp32s3-none-elf",
        }
    }

    fn feature(self) -> &'static str {
        match self {
            Chip::C2 => "esp32c2",
            Chip::C3 => "esp32c3",
            Chip::C5 => "esp32c5",
            Chip::C6 => "esp32c6",
            Chip::C61 => "esp32c61",
            Chip::H2 => "esp32h2",
            Chip::Esp32 => "esp32",
            Chip::S2 => "esp32s2",
            Chip::S3 => "esp32s3",
        }
    }

    fn uses_xtensa_toolchain(self) -> bool {
        matches!(self, Chip::Esp32 | Chip::S2 | Chip::S3)
    }
}

fn infer_chip_from_example_name(example_name: &str) -> Option<Chip> {
    let with_prefix = example_name.strip_prefix("blinky_")?;
    let chip_segment = with_prefix.split('_').next()?;
    match chip_segment {
        "c2" => Some(Chip::C2),
        "c3" => Some(Chip::C3),
        "c5" => Some(Chip::C5),
        "c6" => Some(Chip::C6),
        "c61" => Some(Chip::C61),
        "h2" => Some(Chip::H2),
        "esp32" => Some(Chip::Esp32),
        "s2" => Some(Chip::S2),
        "s3" => Some(Chip::S3),
        _ => None,
    }
}

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "Cross-platform chip/example runner for device-envoy-esp-blinky")]
struct Cli {
    #[arg(value_enum)]
    action: Action,

    #[arg(long, value_enum)]
    chip: Option<Chip>,

    /// Optional example target name, e.g. blinky_s3_devkitc1_v1_0_n16r8
    #[arg(long)]
    example: Option<String>,

    /// Force release profile (run/build already default to release)
    #[arg(long)]
    release: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let chip = cli
        .chip
        .or_else(|| {
            cli.example
                .as_deref()
                .and_then(infer_chip_from_example_name)
        })
        .unwrap_or(DEFAULT_CHIP);

    let mut cargo = Command::new("cargo");

    if chip.uses_xtensa_toolchain() {
        cargo.arg("+esp");
    }

    cargo.arg(match cli.action {
        Action::Run => "run",
        Action::Check => "check",
        Action::Build => "build",
    });

    cargo.arg("--target").arg(chip.target());

    let use_release = cli.release || !matches!(cli.action, Action::Check);
    if use_release {
        cargo.arg("--release");
    }

    cargo.arg("--features").arg(chip.feature());

    if let Some(example) = cli.example.as_deref() {
        cargo.arg("--example").arg(example);
    }

    if chip.uses_xtensa_toolchain() {
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
