use config::Config;
use once_cell::sync::Lazy;

pub static SETTINGS: Lazy<Config> = Lazy::new(setup);

fn setup() -> Config {
    let builder = Config::builder().add_source(config::File::with_name("config"));
    #[cfg(debug_assertions)]
    let builder = builder.add_source(config::File::with_name("config.dev").required(false));

    #[cfg(not(debug_assertions))]
    let builder = builder.add_source(config::File::with_name("config.prod"));

    builder
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap()
}
