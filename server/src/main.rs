use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use log::info;
use tonic::{transport::Server, Request, Response, Status};

use twenty38::{
    twenty38_server::{Twenty38, Twenty38Server},
    StatsRequest, StatsResponse,
};

const LONG_ABOUT: &str = "2038 Server to scan your infra!";

pub mod twenty38 {
    // https://github.com/hyperium/tonic/issues/1056
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("twenty38");
}

#[derive(Debug, Default)]
pub struct Twenty38Service {}

#[tonic::async_trait]
impl Twenty38 for Twenty38Service {
    async fn stats(
        &self,
        request: Request<StatsRequest>,
    ) -> Result<Response<StatsResponse>, Status> {
        let r = request.into_inner();
        info!("Received a stats request: {:?}", r);
        Ok(Response::new(twenty38::StatsResponse {
            stats: HashMap::from([("Cooper".to_string(), 69.0)]),
        }))
    }
}

/// Clap CLI Args struct with metadata in help output
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Address(es) to bind to
    #[clap(short, long, value_parser, default_value = "[::]:1469")]
    bind_uri: String,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let socket_addr: SocketAddr = args
        .bind_uri
        .parse()
        .expect("Unable to parse socket address");
    info!("Starting 2038 server @ {} ...", socket_addr);

    let twenty38_service = Twenty38Service::default();

    Server::builder()
        .add_service(Twenty38Server::new(twenty38_service))
        .serve(socket_addr)
        .await?;

    Ok(())
}
