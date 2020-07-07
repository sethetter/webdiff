use futures::future::try_join_all;
use reqwest;
// use scraper::{Html, Selector};
use std::{thread, time};
use std::default::Default;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let t1 = Target {
        uri: "https://sethetter.com".to_string(),
        interval: 2,
        ..Default::default()
    };

    let targets = vec![t1];
    let tasks: Vec<_> = targets.iter().map(|t| t.watch()).collect();

    try_join_all(tasks).await?;
    Ok(())
}

#[derive(Clone)]
struct Target {
    uri: String,
    interval: u32,
    // selector: String,
}

impl Target {
    async fn watch(&self) -> Result<()> {
        let mut last = "".to_string();
        loop {
            let (changed, next) = self.check(last.clone()).await?;

            // TODO: do something different with this
            if changed {
                println!("DIFF!");
                last = next;
            } else {
                println!("NO DIFF!");
            }

            thread::sleep(time::Duration::from_secs(self.interval as u64));
        }
    }

    async fn check(&self, last: String) -> Result<(bool, String)> {
        let body = reqwest::get(self.uri.as_str()).await?.text().await?;
        // let doc = Html::parse_document(body.as_str());
        // let select = Selector::parse(self.selector.as_str())?;

        let changed = body.to_string() != last;

        Ok((changed, body.to_string()))
    }
}

impl Default for Target {
    fn default() -> Self {
        Target {
            uri: "".to_string(),
            // selector: "".to_string(),
            interval: 30,
        }
    }
}
