use log::{info, trace, warn, error, debug};
use env_logger::Builder;

fn main() {
    Builder::from_env(env_logger::Env::default().default_filter_or("info,rust_dns_proto=error")).init();

    info!("Message with info level");
    error!("Message with error level");
    debug!("Message with debug level");
    trace!("Message with trace level");
    warn!("Message with warn level");
}
