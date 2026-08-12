//! Minimal gpui web smoke test: one colored div, no storybook.
use gpui::{
    App, AppContext, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};

struct Smoke;

impl Render for Smoke {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x2563eb))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .p_4()
                    .rounded_md()
                    .bg(rgb(0xffffff))
                    .text_color(rgb(0x111111))
                    .child("rcn wasm smoke test"),
            )
    }
}

fn main() {
    gpui_platform::web_init();
    let app =
        gpui_platform::application_with_web_backend(gpui_platform::WebBackendPreference::Auto)
            .run_embedded(|cx: &mut App| {
                let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    |_, cx| cx.new(|_| Smoke),
                )
                .expect("failed to open window");
                cx.activate(true);
            });
    std::mem::forget(app);
}
