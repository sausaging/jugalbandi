use env_logger::Builder;

pub fn init_logger() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .default_format()
        .init();
}
