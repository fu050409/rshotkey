#![allow(unused)]
use rshotkey::key::{BindKey, KeySet};
use rshotkey::listener::{HookResult, Listener};
use rshotkey::rdev::{Button, EventType, Key};

use anyhow::Result;
use futures::FutureExt;
use std::process;
use std::time::Duration;
use tokio::time;

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

    let c: KeySet = BindKey::new(vec![EventType::KeyPress(Key::KeyC).into()]).into();
    let ctrl_d_press: KeySet = BindKey::new(vec![
        EventType::KeyPress(Key::KeyD).into(),
        EventType::KeyPress(Key::ControlLeft).into(),
    ])
    .into();
    let ctrl_d_release = KeySet::default().bind(BindKey::new(vec![
        EventType::KeyRelease(Key::KeyD).into(),
        EventType::KeyRelease(Key::ControlLeft).into(),
    ]));
    let ctrl_d = KeySet::default()
        .bind(BindKey::new(vec![
            EventType::KeyPress(Key::KeyD).into(),
            EventType::KeyPress(Key::ControlLeft).into(),
        ]))
        .bind(BindKey::new(vec![
            EventType::KeyRelease(Key::KeyD).into(),
            EventType::KeyRelease(Key::ControlLeft).into(),
        ]));

    let left_click_once: KeySet = BindKey::new(vec![EventType::ButtonPress(Button::Left).into()])
        .delay(Duration::from_secs_f64(1.0))
        .into();

    let delay_click = KeySet::default()
        .bind(
            BindKey::new(vec![EventType::ButtonPress(Button::Left).into()])
                .delay(Duration::from_secs_f64(0.2)),
        )
        .bind(BindKey::new(vec![EventType::ButtonRelease(Button::Left).into()]));

    let double_click = KeySet::default()
        .bind(
            BindKey::new(vec![
                EventType::ButtonPress(Button::Left).into(),
                EventType::ButtonRelease(Button::Left).into(),
            ])
            .delay(Duration::from_secs_f64(0.2)),
        )
        .bind(BindKey::new(vec![EventType::ButtonPress(Button::Left).into()]));

    let listener = Listener::default();
    listener.register(c, press_c).await?;
    listener.register(ctrl_d_press, press_ctrl_d).await?;
    listener
        .register(ctrl_d_release, move || {
            async move {
                println!("Ctrl_D 被释放！");
                Ok(())
            }
            .boxed()
        })
        .await?;
    listener
        .register(ctrl_d, move || {
            async move {
                println!("完整的 Ctrl_D 过程！");
                Ok(())
            }
            .boxed()
        })
        .await?;

    listener.register(left_click_once, clicked).await?;
    listener
        .register(delay_click, move || {
            async move {
                println!("鼠标单击后在 0.2s 内释放！");
                Ok(())
            }
            .boxed()
        })
        .await?;
    listener.register(double_click, double_clicked).await?;

    let runner = listener.listen();

    time::sleep(Duration::from_secs(5)).await;

    println!("上一个键: {:?}", listener.prior_key().await);

    runner.await?;

    tokio::signal::ctrl_c().await?;

    process::exit(0);
}
