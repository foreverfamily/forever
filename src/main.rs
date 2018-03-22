extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate hyper;
extern crate protobuf;
extern crate tls_api;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{const_service, Http, Response, service_fn};
use protobuf::Message;
use std::thread;

pub mod proto;

static TEXT: &'static str = "i love you, 慧慧";

struct Love;

impl proto::ServerLove for Love {
    fn love(&self, o: grpc::RequestOptions, love_you: proto::LoveYou) -> grpc::StreamingResponse<proto::LoveReply> {
        let mut r = proto::LoveReply::new();
        r.set_message(TEXT.to_string());
        println!("req name = {:?}", love_you.get_name());
        grpc::StreamingResponse::completed(vec![r])
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(3000);
    server.add_service(proto::ServerLoveServer::new_service_def(Love {}));

    server.http.set_cpu_pool_threads(4);
    server.build().expect("server");
    loop {
        thread::park();
    }
}