use std::{process, sync::Arc};

use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use rdev::{listen, Event};
use tokio::{sync::RwLock, task::JoinHandle};

use crate::{history::History, hooks::handle_event, key::KeySet};

pub type HookResult = BoxFuture<'static, Result<()>>;
pub type Hook = fn() -> HookResult;

#[derive(Clone, Debug)]
pub struct Listener {
    max_history: usize,
    history: Arc<RwLock<History>>,
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

impl Listener {
    pub fn new(max_history: usize, history: History, hooks: Vec<(KeySet, Hook)>) -> Self {
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

    pub async fn prior_key(&self) -> Option<Event> {
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

pub async fn run(
    history_arc: Arc<RwLock<History>>,
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
