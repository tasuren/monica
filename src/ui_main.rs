use gpui::{App, ElementId, Entity, MouseButton, ReadGlobal, UpdateGlobal, prelude::*, px};
use gpui_component::{
    ActiveTheme, Icon, Selectable, Sizable,
    button::{Button, ButtonCustomVariant, ButtonGroup, ButtonVariants},
    h_flex, v_flex,
};

use crate::canvas::{GlobalState, Tool};

pub struct AppView {
    title_bar: Entity<TitleBar>,
    tool_select: Entity<ToolSelect>,
}

impl AppView {
    pub fn new(cx: &mut Context<'_, Self>) -> Self {
        Self {
            title_bar: cx.new(|_| TitleBar),
            tool_select: cx.new(|_| ToolSelect),
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("toolbar")
            .size_full()
            .child(self.title_bar.clone())
            .child(self.tool_select.clone())
    }
}

struct TitleBar;

impl TitleBar {
    fn render_normal_button(
        &self,
        cx: &mut App,
        id: impl Into<ElementId>,
        icon_path: &'static str,
    ) -> Button {
        Button::new(id)
            .icon(Icon::empty().path(icon_path))
            .ghost()
            .custom(ButtonCustomVariant::new(cx).active(gpui::white().alpha(0.3)))
            .size_7()
            .with_size(px(32.))
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .h_10()
            .items_center()
            .bg(cx.theme().title_bar)
            .border_b_1()
            .border_color(cx.theme().border)
            .pt_1()
            .pr_1()
            .child(
                h_flex()
                    .when_else(
                        cfg!(target_os = "macos"),
                        |this| this.ml_auto(),
                        |this| this.mr_auto(),
                    )
                    .items_center()
                    .px_1()
                    .gap_1()
                    .child(
                        self.render_normal_button(cx, "undo-button", "icons/undo.svg")
                            .on_click(cx.listener(|_view, _, _, cx| {
                                GlobalState::update_global(cx, |state, cx| {
                                    state.canvas_manager().undo(cx);
                                });
                            })),
                    )
                    .child(
                        self.render_normal_button(cx, "trash-button", "icons/trash-2.svg")
                            .custom(
                                ButtonCustomVariant::new(cx)
                                    .foreground(gpui::red())
                                    .active(gpui::white().alpha(0.3)),
                            )
                            .on_click(cx.listener(|_, _, _, cx| {
                                GlobalState::update_global(cx, |state, cx| {
                                    state.canvas_manager().clear(cx);
                                })
                            })),
                    ),
            )
            .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                window.start_window_move();
            })
    }
}

impl Tool {
    fn from_number(number: usize) -> Self {
        match number {
            0 => Self::Cursor,
            1 => Self::Pen,
            2 => Self::Eraser,
            3 => Self::Highlight,
            _ => unreachable!(),
        }
    }
}

struct ToolSelect;

impl ToolSelect {
    fn render_tool_button(
        &self,
        cx: &mut App,
        id: impl Into<ElementId>,
        icon_path: &'static str,
        tool: Tool,
    ) -> Button {
        Button::new(id)
            .icon(Icon::empty().path(icon_path))
            .ghost()
            .custom(ButtonCustomVariant::new(cx).active(gpui::white().alpha(0.3)))
            .size_10()
            .with_size(px(36.))
            .selected(tool == GlobalState::global(cx).tool())
            .rounded_xl()
    }
}

impl Render for ToolSelect {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().size_full().items_center().child(
            ButtonGroup::new("toolbar-tools")
                .size_full()
                .justify_around()
                .items_center()
                .px_2()
                .gap_2()
                .child(self.render_tool_button(
                    cx,
                    "tool-cursor",
                    "icons/mouse-pointer-2.svg",
                    Tool::Cursor,
                ))
                .child(self.render_tool_button(cx, "tool-pen", "icons/pencil.svg", Tool::Pen))
                .child(self.render_tool_button(cx, "tool-eraser", "icons/eraser.svg", Tool::Eraser))
                .child(self.render_tool_button(
                    cx,
                    "tool-highlight",
                    "icons/circle.svg",
                    Tool::Highlight,
                ))
                .on_click(cx.listener(|_, selected: &Vec<usize>, _, cx| {
                    let tool = Tool::from_number(*selected.first().unwrap());

                    GlobalState::update_global(cx, |state, cx| {
                        state.set_tool(cx, tool);
                    });

                    cx.notify();
                })),
        )
    }
}
