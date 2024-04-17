use std::process;

use anyhow::Result;
use futures::future::BoxFuture;
use rdev::{listen, Event};
use tokio::sync::mpsc;

type Hook = fn() -> BoxFuture<'static, Result<()>>;

#[derive(Debug, Clone, Copy)]
pub struct HotKey {}

#[derive(Clone)]
pub struct Listener {
    history: Vec<Event>,
    hooks: Vec<(Vec<HotKey>, Hook)>,
}

pub async fn hook(history: Vec<Event>, hooks: Vec<(Vec<HotKey>, Hook)>) {
    for (hot_key, hook) in hooks {

    }
}

impl Listener {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            hooks: Vec::new(),
        }
    }

    pub fn register(&mut self, route: Vec<HotKey>, f: Hook) {
        self.hooks.push((route, f));
    }

    pub async fn listen(&mut self) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            match listen(move |event| {
                tx.send(event)
                    .unwrap_or_else(|e| eprintln!("Could not send event {:?}", e));
            }) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{:?}", e);
                    process::exit(0);
                }
            }
        });

        loop {
            if let Some(event) = rx.recv().await {
                self.history.push(event);
                tokio::spawn(hook(self.history.clone(), self.hooks.clone()));
            }
        }
    }
}
