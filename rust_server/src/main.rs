extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
mod hello;
mod hello_grpc;
use grpc::result::GrpcResult;
use std::thread;

struct WeatherServiceImpl;

impl hello_grpc::Weather for WeatherServiceImpl {
    fn getWeather(&self, req: hello::WeatherRequest) -> GrpcResult<hello::WeatherResponse> {
        let mut r = hello::WeatherResponse::new();
        r.set_temperature(10);
        Ok(r)
    }
}

fn main() {
    println!("Hello, world!");

    let server = hello_grpc::WeatherServer::new(50051, WeatherServiceImpl);


    loop {
        thread::park();
    }
}
