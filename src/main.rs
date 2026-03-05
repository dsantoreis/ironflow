use anyhow::Context;
use clap::{Parser, ValueEnum};

use agent_data_pipeline_rust::{export_json, ingest_csv, transform, PipelineOptions};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "High-performance data pipeline CLI")]
struct Cli {
    /// Input CSV file path
    #[arg(long)]
    input: String,

    /// Output file path
    #[arg(long)]
    output: String,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// Keep only records with amount >= value
    #[arg(long)]
    min_amount: Option<f64>,

    /// Keep only records with exact category
    #[arg(long)]
    category: Option<String>,

    /// Uppercase the name field
    #[arg(long)]
    uppercase_name: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let options = PipelineOptions {
        min_amount: cli.min_amount,
        uppercase_name: cli.uppercase_name,
        category_equals: cli.category,
    };

    let records =
        ingest_csv(&cli.input).with_context(|| format!("failed to read {}", cli.input))?;
    let transformed = transform(records, &options);

    match cli.format {
        OutputFormat::Json => export_json(&cli.output, &transformed)
            .with_context(|| format!("failed to write {}", cli.output))?,
    }

    println!("pipeline complete: {} records", transformed.len());
    Ok(())
}
