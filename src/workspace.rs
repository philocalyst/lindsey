use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::sidebar::SidebarToggleButton;
use gpui_component::tab::{Tab, TabBar};
use gpui_component::theme::{Theme, ThemeMode};
use gpui_component::*;

use crate::{
    CloseTab, ToggleSidebar,
    dog::{DogId, all_dogs},
    dog_sidebar::{DogSelected, DogSidebar},
    dog_view::DogView,
    settings::LindseySettings,
};

pub struct Workspace {
    focus_handle: FocusHandle,
    sidebar: Entity<DogSidebar>,
    sidebar_collapsed: bool,
    open_dogs: Vec<Entity<DogView>>,
    active_tab: Option<usize>,
    _subscriptions: Vec<Subscription>,
}

impl Focusable for Workspace {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Workspace {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|cx| DogSidebar::new(cx));

        let subscription = cx.subscribe(
            &sidebar,
            |this, _sidebar, event: &DogSelected, cx| {
                this.open_dog(event.dog_id, cx);
            },
        );

        Self {
            focus_handle: cx.focus_handle(),
            sidebar,
            sidebar_collapsed: false,
            open_dogs: Vec::new(),
            active_tab: None,
            _subscriptions: vec![subscription],
        }
    }

    fn open_dog(&mut self, dog_id: DogId, cx: &mut Context<Self>) {
        // Activate existing tab if the dog is already open.
        if let Some(ix) = self
            .open_dogs
            .iter()
            .position(|view| view.read(cx).dog.id == dog_id)
        {
            self.active_tab = Some(ix);
            cx.notify();
            return;
        }

        if let Some(dog) = all_dogs().iter().find(|d| d.id == dog_id) {
            let view = cx.new(|cx| DogView::new(dog, cx));
            self.open_dogs.push(view);
            self.active_tab = Some(self.open_dogs.len() - 1);
            cx.notify();
        }
    }

    fn close_tab(&mut self, ix: usize, cx: &mut Context<Self>) {
        if ix >= self.open_dogs.len() {
            return;
        }
        self.open_dogs.remove(ix);
        self.active_tab = if self.open_dogs.is_empty() {
            None
        } else {
            Some(ix.min(self.open_dogs.len() - 1))
        };
        cx.notify();
    }

    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        let collapsed = self.sidebar_collapsed;
        self.sidebar.update(cx, |sidebar, cx| {
            sidebar.collapsed = collapsed;
            cx.notify();
        });
        cx.notify();
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark_mode = LindseySettings::get(cx).dark_mode;
        let sidebar_collapsed = self.sidebar_collapsed;
        let active_ix = self.active_tab;
        let active_dog_view = active_ix
            .and_then(|ix| self.open_dogs.get(ix))
            .cloned();

        // Pre-build tab list before constructing the element tree.
        let mut tabs: Vec<Tab> = Vec::new();
        for (ix, dog_view) in self.open_dogs.iter().enumerate() {
            let dog_name: SharedString = dog_view.read(cx).dog.name.into();
            let is_active = active_ix == Some(ix);

            let activate = cx.listener(move |this, _ev, _window, cx| {
                // Guard against the tab having been closed by the close button.
                if ix < this.open_dogs.len() {
                    this.active_tab = Some(ix);
                    cx.notify();
                }
            });
            let close = cx.listener(move |this, _ev, _window, cx| {
                cx.stop_propagation();
                this.close_tab(ix, cx);
            });

            tabs.push(
                Tab::new()
                    .label(dog_name)
                    .selected(is_active)
                    .on_click(activate)
                    .suffix(
                        div()
                            .id(("close-tab", ix as u64))
                            .cursor_pointer()
                            .px_1()
                            .rounded_sm()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .on_click(close)
                            .child("×"),
                    ),
            );
        }

        let sidebar = self.sidebar.clone();

        v_flex()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| {
                this.toggle_sidebar(cx);
            }))
            .on_action(cx.listener(|this, _: &CloseTab, _window, cx| {
                if let Some(ix) = this.active_tab {
                    this.close_tab(ix, cx);
                }
            }))
            .child(
                // ── Title bar ─────────────────────────────────────────────
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .px_2()
                        .items_center()
                        .justify_between()
                        .child(
                            h_flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    SidebarToggleButton::new()
                                        .collapsed(sidebar_collapsed)
                                        .on_click(cx.listener(|this, _ev, _window, cx| {
                                            this.toggle_sidebar(cx);
                                        })),
                                )
                                .child(div().font_bold().text_sm().child("Dogpedia")),
                        )
                        .child(
                            Button::new("theme-toggle")
                                .icon(if dark_mode {
                                    IconName::Sun
                                } else {
                                    IconName::Moon
                                })
                                .small()
                                .ghost()
                                .tooltip(if dark_mode {
                                    "Switch to light mode"
                                } else {
                                    "Switch to dark mode"
                                })
                                .on_click(cx.listener(|_this, _ev, _window, cx| {
                                    let new_dark = {
                                        let settings = cx.global_mut::<LindseySettings>();
                                        settings.dark_mode = !settings.dark_mode;
                                        settings.dark_mode
                                    };
                                    Theme::change(
                                        if new_dark {
                                            ThemeMode::Dark
                                        } else {
                                            ThemeMode::Light
                                        },
                                        None,
                                        cx,
                                    );
                                })),
                        ),
                ),
            )
            .child(
                // ── Main content row ───────────────────────────────────────
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .child(sidebar)
                    .child(
                        // ── Tab area ───────────────────────────────────────
                        v_flex()
                            .flex_1()
                            .h_full()
                            .min_w_0()
                            .child(TabBar::new("dog-tabs").w_full().children(tabs))
                            .child(
                                div()
                                    .flex_1()
                                    .min_h_0()
                                    .overflow_hidden()
                                    .when_some(active_dog_view, |this, view| this.child(view))
                                    .when(active_ix.is_none(), |this| {
                                        this.flex()
                                            .items_center()
                                            .justify_center()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(
                                                v_flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .child(div().text_xl().child("🐾"))
                                                    .child(
                                                        div().text_sm().child(
                                                            "Select a dog from the sidebar",
                                                        ),
                                                    ),
                                            )
                                    }),
                            ),
                    ),
            )
    }
}
