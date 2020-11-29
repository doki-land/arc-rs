use crate::{Result,Value};

#[cfg(feature = "json")]
pub fn parse_json(json: &str) -> Result<Value>{
   let data =  serde_json::from_str::<serde_json::Value>(json);
    Ok(data?.into())
}

#[cfg(feature = "toml")]
pub fn parse_toml(toml: &str) -> Result<Value> {
    let data =toml::from_str::<toml::Value>(toml);
    Ok(data?.into())
}

#[cfg(feature = "yaml")]
pub fn parse_yaml(toml: &str) -> Result<Value> {
    let data =toml::from_str::<toml::Value>(toml);
    Ok(data?.into())
}