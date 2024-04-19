use std::process;

use anyhow::Result;
use futures::{future::BoxFuture, TryFutureExt};
use log::{debug, info};
use rdev::{listen, Event};
use tokio::sync::mpsc;

use crate::key::KeySet;

pub type HookResult = BoxFuture<'static, Result<()>>;
pub type Hook = fn() -> HookResult;

#[derive(Clone, Debug)]
pub struct Listener {
    pub max_history: u32,
    history: Vec<Event>,
    hooks: Vec<(KeySet, Hook)>,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            history: Default::default(),
            hooks: Default::default(),
            max_history: 128,
        }
    }
}

pub async fn hook(history: Vec<Event>, hooks: Vec<(KeySet, Hook)>) -> Result<()> {
    let history_length = history.len();
    for (mut key_set, hook) in hooks {
        let mut matched = true;
        let key_set_length = key_set.len();
        if key_set_length > history_length {
            continue;
        }
        // println!("Key Set Length: {}", key_set_length);
        let mut history_to_match = history[history_length - key_set_length..].to_vec();

        // print!("match:");
        // for his in &history_to_match {
        //     print!(" {:?}", his.event_type);
        // }
        // print!("\n");
        for mut bind_key in key_set {
            // println!("bind_len: {}", bind_key.len());
            // println!("history_to_match_len: {}", history_to_match.len());
            let to_idx = bind_key.len();
            // println!("to: {}", to_idx);
            let mut checks = history_to_match[..to_idx].to_vec();
            history_to_match = history_to_match[to_idx..].to_vec();

            // print!("checks:");
            // for his in &checks {
            //     print!(" {:?}", his.event_type);
            // }
            // print!("\n");

            let mut found = false;
            while !checks.is_empty() {
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
            if !found {
                matched = false;
                break;
            }
        }
        if matched {
            hook().await?;
        }
    }
    Ok(())
}

// pub async fn hook(history: Vec<Event>, hooks: Vec<(Vec<KeySet>, Hook)>) -> Result<()> {
//     for (hot_keys, hook) in hooks {
//         let mut checked = true;
//         for hot_key in hot_keys {
//             let mut checks = Vec::new();
//             let mut keys = hot_key.bind_keys.clone();
//             if history.len() < hot_key.bind_keys.len() {
//                 checked = false;
//                 break;
//             }
//             history[history.len() - hot_key.bind_keys.len()..].clone_into(&mut checks);
//             let mut matched = true;
//             while !checks.is_empty() {
//                 let mut found = false;
//                 let mut check_idx = 0;
//                 while check_idx < checks.len() {
//                     let mut key_idx = 0;
//                     while key_idx < keys.len() {
//                         if checks[check_idx].event_type == keys[key_idx].keys {
//                             found = true;
//                             checks.remove(check_idx);
//                             keys.remove(key_idx);
//                             break;
//                         }
//                         key_idx += 1;
//                     }
//                     if found {
//                         break;
//                     }
//                     check_idx += 1;
//                 }
//                 if !found {
//                     matched = false;
//                     break;
//                 }
//             }
//             if !matched {
//                 checked = false;
//                 break;
//             }
//         }
//         if checked {
//             hook().await?;
//         }
//     }
//     Ok(())
// }

impl Listener {
    pub fn new(max_history: u32, history: Vec<Event>, hooks: Vec<(KeySet, Hook)>) -> Self {
        Self {
            history,
            hooks,
            max_history,
        }
    }

    pub fn register(&mut self, key_set: KeySet, callback: Hook) {
        self.hooks.push((key_set, callback));
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
                tokio::spawn(
                    hook(self.history.clone(), self.hooks.clone())
                        .unwrap_or_else(|e| eprint!("{}", e)),
                );
            }
        }
    }
}
