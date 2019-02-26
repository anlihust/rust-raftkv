use std::sync::Arc;
use std::thread;

use crossbeam_channel::{self, Sender};
use futures::{future::Future, sync::oneshot};
use grpcio::{Environment, ServerBuilder};
use grpcio::{RpcContext, UnarySink};
use log::info;
use protobuf::Message;
use raft::eraftpb::Message as RaftMessage;

use crate::rpc::raft::*;
use crate::rpc::raft_grpc::{create_raft, Raft};

use super::node::Msg;

pub struct RaftService {
    pub sender: Sender<Msg>,
    pub ip: String,
    pub port: u16,
}

impl RaftService {
    pub fn start(&self) {
        let raft_service = create_raft(RaftController {
            sender: self.sender.clone(),
        });

        let env = Arc::new(Environment::new(1));
        let mut server = ServerBuilder::new(env)
            .register_service(raft_service)
            .bind(self.ip.as_str(), self.port)
            .build()
            .unwrap();
        thread::spawn(move || {
            let (_tx, rx) = oneshot::channel::<String>();
            server.start();
            for &(ref host, port) in server.bind_addrs() {
                info!("raft listening on {}:{}", host, port);
                println!("raft service listening on {}:{}", host, port);
            }
            let _ = rx.wait();
            let _ = server.shutdown().wait();
        });
    }
}

#[derive(Clone)]
pub struct RaftController {
    sender: Sender<Msg>,
}

impl Raft for RaftController {
    fn send(&mut self, ctx: RpcContext, req: Req, sink: UnarySink<Resp>) {
        let mut m = RaftMessage::new();
        m.merge_from_bytes(req.data.as_slice()).unwrap();

        let _todo = self.sender.send(Msg::Raft(m));
        let mut resp = Resp::new();
        resp.set_code(0);
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}
