pub mod listener;

use std::time::Duration;

use anyhow::Result;
use futures::FutureExt;
use listener::{HookResult, HotKey, Listener};
// use rdev::{listen, Button, EventType};
// use tokio::sync::mpsc;
// use tokio::time::{timeout, Duration};

// #[tokio::main]
// async fn main() -> Result<()> {
//     let (tx, mut rx) = mpsc::unbounded_channel();
//     tokio::spawn(async move {
//         listen(move |event| {
//             tx.send(event)
//                 .unwrap_or_else(|e| eprintln!("Could not send event {:?}", e));
//         })
//         .expect("Could not listen");
//     });

//     loop {
//         let event = match rx.recv().await {
//             Some(event) => event,
//             None => {
//                 continue;
//             }
//         };
//         match event.event_type {
//             rdev::EventType::ButtonPress(rdev::Button::Left) => {
//                 let next_event = loop {
//                     match timeout(Duration::from_secs_f64(0.15), rx.recv()).await {
//                         Ok(Some(event)) => match event.event_type {
//                             EventType::ButtonRelease(Button::Left) => continue,
//                             EventType::ButtonPress(Button::Left) => {
//                                 break Some(event);
//                             }
//                             _ => continue,
//                         },
//                         Err(_) => {
//                             break None;
//                         }
//                         _ => continue,
//                     };
//                 };
//                 if next_event.is_some() {
//                     let duration = next_event
//                         .unwrap()
//                         .time
//                         .duration_since(event.time)?
//                         .as_secs_f64();
//                     if duration < 0.15 {
//                         println!("double click!");
//                     } else {
//                         println!("clicked once!")
//                     }
//                 } else {
//                     println!("clicked once!")
//                 }
//             }
//             _ => {}
//         }
//     }
// }

fn clicked() -> HookResult {
    async move {
        println!("左键被按下");
        Ok(())
    }
    .boxed()
}

fn press_c() -> HookResult {
    async move {
        println!("C被按下");
        Ok(())
    }
    .boxed()
}

fn press_ctrl_d() -> HookResult {
    async move {
        println!("Ctrl+D被按下");
        Ok(())
    }
    .boxed()
}

fn double_clicked() -> HookResult {
    async move {
        println!("鼠标双击");
        Ok(())
    }
    .boxed()
}

#[tokio::main]
async fn main() -> Result<()> {
    // for i in 0..10 {
    //     println!("{}", i);
    // }
    env_logger::init();
    // let route_button_left = vec![HotKey::new(
    //     vec![rdev::EventType::ButtonPress(rdev::Button::Left)],
    //     Duration::from_secs(0),
    // )];
    let route_c = vec![HotKey::new(
        vec![rdev::EventType::KeyPress(rdev::Key::KeyC)],
        Duration::from_secs(0),
    )];
    let ctrl_d = vec![HotKey::new(
        vec![
            rdev::EventType::KeyPress(rdev::Key::KeyD),
            rdev::EventType::KeyPress(rdev::Key::ControlLeft),
        ],
        Duration::from_secs(0),
    )];
    let double_click = vec![
        HotKey::new(
            vec![rdev::EventType::ButtonPress(rdev::Button::Left)],
            Duration::from_secs_f64(0.15),
        ),
        HotKey::new(
            vec![rdev::EventType::ButtonRelease(rdev::Button::Left)],
            Duration::from_secs_f64(0.15),
        ),
    ];

    let mut listener = Listener::new();
    // listener.register(route_button_left, clicked);
    listener.register(route_c, press_c);
    listener.register(ctrl_d, press_ctrl_d);
    listener.register(double_click, double_clicked);
    listener.listen().await?;
    Ok(())
}
