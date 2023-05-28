use tracing::Level;
use tracing_subscriber::filter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_logging() {
    let filter = filter::Targets::new()
        // Enable the `INFO` level for anything in `my_crate`
        .with_target("wgpu", Level::WARN)
        .with_target("taconite", Level::INFO);

    // Build a new subscriber with a custom format without timestamps
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .without_time();

    // Build a new subscriber with the `fmt` layer using the `Targets`
    // filter we constructed above.
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter)
        .init();
}
