use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use log::info;

pub mod twenty38 {
    // https://github.com/hyperium/tonic/issues/1056
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("twenty38");
}

use twenty38::{twenty38_client::Twenty38Client, StatsRequest};

const LONG_ABOUT: &str = "2038 Service CLI - Do all the things!";

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Address(es) to bing to
    #[clap(short, long, value_parser, default_value = "http://[::1]:1469")]
    service_url: String,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Attempting to connect to {}", args.service_url);
    let mut client = Twenty38Client::connect(args.service_url).await?;
    let request = tonic::Request::new(StatsRequest {
        stats: vec![String::from("")],
    });
    let response = client.stats(request).await?;
    println!(
        "Got: '{:?}' from Twenty38 service",
        response.into_inner().stats
    );

    Ok(())
}
