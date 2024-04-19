use std::process;

use anyhow::Result;
use futures::{future::BoxFuture, TryFutureExt};
use rdev::{listen, Event};
use tokio::sync::mpsc;

use crate::key::KeySet;

pub type HookResult = BoxFuture<'static, Result<()>>;
pub type Hook = fn() -> HookResult;

#[derive(Clone, Debug)]
pub struct Listener {
    max_history: usize,
    history: Vec<Event>,
    hooks: Vec<(KeySet, Hook)>,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            max_history: 512,
            history: Default::default(),
            hooks: Default::default(),
        }
    }
}

pub async fn hook_keyset(mut key_set: KeySet, history: &Vec<Event>, hook: Hook) -> Result<()> {
    let mut matched = true;
    let key_set_length = key_set.len();
    if key_set_length > history.len() {
        return Ok(());
    }

    let mut history_to_match = history[history.len() - key_set_length..].to_vec();

    for mut bind_key in key_set {
        let to_idx = bind_key.len();
        let mut checks = history_to_match[..to_idx].to_vec();
        history_to_match = history_to_match[to_idx..].to_vec();

        let last_check = if checks.is_empty() {
            matched = false;
            break;
        } else {
            checks.last().unwrap().to_owned()
        };
        while !checks.is_empty() {
            let mut found = false;
            let mut check_idx = 0;
            while check_idx < checks.len() {
                let mut key_idx = 0;
                while key_idx < bind_key.len() {
                    if checks[check_idx].event_type == bind_key.keys[key_idx] {
                        found = true;
                        checks.remove(check_idx);
                        bind_key.keys.remove(key_idx);
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

        if !bind_key.delay_time.is_zero() {
            let next_time = history_to_match[0].time;
            let duration = next_time.duration_since(last_check.time)?;
            if duration > bind_key.delay_time {
                matched = false;
                break;
            }
        }
    }
    if matched {
        hook().await?;
    };
    Ok(())
}

pub async fn hook(history: Vec<Event>, hooks: Vec<(KeySet, Hook)>) -> Result<()> {
    for (key_set, hook) in hooks {
        hook_keyset(key_set, &history, hook).await?;
    }
    Ok(())
}

impl Listener {
    pub fn new(max_history: usize, history: Vec<Event>, hooks: Vec<(KeySet, Hook)>) -> Self {
        Self {
            history,
            hooks,
            max_history,
        }
    }

    pub fn register(&mut self, key_set: KeySet, callback: Hook) {
        self.hooks.push((key_set, callback));
    }

    pub fn unregister(&mut self, key_set: KeySet) {
        for i in 0..self.hooks.len() {
            if self.hooks[i].0 == key_set {
                self.hooks.remove(i);
            }
        }
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
                    process::exit(1);
                }
            }
        });

        loop {
            if let Some(event) = rx.recv().await {
                self.history.push(event);

                if self.history.len() > self.max_history {
                    self.history = self.history[self.history.len() - 128..].to_vec();
                }

                tokio::spawn(
                    hook(self.history.clone(), self.hooks.clone())
                        .unwrap_or_else(|e| eprint!("{}", e)),
                );
            }
        }
    }
}
