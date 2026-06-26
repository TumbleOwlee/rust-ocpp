//! OCPP JSON-schema → Rust code generator.
//!
//! Two-stage pipeline: a schema-style-specific *frontend* lowers schema JSON
//! into a schema-agnostic [`ir::IrModel`], and a shared *backend* renders that
//! IR to Rust source. A new schema style only needs a new frontend.

mod backend;
mod ir;
mod naming;
mod primitives;
mod v2_x;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Generate rust-ocpp types from OCPP JSON schemas")]
struct Args {
    /// Version module name, e.g. `v2_0_1` or `v2_1`.
    #[arg(long)]
    version: String,

    /// Directory containing the version's `*.json` schema files.
    #[arg(long)]
    schemas: PathBuf,

    /// Output directory for the generated module, e.g. `src/v2_0_1`.
    #[arg(long)]
    out: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let model = v2_x::build_model(&args.version, &args.schemas)?;
    backend::write_model(&model, &args.out)?;
    eprintln!(
        "generated {} types, {} messages into {}",
        model.types.len(),
        model.messages.len(),
        args.out.display()
    );
    Ok(())
}
