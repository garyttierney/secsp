use crossbeam_channel::{Receiver, Sender};
use gen_lsp_server::{handle_shutdown, ErrorCode, RawMessage, RawResponse};
use lsp_types::request::{GotoDefinition, HoverRequest};
use lsp_types::InitializeParams;
use threadpool::ThreadPool;

use crate::query;
use crate::server::dispatcher::PoolDispatcher;

mod dispatcher;
mod handler;

pub struct Server {}

impl Server {
    pub fn new(params: InitializeParams) -> Self {
        Server {}
    }

    pub fn run(
        self,
        receiver: &Receiver<RawMessage>,
        sender: &Sender<RawMessage>,
    ) -> Result<(), failure::Error> {
        let pool = ThreadPool::new(8);

        for msg in receiver {
            match msg {
                RawMessage::Request(request) => {
                    let mut dispatcher = PoolDispatcher {
                        pool: &pool,
                        sender: &sender,
                        req: Some(request),
                        res: None,
                    };

                    let result = dispatcher
                        .on::<HoverRequest>(query::hover_request)?
                        .finish();

                    if let Err(request) = result {
                        let resp = RawResponse::err(
                            request.id,
                            ErrorCode::MethodNotFound as i32,
                            "unknown request".to_string(),
                        );
                        sender.send(resp.into()).unwrap()
                    }
                }
                RawMessage::Notification(notification) => {
                    log::debug!("received unsupported notification: {:#?}", notification);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
