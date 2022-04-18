extern crate notify;
use serde_yaml;
use notify::{Watcher, RecursiveMode, watcher};
use std::{sync::{ Arc, Mutex, mpsc::channel}, fs, time::Duration};
use log::{info, warn};

use crate::structure::Config;

fn read(config: &str, state: &Arc<Mutex<Config>>) {

    let file_contents = fs::read_to_string(config)
        .expect("Something went wrong reading the configuration file");

    let data = serde_yaml::from_str::<Config>(&file_contents);

    assert!(data.is_ok());

    if let Ok(mut s) = state.lock() {
        info!("Writing configuration file to data");
        *s = data.unwrap();
    }
}

pub fn run(config: &str, state: Arc<Mutex<Config>>) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();
    watcher.watch(config, RecursiveMode::Recursive).unwrap();

    /*
    Trigger initial run of function to make everything available as
    soon as the program starts
    */
    read(config, &state);

    loop {
        match rx.recv() {
            Ok(event) => {
                info!("Received event: {:?}", event);
                read(config, &state);
            },
            Err(error) => warn!("Received error: {:?}", error),
        }
    }
}
