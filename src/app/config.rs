use config::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    CONFIG.get::<T>(key).unwrap()
}
