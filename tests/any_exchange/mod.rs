use openlimits::any_exchange::{AnyExchange, AnyExchangeWs};
use openlimits::nash::{Nash, NashParameters, NashCredentials, NashStream};
use openlimits::exchange::OpenLimits;
use std::env;
use dotenv::dotenv;
use nash_native_client::ws_client::client::Environment;
use openlimits::binance::{Binance, BinanceParameters, BinanceCredentials};
use openlimits::coinbase::{CoinbaseParameters, Coinbase, CoinbaseCredentials};
use openlimits::exchange_ws::OpenLimitsWs;
use openlimits::coinbase::client::websocket::CoinbaseWebsocket;

#[tokio::test]
async fn account_test() {

}

async fn nash() -> Nash {
    let parameters = NashParameters {
        affiliate_code: None,
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
        client_id: 1,
        timeout: 1000,
    };
    OpenLimits::instantiate(parameters).await
}

async fn binance() -> Binance {
    let parameters = BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable."),
        }),
        sandbox: true
    };
    OpenLimits::instantiate(parameters).await
}

async fn coinbase() -> Coinbase {
    let parameters = CoinbaseParameters {
        sandbox: true,
        credentials: Some(CoinbaseCredentials {
            api_key: env::var("COINBASE_API_KEY").unwrap(),
            api_secret: env::var("COINBASE_API_SECRET").unwrap(),
            passphrase: env::var("COINBASE_PASSPHRASE").unwrap(),
        }),
    };
    OpenLimits::instantiate(parameters).await
}

async fn init() -> impl AnyExchange {
    dotenv().ok();
    coinbase().await
}

async fn init_ws() -> OpenLimitsWs<CoinbaseWebsocket> {
    dotenv().ok();

    let websocket = NashStream::with_credential(
        &env::var("NASH_API_SECRET").unwrap(),
        &env::var("NASH_API_KEY").unwrap(),
        1234,
        true,
        10000,
    ).await;

    OpenLimitsWs { websocket }
}