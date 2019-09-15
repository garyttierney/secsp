extern crate crossbeam_channel;
extern crate env_logger;
extern crate failure;
extern crate lsp_server;
extern crate log;
extern crate lsp_types;
extern crate secsp_analysis;
extern crate secsp_language_server;

use crossbeam_channel::{Receiver, Sender};
use env_logger::Target;
use lsp_server::{Message, Connection};
use lsp_types::{InitializeParams, ServerCapabilities};

use secsp_language_server::server::Server;

pub mod query;

fn main() -> Result<(), failure::Error> {
    env_logger::builder().target(Target::Stderr).init();
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities::default()).unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;

    main_loop(serde_json::from_value(initialization_params).unwrap(), &connection)?;
    io_threads.join()?;
    Ok(())
}

fn main_loop(
    params: InitializeParams,
    connection: &Connection
) -> Result<(), failure::Error> {
    let server = Server::new(params);
    server.run(connection)
}
