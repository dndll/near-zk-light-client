use config::{Config as ConfigTrait, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

use crate::client::rpc::Network;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub debug: bool,
    pub starting_head: String,
    pub network: Network,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("NEAR_LIGHT_CLIENT_NETWORK").unwrap_or_else(|_| "testnet".into());

        let s = ConfigTrait::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("default"))
            .add_source(File::with_name(&format!("{}", run_mode)).required(false))
            // This file shouldn't be checked in to git
            .add_source(File::with_name("local").required(false))
            // Eg.. `RELAYER_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("RELAYER"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        let r = s.try_deserialize();

        log::debug!("Config: {:#?}", r);
        r
    }
}
