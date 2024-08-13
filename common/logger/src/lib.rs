use std::env;
use std::str::FromStr;

pub fn logger_init(level: String) {
    if env::var_os("RUST_LOG").is_none() {
        let env = format!("rustapi={level},tower_http={level}");
        env::set_var("RUST_LOG", env);
    }

    let level = tracing::Level::from_str(level.as_str()).expect("An error occurred when setting the level to tracing ");
    tracing_subscriber::fmt().with_max_level(level).init();
}
