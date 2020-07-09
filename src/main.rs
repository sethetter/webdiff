use futures::future::try_join_all;
use simple_error::SimpleError;
use scraper::{Html, Selector};
use tokio::time::delay_for;
use std::{time, env, default::Default};
use serde::{Serialize, Deserialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let config_path = args.get(1).unwrap();
    let config = parse_config(config_path)?;

    let tasks: Vec<_> = config.targets.iter().map(|t| t.watch()).collect();
    try_join_all(tasks).await?;

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    targets: Vec<Target>,
}

fn parse_config(fname: &String) -> Result<Config> {
    let fcontents = std::fs::read_to_string(fname)?;
    let config: Config = serde_json::from_str(fcontents.as_str()).map_err(|_| {
        SimpleError::new("failed to parse config")
    })?;
    Ok(config)
}

#[derive(Clone, Serialize, Deserialize)]
struct Target {
    uri: String,
    interval: u32,
    selector: Option<String>,
}

impl Target {
    async fn watch(&self) -> Result<()> {
        let mut last = vec!["".to_string()];
        loop {
            let (changed, next) = self.check(last.clone()).await?;

            if changed {
                let event_str = serde_json::to_string(
                    &self.change_event(last, next.clone())
                ).map_err(|_| {
                    SimpleError::new("failed to serialize change event")
                })?;
                println!("{}", event_str);
            }

            last = next;
            delay_for(time::Duration::from_secs(self.interval as u64)).await;
        }
    }

    async fn check(&self, last: Vec<String>) -> Result<(bool, Vec<String>)> {
        let body = reqwest::get(self.uri.as_str()).await?.text().await?;
        match &self.selector {
            Some(selector) => {
                let doc = Html::parse_document(body.as_str());
                let select = Selector::parse(selector.as_str()).map_err(|_| {
                    SimpleError::new("failed to parse selector")
                })?;

                let matches: Vec<String> = doc.select(&select).map(|el| {
                    el.inner_html()
                }).collect();

                let changed = matches != last;
                Ok((changed, matches))
            },
            None => {
                let new = vec![body.to_string()];
                let changed = new != last;
                Ok((changed, new))
            },
        }
    }

    fn change_event(&self, old: Vec<String>, new: Vec<String>) -> ChangeEvent {
        ChangeEvent { target: self.clone(), old, new }
    }
}

impl Default for Target {
    fn default() -> Self {
        Target {
            uri: "".to_string(),
            selector: None,
            interval: 30,
        }
    }
}

#[derive(Serialize)]
struct ChangeEvent {
    target: Target,
    old: Vec<String>,
    new: Vec<String>,
}
