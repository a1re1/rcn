//! Bare-bones gpui bootstrap: opens a window and renders "Hello, gpui!".
//!
//! This is the seed of the rcn storybook — a gpui app for browsing the
//! component library. For now it only proves the toolchain works: platform
//! init, a window, styled divs, and real text rendering.

use gpui::{
    App, AppContext, Application, Bounds, Context, QuitMode, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(0xcdd6f4))
                    .child("Hello, gpui!"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6c7086))
                    .child("rcn — component library bootstrap"),
            )
    }
}

fn main() {
    // At this gpui rev the platform lives in the gpui_platform crate; zed's own
    // main builds it the same way (current_platform → Application::with_platform).
    let platform = gpui_platform::current_platform(false);
    // macOS's default keeps the process alive after the last window closes
    // (document-app convention); a single-window tool should just quit.
    let app = Application::with_platform(platform).with_quit_mode(QuitMode::LastWindowClosed);
    app.run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("rcn".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| HelloWorld),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
