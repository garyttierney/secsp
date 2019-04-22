extern crate crossbeam_channel;
extern crate env_logger;
extern crate failure;
extern crate gen_lsp_server;
extern crate log;
extern crate lsp_types;
extern crate secsp_analysis;
extern crate secsp_language_server;

use std::io::BufReader;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::thread;

use crossbeam_channel::{bounded, Receiver, Sender};
use env_logger::Target;
use gen_lsp_server::{run_server, stdio_transport, RawMessage, RawResponse, Threads};
use log::debug;
use lsp_types::notification::Exit;
use lsp_types::{InitializeParams, ServerCapabilities};

use secsp_language_server::server::Server;

pub mod query;

fn main() -> Result<(), failure::Error> {
    env_logger::builder().target(Target::Stderr).init();
    let (receiver, sender, io_threads) = stdio_transport();
    let mut capabilities = ServerCapabilities::default();
    capabilities.hover_provider = Some(true);
    gen_lsp_server::run_server(capabilities, receiver, sender, main_loop)?;
    io_threads.join()?;
    Ok(())
}

fn main_loop(
    params: InitializeParams,
    receiver: &Receiver<RawMessage>,
    sender: &Sender<RawMessage>,
) -> Result<(), failure::Error> {
    let server = Server::new(params);
    server.run(&receiver, &sender)
}
