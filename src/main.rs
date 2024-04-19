pub mod event;
pub mod exception;
pub mod key;
pub mod listener;

use std::time::Duration;

use anyhow::Result;
use futures::FutureExt;
use key::{BindKey, KeySet};
use listener::{HookResult, Listener};
use rdev::{Button, EventType, Key};
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
    env_logger::init();

    // let route_button_left = vec![HotKey::new(
    //     vec![rdev::EventType::ButtonPress(rdev::Button::Left)],
    //     Duration::from_secs(0),
    // )];
    let c: KeySet = BindKey::new(vec![EventType::KeyPress(Key::KeyC)]).into();
    let ctrl_d_press: KeySet = BindKey::new(vec![
        EventType::KeyPress(Key::KeyD),
        EventType::KeyPress(Key::ControlLeft),
    ])
    .into();
    let ctrl_d_release = KeySet::default().bind(BindKey::new(vec![
        EventType::KeyRelease(Key::KeyD),
        EventType::KeyRelease(Key::ControlLeft),
    ]));
    let ctrl_d = KeySet::default()
        .bind(BindKey::new(vec![
            EventType::KeyPress(Key::KeyD),
            EventType::KeyPress(Key::ControlLeft),
        ]))
        .bind(BindKey::new(vec![
            EventType::KeyRelease(Key::KeyD),
            EventType::KeyRelease(Key::ControlLeft),
        ]));

    let left_click_once: KeySet = vec![EventType::ButtonPress(Button::Left)].into();

    let delay_click = KeySet::default()
        .bind(
            BindKey::new(vec![EventType::ButtonPress(Button::Left)])
                .delay(Duration::from_secs_f64(0.2)),
        )
        .bind(BindKey::new(vec![EventType::ButtonRelease(Button::Left)]));

    let double_click = KeySet::default()
        .bind(
            BindKey::new(vec![
                EventType::ButtonPress(Button::Left),
                EventType::ButtonRelease(Button::Left),
            ])
            .delay(Duration::from_secs_f64(0.2)),
        )
        .bind(BindKey::new(vec![EventType::ButtonPress(Button::Left)]));

    let mut listener = Listener::default();
    listener.register(c, press_c);
    listener.register(ctrl_d_press, press_ctrl_d);
    listener.register(ctrl_d_release, move || {
        async move {
            println!("Ctrl_D 被释放！");
            Ok(())
        }
        .boxed()
    });
    listener.register(ctrl_d, move || {
        async move {
            println!("完整的 Ctrl_D 过程！");
            Ok(())
        }
        .boxed()
    });

    listener.register(left_click_once, clicked);
    listener.register(delay_click, move || {
        async move {
            println!("鼠标单击后在 0.2s 内释放！");
            Ok(())
        }
        .boxed()
    });
    listener.register(double_click, double_clicked);
    listener.listen().await?;
    Ok(())
}
