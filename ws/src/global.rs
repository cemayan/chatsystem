use clap::Parser;
use config::Configs;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref HASHMAP: HashMap<&'static str, String> = {
        let args = Args::parse();
        let mut m = HashMap::new();
        m.insert("configPath",args.config);
        m
    };
}


lazy_static! {
  pub static ref CONFIGS: Configs = Configs::new(HASHMAP.get("configPath").to_owned()).expect("an error occurred when getting configs");
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    pub(crate) config: String,
}
