# ICE Portal API Prometheus Exporter

Exposes the API in a Deutsche Bahn ICE train in a Prometheus scrapable format.

## How it works
Every ICE exposes an API with current state about its journey if you are connected to the board wifi (e.g. WIFIonICE).

```
> GET /api1/rs/status HTTP/1.1
> Host: iceportal.de

{
   "connection":true,
   "serviceLevel":"AVAILABLE_SERVICE",
   "gpsStatus":"VALID",
   "internet":"HIGH",
   "latitude":50.1234567,
   "longitude":10.1234567,
   "tileY":100,
   "tileX":30,
   "series":"415",
   "serverTime":1746270031172,
   "speed":112.1,
   "trainType":"ICE",
   "tzn":"ICE1234",
   "wagonClass":"SECOND",
   "connectivity":{
      "currentState":"HIGH",
      "nextState":"UNSTABLE",
      "remainingTimeSeconds":5600
   },
   "bapInstalled":true
}
```

This application queries the API and hosts a page at 0.0.0.0:9184/metrics in a format that can be scraped with Prometheus.

## Features
The speed, latitude and longitude of the ICE are published. The current train is labeled with the TZN (e.g. ICE1234).