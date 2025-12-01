use std::collections::{HashMap, VecDeque};

use display_config::DisplayId;
use gpui::{App, AppContext, Context, Entity, Global};

use crate::canvas::Canvas;

pub enum ActionScope {
    Display(DisplayId),
    All,
}

pub struct CanvasOrchestrator {
    canvases: HashMap<DisplayId, Entity<Canvas>>,
    action_history: VecDeque<ActionScope>,
}

impl Global for CanvasOrchestrator {}

impl CanvasOrchestrator {
    pub fn register_global(cx: &mut App) {
        let orchestrator = Self {
            canvases: HashMap::new(),
            action_history: VecDeque::new(),
        };

        cx.set_global(orchestrator);
    }

    pub fn add_canvas(&mut self, cx: &mut App, display_id: DisplayId) {
        self.canvases.insert(display_id, cx.new(|_| Canvas::new()));
    }

    pub fn remove_canvas(&mut self, display_id: &DisplayId) {
        self.canvases.remove(display_id);
    }

    pub fn undo(&mut self, cx: &mut App) {
        if let Some(scope) = self.action_history.pop_back() {
            match scope {
                ActionScope::Display(display_id) => {
                    if let Some(canvas) = self.canvases.get(&display_id) {
                        canvas.update(cx, |canvas, _| canvas.undo());
                    }
                }
                ActionScope::All => {
                    for canvas in self.canvases.values() {
                        canvas.update(cx, |canvas, _| canvas.undo());
                    }
                }
            };
        }
    }

    pub fn clear(&mut self, cx: &mut App) {
        for canvas in self.canvases.values() {
            canvas.update(cx, |canvas, _| canvas.clear())
        }

        self.action_history.push_back(ActionScope::All);
    }

    pub fn action_canvas(
        &mut self,
        cx: &mut App,
        display_id: DisplayId,
        f: impl FnOnce(&mut Canvas, &mut Context<'_, Canvas>),
    ) {
        if let Some(canvas) = self.canvases.get_mut(&display_id) {
            canvas.update(cx, f);

            self.action_history
                .push_back(ActionScope::Display(display_id));
        }
    }

    pub fn update_canvas(
        &mut self,
        cx: &mut App,
        display_id: &DisplayId,
        f: impl FnOnce(&mut Canvas, &mut Context<'_, Canvas>),
    ) {
        if let Some(canvas) = self.canvases.get_mut(display_id) {
            canvas.update(cx, f);
        }
    }
}
