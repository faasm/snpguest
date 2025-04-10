mod certs;
mod fetch;
mod report;
pub mod verify;

use anyhow::{Context, Result};
use clap::{arg, Parser, Subcommand, ValueEnum};

pub use verify::attestation::verify_attestation;

