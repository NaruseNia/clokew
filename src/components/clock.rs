use chrono::{DateTime, Local};
use std::time::{Duration, SystemTime};

use gpui::{
    div, px, rems, rgb, Context, IntoElement, ParentElement, Rems, Render, SharedString, Styled,
    Task, Window,
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
        let now = DateTime::<Local>::from(SystemTime::UNIX_EPOCH + self.current_time);

        // Calculate next 6:00 AM
        let today_six_am = now
            .date_naive()
            .and_hms_opt(6, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let target_time = if now > today_six_am {
            today_six_am + chrono::Duration::days(1)
        } else {
            today_six_am
        };

        let duration_until_target = target_time - now;
        let hours = duration_until_target.num_hours();
        let minutes = duration_until_target.num_minutes() % 60;
        let seconds = duration_until_target.num_seconds() % 60;

        div()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .gap(px(-12.0))
            .child(
                div()
                    .child(format!("{}", now.format("%H:%M:%S")))
                    .text_color(rgb(0xfaef8c))
                    .text_size(rems(2.5)),
            )
            .child(
                div()
                    .child(format!("{}", now.format("%m/%d")))
                    .text_size(rems(1.5)),
            )
            .child(
                div()
                    .mt_4()
                    .text_size(rems(1.0))
                    .child(format!(
                        "{:02}:{:02}:{:02} (→6:00)",
                        hours, minutes, seconds
                    ))
                    .text_sm()
                    .text_color(rgb(0x5a5a5a)),
            )
    }
}
