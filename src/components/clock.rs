use chrono::{DateTime, Local};
use std::time::{Duration, SystemTime};

use gpui::{
    div, rems, rgb, Context, IntoElement, ParentElement, Rems, Render, SharedString, Styled, Task,
    Window,
};

pub struct Clock {
    pub clock_task: Option<Task<()>>,
    pub current_time: Duration,
}

impl Clock {
    pub fn new() -> Clock {
        Self {
            clock_task: None,
            current_time: Duration::from_secs(0),
        }
    }

    pub fn start(&mut self, cx: &mut Context<Clock>) {
        self.spawn_timer(cx);
    }

    pub fn stop(&mut self) {
        self.discard_timer();
    }

    fn discard_timer(&mut self) {
        if let Some(task) = self.clock_task.take() {
            drop(task);
        }
    }

    fn spawn_timer(&mut self, cx: &mut Context<Clock>) {
        self.clock_task = Some(cx.spawn(async |ent, cx| loop {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            ent.update(cx, |clock, cx| {
                cx.notify();
                clock.current_time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
            })
            .unwrap();
        }))
    }
}

impl Render for Clock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .child(format!(
                        "{}",
                        DateTime::<Local>::from(SystemTime::UNIX_EPOCH + self.current_time)
                            .format("%H:%M:%S")
                    ))
                    .text_color(rgb(0xfaef8c))
                    .text_size(rems(2.5)),
            )
            .child(
                div()
                    .child(format!(
                        "{}",
                        DateTime::<Local>::from(SystemTime::UNIX_EPOCH + self.current_time)
                            .format("%m/%d")
                    ))
                    .text_size(rems(1.5)),
            )
    }
}
