// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait ServerLove {
    fn love(&self, o: ::grpc::RequestOptions, p: super::love::LoveYou) -> ::grpc::StreamingResponse<super::love::LoveReply>;
}

// client

pub struct ServerLoveClient {
    grpc_client: ::grpc::Client,
    method_Love: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::love::LoveYou, super::love::LoveReply>>,
}

impl ServerLoveClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ServerLoveClient {
            grpc_client: grpc_client,
            method_Love: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/love.ServerLove/Love".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ServerLoveClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ServerLoveClient::with_client(c)
        })
    }
}

impl ServerLove for ServerLoveClient {
    fn love(&self, o: ::grpc::RequestOptions, p: super::love::LoveYou) -> ::grpc::StreamingResponse<super::love::LoveReply> {
        self.grpc_client.call_server_streaming(o, p, self.method_Love.clone())
    }
}

// server

pub struct ServerLoveServer;


impl ServerLoveServer {
    pub fn new_service_def<H : ServerLove + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/love.ServerLove",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/love.ServerLove/Love".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.love(o, p))
                    },
                ),
            ],
        )
    }
}
