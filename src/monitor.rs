use reqwest::Url;

use crate::{
    config::Site,
    product::{Product, Products},
};

pub struct Monitor {
    site: Site,
    proxies: Option<Vec<String>>,
    client: reqwest::Client,
    products: Vec<Product>,
}

impl Monitor {
    pub fn new(site: Site, proxies: Option<Vec<String>>) -> Self {
        Self {
            site,
            proxies,
            client: reqwest::Client::new(),
            products: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        println!("Monitoring {}", self.site.url);
        let url = Url::parse(&self.site.url).unwrap();
        let products_url = format!(
            "{}://{}/products.json",
            url.scheme(),
            url.host_str().unwrap()
        );

        loop {
            self.get_products(products_url.clone()).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    async fn get_products(&mut self, url: String) {
        let res = self.client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.84 Safari/537.36 OPR/85.0.4341.72")
            .send()
            .await;

        let products = match res {
            Ok(res) => res.json::<Products>().await.unwrap(),
            Err(err) => {
                println!("{:?}", err);
                return;
            }
        };

        for product in products.products {
            if self.products.contains(&product) {
                println!("Product already found: {}", product.title);
                continue;
            } else {
                println!("Product found: {}", product.title);
                self.products.push(product);
            }
        }
    }
}
