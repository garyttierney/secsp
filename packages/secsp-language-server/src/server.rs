use crossbeam_channel::{Receiver, Sender};
use gen_lsp_server::{ErrorCode, RawMessage, RawResponse};
use lsp_types::request::HoverRequest;
use lsp_types::InitializeParams;
use threadpool::ThreadPool;

use secsp_analysis::AnalysisHost;

use crate::query;
use crate::server::dispatcher::PoolDispatcher;

mod dispatcher;

pub struct Server {
    params: InitializeParams,
}

impl Server {
    pub fn new(params: InitializeParams) -> Self {
        Server { params }
    }

    pub fn run(
        self,
        receiver: &Receiver<RawMessage>,
        sender: &Sender<RawMessage>,
    ) -> Result<(), failure::Error> {
        let analysis_root = self.params.root_path.unwrap_or_else(|| ".".to_owned());
        let _analysis = AnalysisHost::from_workspace(analysis_root);
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
