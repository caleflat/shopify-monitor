use std::{fs, io::{self, BufRead}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub url: String,
}

pub fn read_sites() -> Result<Vec<Site>, String> {
    let file = match fs::File::open("./config/sites.json") {
        Ok(file) => file,
        Err(err) => return Err(format!("{:?}", err)),
    };

    let sites: Vec<Site> = match serde_json::from_reader(file) {
        Ok(sites) => sites,
        Err(err) => return Err(format!("{:?}", err)),
    };

    Ok(sites)
}

pub fn read_proxies() -> Result<Vec<String>, String> {
    let file = match fs::File::open("./config/proxies.txt") {
        Ok(file) => file,
        Err(err) => return Err(format!("{:?}", err)),
    };

    let proxies: Result<Vec<String>, io::Error> = io::BufReader::new(file).lines().collect();
    if proxies.is_err() {
        return Err(format!("{:?}", proxies.err().unwrap()));
    }

    Ok(proxies.unwrap())
}