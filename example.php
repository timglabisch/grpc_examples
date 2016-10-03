<?php

use hello\WeatherClient;

require __DIR__.'/vendor/autoload.php';
require __DIR__.'/hello.php';

$weatherClient = new WeatherClient('localhost:50051', [
    'credentials' => \Grpc\ChannelCredentials::createInsecure(),
]);

while (true) {

    $response = $weatherClient->getWeather(
        (new \hello\WeatherRequest())->setLocation(
            "fooooooo"
        )
    );

    /** @var $res \hello\WeatherResponse */
    list($res, $metadata) = $response->wait();

    if($res->getTemperature()) {
        echo ".";
    }
}