pub mod listener;

use anyhow::Result;
use rdev::{listen, Button, EventType};
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        listen(move |event| {
            tx.send(event)
                .unwrap_or_else(|e| eprintln!("Could not send event {:?}", e));
        })
        .expect("Could not listen");
    });

    loop {
        let event = match rx.recv().await {
            Some(event) => event,
            None => {
                continue;
            }
        };
        match event.event_type {
            rdev::EventType::ButtonPress(rdev::Button::Left) => {
                let next_event = loop {
                    match timeout(Duration::from_secs_f64(0.15), rx.recv()).await {
                        Ok(Some(event)) => match event.event_type {
                            EventType::ButtonRelease(Button::Left) => continue,
                            EventType::ButtonPress(Button::Left) => {
                                break Some(event);
                            }
                            _ => continue,
                        },
                        Err(_) => {
                            break None;
                        }
                        _ => continue,
                    };
                };
                if next_event.is_some() {
                    let duration = next_event
                        .unwrap()
                        .time
                        .duration_since(event.time)?
                        .as_secs_f64();
                    if duration < 0.15 {
                        println!("double click!");
                    } else {
                        println!("clicked once!")
                    }
                } else {
                    println!("clicked once!")
                }
            }
            _ => {}
        }
    }
}
