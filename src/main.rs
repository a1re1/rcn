//! rcn — a copy-paste component library for gpui. This binary is the
//! storybook: browse each component in isolation and play with its options
//! in the controls panel (see [`storybook`]).

mod assets;
mod components;
mod motion;
mod storybook;
mod storybook_docs;
mod theme;

use gpui::{
    App, AppContext, Application, Bounds, Focusable as _, QuitMode, WindowBounds, WindowOptions,
    px, size,
};

use assets::Assets;
use components::Input;
use storybook::Storybook;
use theme::Theme;

fn main() {
    // At this gpui rev the platform lives in the gpui_platform crate; zed's own
    // main builds it the same way (current_platform → Application::with_platform).
    let platform = gpui_platform::current_platform(false);
    // macOS's default keeps the process alive after the last window closes
    // (document-app convention); a single-window tool should just quit.
    let app = Application::with_platform(platform)
        .with_assets(Assets)
        .with_quit_mode(QuitMode::LastWindowClosed);
    app.run(|cx: &mut App| {
        cx.set_global(Theme::light());
        Input::register_key_bindings(cx);
        let bounds = Bounds::centered(None, size(px(1100.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("rcn".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let storybook = cx.new(Storybook::new);
                window.focus(&storybook.focus_handle(cx), cx);
                storybook
            },
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
