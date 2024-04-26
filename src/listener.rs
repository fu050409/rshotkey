#![allow(unused)]
use std::{process, sync::Arc, time::SystemTime};

use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, TryFutureExt};
use rdev::{listen, Event};
use tokio::{sync::RwLock, task::JoinHandle, time};

use crate::key::KeySet;

pub type HookResult = BoxFuture<'static, Result<()>>;
pub type Hook = fn() -> HookResult;

#[derive(Clone, Debug)]
pub struct Listener {
    max_history: usize,
    history: Arc<RwLock<Vec<Event>>>,
    hooks: Arc<RwLock<Vec<(KeySet, Hook)>>>,
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

pub async fn run(
    history_arc: Arc<RwLock<Vec<Event>>>,
    hooks: Arc<RwLock<Vec<(KeySet, Hook)>>>,
    max_history: usize,
) {
    tokio::spawn(async move {
        match listen(move |event| {
            tokio::spawn(handle_event(
                event,
                Arc::clone(&history_arc),
                Arc::clone(&hooks),
                max_history,
            ));
        }) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Unable to listen: {:?}", e);
                process::exit(1);
            }
        }
    });
}

pub async fn hook_keyset(
    key_set: KeySet,
    history_arc: Arc<RwLock<Vec<Event>>>,
    hook: Hook,
) -> Result<()> {
    let mut matched = true;
    let history = history_arc.read().await.clone();
    let key_set_length = key_set.len();
    if key_set_length > history.len() {
        return Ok(());
    }

    let mut history_to_match = history[history.len() - key_set_length..].to_vec();

    let last_key = key_set.last().unwrap();
    let last_delay = last_key.delay_time;

    let last_history = history.last().unwrap();
    let last_time = last_history.time;

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

        if !bind_key.delay_time.is_zero() && !history_to_match.is_empty() {
            let next_time = history_to_match[0].time;
            let duration = next_time.duration_since(last_check.time)?;
            if duration > bind_key.delay_time {
                matched = false;
                break;
            }
        }
    }
    if matched {
        // let now = SystemTime::now();
        // if !last_delay.is_zero() {
        //     let elapsed = now.duration_since(last_time)?;
        //     if elapsed < last_delay {
        //         time::sleep(last_delay - elapsed).await;
        //     }
        //     let now_last_time = history_arc.read().await.last().unwrap().time;
        //     if now_last_time != last_history.time {
        //         matched = false;
        //     }
        // };
        if matched {
            hook().await?;
        }
    };
    Ok(())
}

pub async fn hook(
    history: Arc<RwLock<Vec<Event>>>,
    hooks_arc: Arc<RwLock<Vec<(KeySet, Hook)>>>,
) -> Result<()> {
    let hooks = hooks_arc.read().await.clone();
    for (key_set, hook) in hooks {
        hook_keyset(key_set, Arc::clone(&history), hook).await?;
    }
    Ok(())
}

pub async fn handle_event(
    event: Event,
    history_arc: Arc<RwLock<Vec<Event>>>,
    hooks: Arc<RwLock<Vec<(KeySet, Hook)>>>,
    max_history: usize,
) {
    let mut history = history_arc.write().await;
    history.push(event);

    if history.len() > max_history {
        *history = history[history.len() - 128..].to_vec();
    }

    drop(history);

    tokio::spawn(
        hook(Arc::clone(&history_arc), Arc::clone(&hooks)).unwrap_or_else(|e| eprint!("{}", e)),
    );
}

impl Listener {
    pub fn new(max_history: usize, history: Vec<Event>, hooks: Vec<(KeySet, Hook)>) -> Self {
        Self {
            history: Arc::new(RwLock::new(history)),
            hooks: Arc::new(RwLock::new(hooks)),
            max_history,
        }
    }

    pub async fn register(&self, key_set: KeySet, callback: Hook) -> Result<()> {
        if key_set.is_empty() {
            Err(anyhow!("Key set should not be empty!"))
        } else {
            self.hooks.write().await.push((key_set, callback));
            Ok(())
        }
    }

    pub async fn unregister(&self, key_set: KeySet) {
        let mut hooks = self.hooks.write().await;
        for i in 0..hooks.len() {
            if hooks[i].0 == key_set {
                hooks.remove(i);
            }
        }
    }

    pub async fn priorkey(&self) -> Option<Event> {
        self.history.read().await.last().cloned()
    }

    pub fn listen(&self) -> JoinHandle<()> {
        tokio::spawn(run(
            Arc::clone(&self.history),
            Arc::clone(&self.hooks),
            self.max_history.clone(),
        ))
    }
}
