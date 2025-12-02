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
    cursor_display_pos: Option<DisplayId>,
}

impl Global for CanvasOrchestrator {}

impl CanvasOrchestrator {
    pub fn register_global(cx: &mut App) {
        let orchestrator = Self {
            canvases: HashMap::new(),
            action_history: VecDeque::new(),
            cursor_display_pos: None,
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
                        canvas.update(cx, |canvas, cx| {
                            canvas.undo();
                            cx.notify();
                        });
                    }
                }
                ActionScope::All => {
                    for canvas in self.canvases.values() {
                        canvas.update(cx, |canvas, cx| {
                            canvas.undo();
                            cx.notify();
                        });
                    }
                }
            };
        }
    }

    pub fn clear(&mut self, cx: &mut App) {
        for canvas in self.canvases.values() {
            canvas.update(cx, |canvas, cx| {
                canvas.clear();
                cx.notify();
            });
        }

        self.action_history.push_back(ActionScope::All);
    }

    pub fn action_canvas(
        &mut self,
        cx: &mut App,
        display_id: DisplayId,
        f: impl FnOnce(&mut Canvas, &mut Context<'_, Canvas>) -> bool,
    ) {
        if let Some(canvas) = self.canvases.get_mut(&display_id) {
            let action = canvas.update(cx, f);

            if action {
                self.action_history
                    .push_back(ActionScope::Display(display_id));
            }
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

    pub fn notify_old_working_canvas(&mut self, cx: &mut App, new_display_id: Option<&DisplayId>) {
        if let Some(new_display_id) = new_display_id {
            // When moving from one canvas window to another,
            // the system redraws the previous window to ensure no highlights or other elements remain visible.
            if self.cursor_display_pos.is_none() {
                self.cursor_display_pos = Some(new_display_id.clone());
                return;
            }

            if let Some(display_id) = self.cursor_display_pos.as_mut()
                && display_id != new_display_id
            {
                let old_display_id = std::mem::replace(display_id, new_display_id.clone());

                if let Some(old_canvas) = self.canvases.get(&old_display_id) {
                    cx.notify(old_canvas.entity_id());
                }
            }
        } else if let Some(canvas) = self
            .cursor_display_pos
            .take()
            .and_then(|display_id| self.canvases.get(&display_id))
        {
            // When moving outside the canvas window,
            // redraw the canvas to prevent highlights or other elements from remaining.
            cx.notify(canvas.entity_id());
        }
    }
}
