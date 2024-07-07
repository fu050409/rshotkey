use std::sync::Arc;

use anyhow::Result;
use futures::TryFutureExt;
use rdev::Event;
use tokio::sync::RwLock;

use crate::{history::History, key::KeySet, listener::Hook};

pub async fn hook_keyset(
    key_set: KeySet,
    history_arc: Arc<RwLock<History>>,
    hook: Hook,
) -> Result<()> {
    let mut matched = true;
    let history = history_arc.read().await.clone();
    let key_set_length = key_set.len();
    if key_set_length > history.len() {
        return Ok(());
    }

    let mut history_to_match = history.last_n(key_set_length).to_vec();

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
                    if bind_key.keys[key_idx] == checks[check_idx].event_type.into() {
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
    history: Arc<RwLock<History>>,
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
    history_arc: Arc<RwLock<History>>,
    hooks: Arc<RwLock<Vec<(KeySet, Hook)>>>,
    max_history: usize,
) {
    let mut history = history_arc.write().await;
    history.push(event);

    if history.len() > max_history {
        history.clean();
    }

    drop(history);

    tokio::spawn(
        hook(Arc::clone(&history_arc), Arc::clone(&hooks)).unwrap_or_else(|e| eprint!("{}", e)),
    );
}
