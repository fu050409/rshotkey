use std::{process, time::Duration};

use anyhow::Result;
use futures::{future::BoxFuture, TryFutureExt};
use rdev::{listen, Event, EventType};
use tokio::sync::mpsc;

pub type HookResult = BoxFuture<'static, Result<()>>;
pub type Hook = fn() -> HookResult;

#[derive(Debug, Clone)]
pub struct HotKey {
    pub key: Vec<EventType>,
    pub delay_time: Duration,
}

impl HotKey {
    pub fn new(key: Vec<EventType>, delay_time: Duration) -> Self {
        Self { key, delay_time }
    }
}

#[derive(Clone)]
pub struct Listener {
    history: Vec<Event>,
    hooks: Vec<(Vec<HotKey>, Hook)>,
}

pub async fn hook(history: Vec<Event>, hooks: Vec<(Vec<HotKey>, Hook)>) -> Result<()> {
    for (hot_keys, hook) in hooks {
        let mut checked = true;
        for hot_key in hot_keys {
            let mut checks = Vec::new();
            let mut keys = hot_key.key.clone();
            if history.len() < hot_key.key.len() {
                checked = false;
                break;
            }
            history[history.len() - hot_key.key.len()..].clone_into(&mut checks);
            let mut matched = true;
            while !checks.is_empty() {
                let mut found = false;
                let mut check_idx = 0;
                while check_idx < checks.len() {
                    let mut key_idx = 0;
                    while key_idx < keys.len() {
                        if checks[check_idx].event_type == keys[key_idx] {
                            found = true;
                            checks.remove(check_idx);
                            keys.remove(key_idx);
                            break;
                        }
                        key_idx += 1;
                    }
                    if found {
                        break;
                    }
                    check_idx += 1;
                }
                if !found {
                    matched = false;
                    break;
                }
            }
            if !matched {
                checked = false;
                break;
            }
        }
        if checked {
            hook().await?;
        }
    }
    Ok(())
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
                tokio::spawn(
                    hook(self.history.clone(), self.hooks.clone())
                        .unwrap_or_else(|e| eprint!("{}", e)),
                );
            }
        }
    }
}
