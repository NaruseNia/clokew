use gpui::*;
use gpui_component::{Root, TitleBar};

use crate::components::clock::Clock;

mod components;

struct Clokew {
    clock: Entity<Clock>,
}

impl Clokew {
    fn new(cx: &mut Context<Self>) -> Self {
        let clock = cx.new(|_| Clock::new());

        clock.update(cx, |ent, cx| {
            ent.start(cx);
        });

        Self { clock }
    }
}

impl Render for Clokew {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x141414))
            .size_full()
            .justify_center()
            .items_center()
            .child(self.clock.clone())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(280.), px(280.)), cx);
        let window_options = WindowOptions {
            titlebar: Some(TitleBar::title_bar_options()),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            is_resizable: false,
            ..Default::default()
        };

        cx.activate(true);
        cx.on_window_closed(|app| {
            if app.windows().is_empty() {
                app.quit();
            }
        })
        .detach();
        cx.spawn(async move |cx| -> anyhow::Result<()> {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Clokew::new(cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok(())
        })
        .detach();
    });
}
