use std::time::Duration;

use rshotkey::key::{BindKey, KeySet};
use rshotkey::listener::{HookResult, Listener};
use anyhow::Result;
use futures::FutureExt;
use rdev::{Button, EventType, Key};

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
