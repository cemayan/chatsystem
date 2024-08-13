use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ws {
    pub addr: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadApi {
    pub addr: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WriteApi {
    pub addr: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Logger {
    pub level: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub ws: Option<Ws>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<ReadApi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<WriteApi>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Configs {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
}


impl Configs {
    pub fn new(path: Option<&String>) -> Result<Self, ConfigError> {
        match path {
            None => {
                let msg = "path not found";
                Err(ConfigError::Message(msg.to_string()))
            }
            Some(p) => {
                let builder = Config::builder()
                    .add_source(File::with_name(p));
                builder.build()?.try_deserialize()
            }
        }
    }
}



