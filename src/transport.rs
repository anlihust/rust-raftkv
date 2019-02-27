use std::collections::HashMap;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use protobuf::Message;
use raft::eraftpb::{Message as RaftMessage, MessageType};
use raft::SnapshotStatus;

use grpcio::{ChannelBuilder, EnvBuilder};

use super::Addr;
use crate::node::*;
use std::sync::Arc;

use crate::rpc::raft::Req;
use crate::rpc::raft_grpc::RaftClient;

pub struct Transport {
    sender: Sender<Msg>,
    node_chs: HashMap<u64, Sender<RaftMessage>>,
    id: u64,
}

impl Transport {
    pub fn new(sender: Sender<Msg>, id: u64) -> Transport {
        Transport {
            sender: sender,
            node_chs: HashMap::new(),
            id,
        }
    }

    pub fn start(&mut self, node_addrs: HashMap<u64, Addr>) {
        for (id, addr) in node_addrs.iter() {
            //skip self
            if self.id.eq(id) {
                continue;
            }
            let (s, r) = unbounded();
            self.node_chs.insert(*id, s);

            let id = *id;
            let addr = addr.raft_addr.clone();
            let sender = self.sender.clone();
            thread::spawn(move || {
                on_transport(r, id, addr, sender);
            });
        }
    }
    pub fn add_peer(&mut self, id: u64, addr: &str) {
        //skip self
        if id == self.id {
            return;
        }
        let (s, r) = unbounded();
        self.node_chs.insert(id, s);

        let sender = self.sender.clone();
        let addr = addr.to_string();
        thread::spawn(move || {
            on_transport(r, id, addr, sender);
        });
    }
    pub fn remove_peer(&mut self, id: u64) {
        self.node_chs.remove(&id);
    }
    pub fn remove_all(&mut self) {
        self.node_chs.clear();
    }

    pub fn send(&self, id: u64, msg: RaftMessage) {
        if let Some(s) = self.node_chs.get(&id) {
            s.send(msg).unwrap();
        }
    }
}

fn on_transport(ch: Receiver<RaftMessage>, id: u64, addr: String, sender: Sender<Msg>) {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect(&addr);
    let client = RaftClient::new(channel);
    while let Ok(msg) = ch.recv() {
        let value = msg.write_to_bytes().unwrap();
        let mut req = Req::new();
        req.set_data(value);
        let is_snapshot = msg.get_msg_type() == MessageType::MsgSnapshot;
        if let Err(_) = client.send(&req) {
            sender
                .send(Msg::ReportSnapshot {
                    id: id,
                    status: SnapshotStatus::Failure,
                })
                .unwrap();
            sender.send(Msg::ReportUnreachable(id)).unwrap();
        }

        if is_snapshot {
            sender
                .send(Msg::ReportSnapshot {
                    id: id,
                    status: SnapshotStatus::Finish,
                })
                .unwrap();
        }
    }
}
