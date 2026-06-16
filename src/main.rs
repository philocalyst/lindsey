mod dog;
mod dog_sidebar;
mod dog_view;
mod settings;
mod workspace;

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::{Root, TitleBar};
use gpui_component_assets::Assets;

use settings::LindseySettings;
use workspace::Workspace;

actions!(lindsey, [Quit, ToggleSidebar, CloseTab]);

fn main() {
    let app = gpui_platform::application().with_assets(Assets);

    app.run(|cx: &mut App| {
        gpui_component::init(cx);

        // ── Settings store + live file monitoring ─────────────────────────
        LindseySettings::init(cx);

        // ── App-level key bindings ────────────────────────────────────────
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseTab, None),
            KeyBinding::new("cmd-b", ToggleSidebar, None),
        ]);

        // ── Global actions ────────────────────────────────────────────────
        cx.on_action(|_: &Quit, cx| cx.quit());

        // ── Native menu bar ───────────────────────────────────────────────
        cx.set_menus(vec![
            Menu::new("Dogpedia").items([MenuItem::action("Quit Dogpedia", Quit)]),
            Menu::new("View").items([MenuItem::action("Toggle Sidebar", ToggleSidebar)]),
        ]);

        // ── Quit when last window is closed ──────────────────────────────
        cx.on_window_closed(|cx, _window_id| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // ── Open the main window ──────────────────────────────────────────
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let workspace = cx.new(|cx| Workspace::new(cx));
                cx.new(|cx| Root::new(workspace, window, cx))
            },
        )
        .expect("failed to open main window");

        cx.activate(true);
    });
}
