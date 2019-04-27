use crossbeam_channel::Sender;
use gen_lsp_server::{RawMessage, RawRequest, RawResponse};
use lsp_types::request::Request;
use threadpool::ThreadPool;

pub struct PoolDispatcher<'a> {
    pub sender: &'a Sender<RawMessage>,
    pub req: Option<RawRequest>,
    pub res: Option<u64>,
    pub pool: &'a ThreadPool,
}

impl<'a> PoolDispatcher<'a> {
    pub fn on<R>(
        &mut self,
        f: fn(R::Params) -> Result<R::Result, ()>,
    ) -> Result<&mut Self, failure::Error>
    where
        R: Request,
        R::Params: serde::de::DeserializeOwned + Send + 'static,
        R::Result: serde::ser::Serialize + 'static,
    {
        let req = match self.req.take() {
            None => return Ok(self),
            Some(req) => req,
        };
        match req.cast::<R>() {
            Ok((id, params)) => {
                let sender = self.sender.clone();
                self.pool.execute(move || {
                    let resp = match f(params) {
                        Ok(resp) => RawResponse::ok::<R>(id, &resp),
                        _ => unimplemented!(),
                    };

                    sender.send(RawMessage::Response(resp)).unwrap();
                });
                self.res = Some(id);
            }
            Err(req) => self.req = Some(req),
        }
        Ok(self)
    }

    pub fn finish(&mut self) -> ::std::result::Result<u64, RawRequest> {
        match (self.res.take(), self.req.take()) {
            (Some(res), None) => Ok(res),
            (None, Some(req)) => Err(req),
            _ => unreachable!(),
        }
    }
}
