use tracing_subscriber::EnvFilter;

pub fn init_tracing(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
