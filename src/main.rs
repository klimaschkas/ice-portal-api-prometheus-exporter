use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::time::Duration;
use anyhow::Result;
use metrics::gauge;
use metrics::Label;
use metrics_exporter_prometheus::PrometheusBuilder;
use reqwest::header::HOST;
use reqwest::header::{HeaderMap, ACCEPT, USER_AGENT};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use tokio::time::sleep;
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct StatusResponse {
    connection: bool,
    service_level: String,
    gps_status: String,
    internet: String,
    latitude: f64,
    longitude: f64,
    tile_y: i64,
    tile_x: i64,
    series: String,
    server_time: i64,
    speed: f64,
    train_type: String,
    tzn: String,
    wagon_class: String,
    connectivity: ConnectivityInfo
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ConnectivityInfo {
    current_state: String,
    next_state: String,
    remaining_time_seconds: i64
}

async fn get_status_response() -> Result<StatusResponse> {
    let client = reqwest::Client::new();
    
    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:137.0) Gecko/20100101 Firefox/137.0".parse()?);
    const URL: &str = "https://iceportal.de/api1/rs/status";
    
    let api_response: StatusResponse = client.get(URL).headers(header_map).send().await?.json().await?;

    return Ok(api_response)
}

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();
    let builder = PrometheusBuilder::new();

    if let Err(e) = builder.with_http_listener(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9184)).install() {
        error!("Failed to install Prometheus exporter: {}", e);
    };

    loop {
        if let Ok(status_response) = get_status_response().await {

            let labels = vec![Label::new("tzn", status_response.tzn)];

            let ice_speed = gauge!("ice_speed", labels.clone());
            let ice_latitude = gauge!("ice_latitude", labels.clone());
            let ice_longitude = gauge!("ice_longitude", labels.clone());

            ice_speed.set(status_response.speed as f64);
            ice_latitude.set(status_response.latitude);
            ice_longitude.set(status_response.longitude);
        }
        
        sleep(Duration::from_secs(5)).await;
    }
}
