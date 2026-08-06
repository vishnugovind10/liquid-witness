use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
use witness_core::{diff_claim, CabBundle, HolderAmount, IssuerClaim, ObservedState, Verdict};
use witness_lwk::{scan_fixture, scan_live_incomplete, ScanRequest};

#[derive(Parser)]
#[command(name = "witness")]
#[command(about = "Liquid/AMP witness for CAB-compatible asset-claim verification")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        #[arg(long)]
        asset_id: String,
        #[arg(long)]
        descriptor: String,
        #[arg(long, default_value = "testnet")]
        network: String,
        #[arg(long)]
        fixture: Option<PathBuf>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        claim: PathBuf,
        #[arg(long)]
        asset_id: String,
        #[arg(long)]
        descriptor: String,
        #[arg(long, default_value = "testnet")]
        network: String,
        #[arg(long)]
        fixture: Option<PathBuf>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    VerifyBundle {
        #[arg(long)]
        cab: PathBuf,
    },
    Export {
        #[arg(long, value_enum)]
        format: ExportFormat,
        #[arg(long)]
        out: PathBuf,
    },
}

#[derive(Clone, ValueEnum)]
enum ExportFormat {
    Cab,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let verdict = match cli.command {
        Commands::Scan {
            asset_id,
            descriptor,
            network,
            fixture,
            out,
        } => {
            let state = observed_state(asset_id, descriptor, network, fixture)?;
            write_json_or_stdout(&state, out)?;
            if state.demo {
                Verdict::Demo
            } else if state.complete {
                Verdict::Verified
            } else {
                Verdict::Incomplete
            }
        }
        Commands::Verify {
            claim,
            asset_id,
            descriptor,
            network,
            fixture,
            out,
        } => {
            let claim = read_claim(&claim)?;
            let observed = observed_state(asset_id, descriptor.clone(), network.clone(), fixture)?;
            let diff = diff_claim(&claim, &observed)?;
            let bundle = CabBundle::from_diff(
                &claim,
                observed,
                diff,
                network,
                descriptor_scope(&descriptor),
            );
            let verdict = bundle.verdict.clone();
            write_json_or_stdout(&bundle, out)?;
            verdict
        }
        Commands::VerifyBundle { cab } => {
            let bundle: CabBundle = read_json(&cab)?;
            println!("{}", serde_json::to_string_pretty(&bundle)?);
            bundle.verdict
        }
        Commands::Export { format, out } => {
            match format {
                ExportFormat::Cab => write_json_or_stdout(&example_bundle()?, Some(out))?,
            }
            Verdict::Demo
        }
    };

    std::process::exit(verdict.exit_code().0);
}

fn observed_state(
    asset_id: String,
    descriptor: String,
    network: String,
    fixture: Option<PathBuf>,
) -> Result<ObservedState> {
    if let Some(path) = fixture {
        return scan_fixture(&path)
            .with_context(|| format!("failed to load fixture {}", path.display()));
    }
    let request = ScanRequest {
        asset_id,
        descriptor,
        network,
        electrum_url: None,
    };
    scan_live_incomplete(&request).context("live scan boundary failed")
}

fn read_claim(path: &PathBuf) -> Result<IssuerClaim> {
    read_json(path).with_context(|| format!("failed to read issuer claim {}", path.display()))
}

fn read_json<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

fn write_json_or_stdout<T: serde::Serialize>(value: &T, out: Option<PathBuf>) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    if let Some(path) = out {
        fs::write(path, json)?;
    } else {
        println!("{json}");
    }
    Ok(())
}

fn descriptor_scope(descriptor: &str) -> String {
    let prefix: String = descriptor.chars().take(18).collect();
    format!("{prefix}...")
}

fn example_bundle() -> Result<CabBundle> {
    let asset_id = "ab".repeat(32);
    let claim = IssuerClaim {
        asset_id: asset_id.clone(),
        total_supply: 1000,
        holders: vec![
            HolderAmount {
                category: "qualified-investor".to_string(),
                amount: 700,
            },
            HolderAmount {
                category: "issuer-treasury".to_string(),
                amount: 300,
            },
        ],
    };
    let observed = ObservedState {
        asset_id,
        total_supply: 1000,
        holders: claim.holders.clone(),
        complete: true,
        demo: true,
        source: "examples/testnet-amp-scan/output.cab".to_string(),
    };
    let diff = diff_claim(&claim, &observed)?;
    Ok(CabBundle::from_diff(
        &claim,
        observed,
        diff,
        "testnet",
        "example watch-only descriptor",
    ))
}
