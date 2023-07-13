use std::fs;
use serde_derive::Deserialize;
use toml;


#[derive(Debug)]
pub struct Config
{
    pub(crate) delay :i32
}
#[derive(Debug, Deserialize)]
pub struct ConfigScheme
{
    general : General
}
#[derive(Debug,Deserialize)]
struct General
{
    delay : i32
}

pub fn main() -> Config
{
    let configed = load_configs(init_configs());
    configed
}

pub fn init_configs() -> ConfigScheme
{
    let binding = fs::read_to_string("config.ini").unwrap();
    let config_rw = binding.as_str();
    let config : ConfigScheme = toml::from_str(config_rw).unwrap();
    config
}

pub fn load_configs(config : ConfigScheme) -> Config
{
    let conf = Config
    {
        delay : config.general.delay,
    };
    conf
}