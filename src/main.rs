use std::{env::var, thread, sync::{Arc, Mutex}};
use log::info;
use lazy_static::lazy_static;

mod notifier;
mod webserver;
mod structure;
use structure::Config;

lazy_static! {
    static ref CONFIG_FILE: String = var("CONFIG_FILE").unwrap();
    static ref TEMPLATE_FILE: String = var("TEMPLATE_FILE").unwrap();
    static ref ADDRESS: String = var("ADDRESS").unwrap();
}

fn main() {
    env_logger::init();

    /*
    Define variable for sharing data across threads.
    This is being handled by wrapping the given data across
    a mutex (so that data can be shared across threads), then
    wrapping everything inside of an Arc to add reference counting
    (so that the value is dropped once it's no longer needed)
    */
    let state: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));

    info!("Starting...");
        
    let notifier_state = state.clone();
    let notifier_handle = thread::spawn(move || {
        notifier::run(&CONFIG_FILE, notifier_state);
    });

    let webserver_state = state.clone();
    let webserver_handle = thread::spawn(move || {
        webserver::run(&ADDRESS, &TEMPLATE_FILE, webserver_state);
    });
    
    notifier_handle.join().unwrap();
    webserver_handle.join().unwrap();
}