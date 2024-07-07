use std::time::SystemTime;

use criterion::{criterion_group, criterion_main, Criterion};
use rdev::Event;
use rshotkey::history;

fn insert_benchmark(c: &mut Criterion) {
    let event = Event {
        time: SystemTime::now(),
        name: None,
        event_type: rdev::EventType::KeyPress(rdev::Key::Alt),
    };
    c.bench_function("history_insert", |b| {
        b.iter(|| {
            let mut history = history::History::new(256);
            for _ in 0..256 {
                history.push(event.clone())
            }
        })
    });
}

criterion_group!(benches, insert_benchmark,);
criterion_main!(benches);
