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

pub struct CanvasManager {
    all: HashMap<DisplayId, WeakEntity<Canvas>>,
    use_history: VecDeque<DisplayId>,
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
        self.use_history.push_back(display_id);
    }

    pub fn undo(&mut self, cx: &mut App) -> bool {
        if let Some(id) = self.use_history.pop_back()
            && let Some(canvas) = self.all.get(&id)
        {
            canvas
                .update(cx, |canvas, cx| {
                    canvas.undo();
                    cx.notify();
                })
                .unwrap();

            !self.use_history.is_empty()
        } else {
            false
        }
    }
}

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
}

pub struct Canvas {
    window_handle: AnyWindowHandle,
    display_id: DisplayId,
    stack: VecDeque<CanvasPath>,
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
        for path in self.stack.iter() {
            path.paint(window)
        }
    }

    pub fn draw(&mut self, cx: &App, pos: Point<Pixels>) {
        if !self.painting {
            self.painting = true;
            let state = GlobalState::global(cx);

            let color = if state.tool == Tool::Eraser {
                gpui::transparent_white()
            } else {
                state.color
            };

            let mut path = CanvasPath::new(color);
            path.draw(pos);
            self.stack.push_back(path);
        } else {
            let path = self.stack.get_mut(self.stack.len() - 1).unwrap();
            path.draw(pos);
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

    pub fn undo(&mut self) {
        self.stack.pop_back();
    }
}
