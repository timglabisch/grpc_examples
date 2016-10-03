var PROTO_PATH = __dirname + '/../hello.proto';

var grpc = require('grpc');
var hello_proto = grpc.load(PROTO_PATH).hello;

/**
 * Implements the SayHello RPC method.
 */
function getWeather(call, callback) {
    callback(null, {temperature: 10});
}

/**
 * Starts an RPC server that receives requests for the Greeter service at the
 * sample server port
 */
function main() {
    var server = new grpc.Server();
    server.addProtoService(hello_proto.Weather.service, {getWeather: getWeather});
    server.bind('0.0.0.0:50051', grpc.ServerCredentials.createInsecure());
    server.start();
}

main();