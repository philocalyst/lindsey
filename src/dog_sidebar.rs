use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::sidebar::{
    Sidebar, SidebarCollapsible, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
    SidebarMenuItem,
};
use gpui_component::*;

use crate::dog::{DogId, all_dogs};

pub struct DogSelected {
    pub dog_id: DogId,
}

pub struct DogSidebar {
    pub collapsed: bool,
    selected: Option<DogId>,
    focus_handle: FocusHandle,
}

impl EventEmitter<DogSelected> for DogSidebar {}

impl Focusable for DogSidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl DogSidebar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            collapsed: false,
            selected: None,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for DogSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let mut items = Vec::new();
        for dog in all_dogs() {
            let dog_id = dog.id;
            let is_selected = self.selected == Some(dog_id);
            let handler = cx.listener(move |this, _event, _window, cx| {
                this.selected = Some(dog_id);
                cx.emit(DogSelected { dog_id });
                cx.notify();
            });
            items.push(
                SidebarMenuItem::new(dog.name)
                    .icon(dog.icon.clone())
                    .active(is_selected)
                    .on_click(handler),
            );
        }

        v_flex()
            .h_full()
            .child(
                Sidebar::new("dog-sidebar")
                    .collapsible(SidebarCollapsible::Icon)
                    .collapsed(self.collapsed)
                    .header(
                        SidebarHeader::new().child(
                            v_flex()
                                .gap_1()
                                .child(
                                    gpui::div()
                                        .text_sm()
                                        .font_bold()
                                        .text_color(cx.theme().sidebar_primary_foreground)
                                        .child("Dogpedia"),
                                )
                                .child(
                                    gpui::div()
                                        .text_xs()
                                        .text_color(cx.theme().sidebar_foreground)
                                        .child("A dog fact browser"),
                                ),
                        ),
                    )
                    .child(SidebarGroup::new("Dogs").child(SidebarMenu::new().children(items)))
                    .footer(
                        SidebarFooter::new().child(
                            gpui::div()
                                .text_xs()
                                .text_color(cx.theme().sidebar_foreground)
                                .child("5 breeds catalogued"),
                        ),
                    ),
            )
    }
}
