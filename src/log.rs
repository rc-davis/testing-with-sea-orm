use tracing_subscriber::{prelude::*, EnvFilter};

pub fn setup_logging(verbose: bool) {
    let filter = match verbose {
        true => "debug",
        false => "testing_with_sea_orm=info,sea_orm_migration=info",
    };

    let filter_layer = EnvFilter::try_new(filter).unwrap();

    if verbose {
        let fmt_layer = tracing_subscriber::fmt::layer();
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init()
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_level(false)
            .without_time();
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init()
    };
}
