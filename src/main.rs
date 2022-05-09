// use std::thread::JoinHandle;
use tokio::task::JoinHandle;

use monitor::Monitor;

mod product;
mod config;
mod monitor;

#[tokio::main]
async fn main() {
    let sites = config::read_sites().unwrap();
    let proxies = config::read_proxies().unwrap();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for site in sites {
        let handle = tokio::spawn(async move {
            let mut monitor = Monitor::new(site, None);
            monitor.run().await;
        });

        handles.push(handle);
    }

    for task in handles {
        task.await.unwrap();
    }

    println!("Done!");
}
