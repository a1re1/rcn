//! rcn storybook, in a browser: the same [`rcn::storybook::Storybook`] the
//! native app renders, booted through gpui's web platform (gpui_web +
//! gpui_wgpu) and drawn to a full-page canvas with WebGPU (or WebGL as a
//! fallback). Build and serve with `trunk serve` from this directory.

use gpui::{App, AppContext, Bounds, Focusable as _, WindowBounds, WindowOptions, px, size};

use rcn::assets::Assets;
use rcn::components::Input;
use rcn::storybook::Storybook;
use rcn::theme::Theme;

/// `?backend=webgpu` / `?backend=webgl` force a renderer; default auto-detects
/// (WebGPU where available, WebGL otherwise).
fn requested_backend() -> gpui_platform::WebBackendPreference {
    let search = web_sys::window()
        .and_then(|window| window.location().search().ok())
        .unwrap_or_default();
    let has = |needle: &str| {
        search
            .trim_start_matches('?')
            .split('&')
            .any(|p| p == needle)
    };
    if has("backend=webgpu") {
        gpui_platform::WebBackendPreference::WebGpu
    } else if has("backend=webgl") {
        gpui_platform::WebBackendPreference::WebGl
    } else {
        gpui_platform::WebBackendPreference::Auto
    }
}

fn main() {
    gpui_platform::web_init();
    // On wasm the browser owns the run loop: Platform::run returns immediately,
    // so `Application::run`'s stack frame — which keeps the App alive on native
    // — would drop the whole app right after launch. run_embedded returns a
    // handle instead; leak it so the app lives for the lifetime of the page.
    let app = gpui_platform::application_with_web_backend(requested_backend())
        .with_assets(Assets)
        .run_embedded(|cx: &mut App| {
            cx.set_global(Theme::light());
            Input::register_key_bindings(cx);
            let bounds = Bounds::centered(None, size(px(1100.0), px(720.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
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
    std::mem::forget(app);
}
