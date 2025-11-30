use std::collections::{HashMap, VecDeque};

use display_config::DisplayId;
use gpui::{
    AnyWindowHandle, App, AppContext, Entity, Hsla, PathBuilder, Pixels, Point, ReadGlobal,
    UpdateGlobal, WeakEntity, Window, px,
};

use crate::platform_impl::WindowExt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Cursor,
    Pen,
    Eraser,
    Hightlight,
}

impl Tool {
    pub fn is_canvas_related(&self) -> bool {
        *self == Self::Pen || *self == Self::Eraser
    }
}

pub struct GlobalState {
    tool: Tool,
    pub color: Hsla,
    canvas_manager: CanvasManager,
}

impl GlobalState {
    pub fn new(tool: Tool, color: Hsla) -> Self {
        Self {
            color,
            tool,
            canvas_manager: CanvasManager::new(),
        }
    }

    pub fn tool(&self) -> Tool {
        self.tool
    }

    pub fn set_tool(&mut self, cx: &mut App, tool: Tool) {
        self.tool = tool;

        for canvas in self.canvas_manager.all.values() {
            canvas
                .update(cx, |canvas, cx| {
                    canvas
                        .window_handle
                        .update(cx, |_, window, _| {
                            window.set_ignore_cursor_events(!tool.is_canvas_related())
                        })
                        .unwrap();
                })
                .unwrap();
        }
    }

    pub fn canvas_manager(&mut self) -> &mut CanvasManager {
        &mut self.canvas_manager
    }
}

impl gpui::Global for GlobalState {}

pub enum UseScope {
    OneDisplay(DisplayId),
    All,
}

pub struct CanvasManager {
    all: HashMap<DisplayId, WeakEntity<Canvas>>,
    use_history: VecDeque<UseScope>,
}

impl CanvasManager {
    pub fn new() -> Self {
        Self {
            all: HashMap::new(),
            use_history: VecDeque::new(),
        }
    }

    pub fn create_canvas(
        &mut self,
        cx: &mut App,
        window: &mut Window,
        display_id: DisplayId,
    ) -> Entity<Canvas> {
        let canvas = cx.new(|_| Canvas::new(display_id.clone(), window.window_handle()));
        self.all.insert(display_id, canvas.downgrade());
        canvas
    }

    fn use_canvas(&mut self, display_id: DisplayId) {
        self.use_history.push_back(UseScope::OneDisplay(display_id));
    }

    pub fn undo(&mut self, cx: &mut App) -> bool {
        if let Some(use_history) = self.use_history.pop_back() {
            match use_history {
                UseScope::All => {
                    for canvas in self.all.values() {
                        canvas
                            .update(cx, |canvas, cx| {
                                canvas.undo();
                                cx.notify();
                            })
                            .unwrap();
                    }
                }
                UseScope::OneDisplay(display_id) => {
                    if let Some(canvas) = self.all.get(&display_id) {
                        canvas
                            .update(cx, |canvas, cx| {
                                canvas.undo();
                                cx.notify();
                            })
                            .unwrap();
                    };
                }
            }

            !self.use_history.is_empty()
        } else {
            false
        }
    }

    pub fn clear(&mut self, cx: &mut App) {
        for canvas in self.all.values() {
            canvas
                .update(cx, |canvas, cx| {
                    canvas.clear();
                    cx.notify();
                })
                .unwrap();
        }

        self.use_history.push_back(UseScope::All);
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
    window_handle: AnyWindowHandle,
    display_id: DisplayId,
    stack: VecDeque<CanvasAction>,
    painting: bool,
}

impl Canvas {
    pub fn new(display_id: DisplayId, window_handle: AnyWindowHandle) -> Self {
        Self {
            window_handle,
            display_id,
            stack: VecDeque::new(),
            painting: false,
        }
    }

    pub fn paint(&self, window: &mut Window) {
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
    }

    pub fn draw(&mut self, cx: &App, pos: Point<Pixels>) {
        if !self.painting {
            self.painting = true;
            let state = GlobalState::global(cx);

            if state.tool == Tool::Eraser {
                let mut eraser = CanvasEraser::new(px(20.));
                eraser.draw(pos);
                self.stack.push_back(CanvasAction::Erase(eraser));
            } else {
                let color = state.color;
                let mut path = CanvasPath::new(color);
                path.draw(pos);
                self.stack.push_back(CanvasAction::DrawLine(path));
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

    pub fn is_painting(&self) -> bool {
        self.painting
    }

    pub fn flush(&mut self, cx: &mut App) {
        self.painting = false;

        GlobalState::update_global(cx, |state, _| {
            state.canvas_manager().use_canvas(self.display_id.clone())
        });
    }

    fn undo(&mut self) {
        self.painting = false;
        self.stack.pop_back();
    }

    fn clear(&mut self) {
        self.painting = false;
        self.stack.push_back(CanvasAction::Clear);
    }
}
