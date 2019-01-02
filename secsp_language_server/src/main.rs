extern crate crossbeam_channel;
extern crate failure;
extern crate gen_lsp_server;
extern crate languageserver_types;

use crossbeam_channel::{Receiver, Sender};
use gen_lsp_server::{handle_shutdown, run_server, stdio_transport, RawMessage, RawResponse};
use languageserver_types::{
    request::{GotoDefinition, GotoDefinitionResponse},
    InitializeParams, ServerCapabilities,
};

fn main() -> Result<(), failure::Error> {
    let (receiver, sender, io_threads) = stdio_transport();
    run_server(ServerCapabilities::default(), receiver, sender, main_loop)?;
    io_threads.join()?;
    Ok(())
}

fn main_loop(
    params: InitializeParams,
    receiver: &Receiver<RawMessage>,
    sender: &Sender<RawMessage>,
) -> Result<(), failure::Error> {
    let cwd = ::std::env::current_dir()?;
    let root = params
        .root_uri
        .and_then(|it| it.to_file_path().ok())
        .unwrap_or(cwd);

    for msg in receiver {
        match msg {
            RawMessage::Request(req) => {
                let req = match handle_shutdown(req, sender) {
                    None => return Ok(()),
                    Some(req) => req,
                };
                let req = match req.cast::<GotoDefinition>() {
                    Ok((id, _params)) => {
                        let resp = RawResponse::ok::<GotoDefinition>(
                            id,
                            &Some(GotoDefinitionResponse::Array(Vec::new())),
                        );
                        sender.send(RawMessage::Response(resp));
                        continue;
                    }
                    Err(req) => req,
                };
                // ...
            }
            RawMessage::Response(_resp) => (),
            RawMessage::Notification(_not) => (),
        }
    }
    Ok(())
}
