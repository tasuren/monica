use std::collections::VecDeque;

use gpui::{App, Global, Hsla, PathBuilder, Pixels, Point, ReadGlobal, UpdateGlobal, Window, px};

use crate::canvas_window_manager::CanvasWindowManager;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Cursor,
    Pen,
    Eraser,
    Highlight,
}

impl Tool {
    pub fn is_canvas_related(&self) -> bool {
        *self == Self::Pen || *self == Self::Eraser
    }
}

pub struct ToolState {
    tool: Tool,
    pub color: Hsla,
}

impl Global for ToolState {}

impl ToolState {
    pub fn register_global(cx: &mut App, tool: Tool, color: Hsla) {
        cx.set_global(Self { color, tool });
    }

    pub fn tool(&self) -> Tool {
        self.tool
    }

    pub fn set_tool(&mut self, cx: &mut App, tool: Tool) {
        self.tool = tool;

        CanvasWindowManager::update_global(cx, |windows, cx| {
            let canvas_action_mode = !tool.is_canvas_related();
            windows.set_action_mode(cx, canvas_action_mode);
        });
    }
}

#[derive(Clone, Debug)]
pub struct CanvasEraser {
    radius: Pixels,
    trail: Vec<Point<Pixels>>,
}

impl CanvasEraser {
    fn new(radius: Pixels) -> Self {
        Self {
            radius,
            trail: Vec::new(),
        }
    }

    fn draw(&mut self, pos: Point<Pixels>) {
        self.trail.push(pos);
    }
}

#[derive(Clone, Debug)]
pub struct CanvasPath {
    color: Hsla,
    stroke: Pixels,
    trail: Vec<Point<Pixels>>,
}

impl CanvasPath {
    fn new(color: Hsla) -> Self {
        let stroke = px(3.);

        Self {
            color,
            stroke,
            trail: Vec::new(),
        }
    }

    fn paint(&self, window: &mut Window) {
        if self.trail.is_empty() {
            return;
        }

        let mut path = PathBuilder::stroke(self.stroke);
        let mut iter = self.trail.iter().cloned();
        path.move_to(iter.next().unwrap());

        for pos in iter {
            path.line_to(pos);
        }

        window.paint_path(path.build().unwrap(), self.color);
    }

    fn draw(&mut self, pos: Point<Pixels>) {
        self.trail.push(pos);
    }

    fn erase(&self, eraser_trail: &[Point<Pixels>], radius: Pixels) -> Option<Vec<Self>> {
        let radius_f32 = f32::from(radius);
        let radius_sq = radius_f32 * radius_f32;
        let mut new_paths = Vec::new();
        let mut current_trail = Vec::new();
        let mut modified = false;

        for point in &self.trail {
            let mut hit = false;
            for e_pos in eraser_trail {
                let dx = f32::from(point.x - e_pos.x);
                let dy = f32::from(point.y - e_pos.y);
                if dx * dx + dy * dy <= radius_sq {
                    hit = true;
                    break;
                }
            }

            if hit {
                modified = true;

                if !current_trail.is_empty() {
                    new_paths.push(Self {
                        color: self.color,
                        stroke: self.stroke,
                        trail: current_trail,
                    });
                    current_trail = Vec::new();
                }
            } else {
                current_trail.push(*point);
            }
        }

        if !modified {
            return None;
        }

        if !current_trail.is_empty() {
            new_paths.push(Self {
                color: self.color,
                stroke: self.stroke,
                trail: current_trail,
            });
        }

        Some(new_paths)
    }
}

#[derive(Clone, Debug)]
pub enum CanvasAction {
    Clear,
    DrawLine(CanvasPath),
    Erase(CanvasEraser),
}

pub struct Canvas {
    stack: VecDeque<CanvasAction>,
    painting: bool,
    highlight_pos: Option<Point<Pixels>>,
}

impl Canvas {
    pub const MAX_STACK_SIZE: usize = 1000;

    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            painting: false,
            highlight_pos: None,
        }
    }

    pub fn paint(&mut self, window: &mut Window) {
        // Normal user drawings
        let start_index = self
            .stack
            .iter()
            .rposition(|action| matches!(action, CanvasAction::Clear))
            .map(|i| i + 1)
            .unwrap_or(0);

        let mut visible_paths: Vec<CanvasPath> = Vec::new();
        for action in self.stack.iter().skip(start_index) {
            match action {
                CanvasAction::DrawLine(path) => visible_paths.push(path.clone()),
                CanvasAction::Erase(eraser) => {
                    let mut next_paths = Vec::new();
                    for path in visible_paths {
                        if let Some(fragments) = path.erase(&eraser.trail, eraser.radius) {
                            next_paths.extend(fragments);
                        } else {
                            next_paths.push(path);
                        }
                    }

                    visible_paths = next_paths;
                }
                CanvasAction::Clear => visible_paths.clear(),
            }
        }

        for path in visible_paths {
            path.paint(window);
        }

        // Cursor highlight
        if let Some(pos) = self.highlight_pos.take() {
            let cx = pos.x;
            let cy = pos.y;
            let r = px(20.0);
            let mut path = PathBuilder::fill();

            path.move_to(Point::new(cx + r, cy));
            path.arc_to(
                Point::new(r, r),
                px(0.0),
                false,
                false,
                Point::new(cx - r, cy),
            );
            path.arc_to(
                Point::new(r, r),
                px(0.0),
                false,
                false,
                Point::new(cx + r, cy),
            );

            window.paint_path(path.build().unwrap(), gpui::red().alpha(0.4))
        }
    }

    pub fn draw(&mut self, cx: &App, pos: Point<Pixels>) {
        if !self.painting {
            self.painting = true;
            let state = ToolState::global(cx);

            if state.tool == Tool::Eraser {
                let mut eraser = CanvasEraser::new(px(20.));
                eraser.draw(pos);
                self.push_action(CanvasAction::Erase(eraser));
            } else {
                let color = state.color;
                let mut path = CanvasPath::new(color);
                path.draw(pos);
                self.push_action(CanvasAction::DrawLine(path));
            }
        } else {
            let maybe_action = self.stack.back_mut().unwrap();

            match maybe_action {
                CanvasAction::DrawLine(path) => path.draw(pos),
                CanvasAction::Erase(eraser) => eraser.draw(pos),
                _ => {}
            }
        }
    }

    fn push_action(&mut self, action: CanvasAction) {
        if self.stack.len() >= Self::MAX_STACK_SIZE {
            self.stack.pop_front();
        }

        self.stack.push_back(action);
    }

    pub fn is_painting(&self) -> bool {
        self.painting
    }

    pub fn flush(&mut self) {
        self.painting = false;
    }

    pub fn undo(&mut self) {
        self.painting = false;
        self.stack.pop_back();
    }

    pub fn clear(&mut self) {
        self.painting = false;
        self.push_action(CanvasAction::Clear);
    }

    pub fn set_highlight(&mut self, pos: Point<Pixels>) {
        self.highlight_pos = Some(pos);
    }

    pub fn clear_highlight(&mut self) {
        self.highlight_pos = None;
    }
}
