// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Weather {
    fn getWeather(&self, p: super::hello::WeatherRequest) -> ::grpc::result::GrpcResult<super::hello::WeatherResponse>;
}

pub trait WeatherAsync {
    fn getWeather(&self, p: super::hello::WeatherRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::hello::WeatherResponse>;
}

// sync client

pub struct WeatherClient {
    async_client: WeatherAsyncClient,
}

impl WeatherClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        WeatherAsyncClient::new(host, port, tls).map(|c| {
            WeatherClient {
                async_client: c,
            }
        })
    }
}

impl Weather for WeatherClient {
    fn getWeather(&self, p: super::hello::WeatherRequest) -> ::grpc::result::GrpcResult<super::hello::WeatherResponse> {
        ::futures::Future::wait(self.async_client.getWeather(p))
    }
}

// async client

pub struct WeatherAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_getWeather: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::hello::WeatherRequest, super::hello::WeatherResponse>>,
}

impl WeatherAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            WeatherAsyncClient {
                grpc_client: c,
                method_getWeather: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/hello.Weather/getWeather".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl WeatherAsync for WeatherAsyncClient {
    fn getWeather(&self, p: super::hello::WeatherRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::hello::WeatherResponse> {
        self.grpc_client.call_unary(p, self.method_getWeather.clone())
    }
}

// sync server

pub struct WeatherServer {
    async_server: WeatherAsyncServer,
}

struct WeatherServerHandlerToAsync {
    handler: ::std::sync::Arc<Weather + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl WeatherAsync for WeatherServerHandlerToAsync {
    fn getWeather(&self, p: super::hello::WeatherRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::hello::WeatherResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.getWeather(p)
        })
    }
}

impl WeatherServer {
    pub fn new<H : Weather + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = WeatherServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        WeatherServer {
            async_server: WeatherAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct WeatherAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl WeatherAsyncServer {
    pub fn new<H : WeatherAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/hello.Weather/getWeather".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.getWeather(p))
                    },
                ),
            ],
        );
        WeatherAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
