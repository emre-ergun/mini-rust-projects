use env_logger;
use log::{info, warn, error};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("Oops, nothing implemented");
    error!("something went wrong!");
}
