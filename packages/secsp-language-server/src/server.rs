use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Connection, ErrorCode, Message, Response};
use lsp_types::InitializeParams;
use lsp_types::request::HoverRequest;
use threadpool::ThreadPool;

use secsp_analysis::AnalysisHost;

use crate::query;

pub struct Server {
    params: InitializeParams,
}

impl Server {
    pub fn new(params: InitializeParams) -> Self {
        Server { params }
    }

    pub fn run(
        self,
        connection: &Connection,
    ) -> Result<(), failure::Error> {
        let analysis_root = self.params.root_path.unwrap_or_else(|| ".".to_owned());
        let _analysis = AnalysisHost::from_workspace(analysis_root);
        let pool = ThreadPool::new(8);

        for msg in &connection.receiver {
            match msg {
                Message::Request(request) => {}
                Message::Notification(notification) => {
                    log::debug!("received unsupported notification: {:#?}", notification);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
