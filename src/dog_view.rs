use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::text::TextView;
use gpui_component::*;

use crate::dog::Dog;

pub struct DogView {
    pub dog: &'static Dog,
    focus_handle: FocusHandle,
}

impl Focusable for DogView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl DogView {
    pub fn new(dog: &'static Dog, cx: &mut Context<Self>) -> Self {
        Self {
            dog,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for DogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let dog = self.dog;

        v_flex()
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().background)
            .child(
                // Breed header strip
                gpui::div()
                    .px_6()
                    .py_3()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        gpui::div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(dog.breed),
                    ),
            )
            .child(
                // Markdown content, scrollable
                gpui::div()
                    .flex_1()
                    .overflow_hidden()
                    .p_6()
                    .child(
                        TextView::markdown(("dog-facts", dog.id.0 as u64), dog.facts_markdown)
                            .selectable(true),
                    ),
            )
    }
}
