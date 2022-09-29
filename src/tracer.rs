use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{format::FmtSpan, layer},
    prelude::*,
    EnvFilter, Registry,
};

pub fn init() {
    LogTracer::init().unwrap();

    let layer = layer()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_file(false)
        .boxed();

    let registry = Registry::default()
        .with(EnvFilter::from("debug,tower_http=error,hyper=error"))
        .with(layer);

    tracing::subscriber::set_global_default(registry).unwrap();
}
