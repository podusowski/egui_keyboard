#![doc = include_str!("../../README.md")]

mod clipboard;
mod layouts;

use egui::{
    vec2, Align2, Button, Context, Event, Frame, Id, Modifiers, Order, Rect, Ui, Vec2, WidgetText,
    Window,
};
use std::collections::VecDeque;

enum Key {
    Text(&'static str),
    Backspace,
    Upper,
}

pub struct Keyboard {
    input_widget: Option<Id>,
    events: VecDeque<Event>,
    upper: bool,

    /// How much keyboard is needed. It's a number so we can implement this as some sort of
    /// hysteresis to avoid flickering.
    needed: u32,

    last_rect: Rect,
}

fn button(text: &str) -> Button {
    let text = WidgetText::from(text).heading();
    Button::new(text).frame(false).min_size(Vec2::new(10., 40.))
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            input_widget: None,
            events: Default::default(),
            upper: false,
            needed: 0,
            last_rect: Rect::NOTHING,
        }
    }

    /// Inject text events into Egui context. This function needs to be called before any widget is
    /// created, otherwise the key presses will be ignored.
    pub fn pump_events(&mut self, ctx: &Context) {
        ctx.input_mut(|input| input.events.extend(std::mem::take(&mut self.events)));
    }

    /// Area which is free from the keyboard. This is useful when you want to constrain a window to
    /// the area which is not covered by the keyboard.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # egui::__run_test_ctx(|ctx| {
    /// # let keyboard = egui_keyboard::Keyboard::default();
    /// egui::Window::new("Hello")
    ///   .constrain_to(keyboard.safe_rect(ctx))
    ///   .show(ctx, |ui| {
    ///      ui.label("it is a window");
    ///   });
    /// # });
    /// ```
    pub fn safe_rect(&self, ctx: &Context) -> Rect {
        let screen_rect = ctx.screen_rect();
        if self.needed == 0 {
            screen_rect
        } else {
            Rect::from_min_max(screen_rect.min, screen_rect.max - vec2(0., 200.))
        }
    }

    /// Shows the virtual keyboard if needed.
    pub fn show(&mut self, ctx: &Context) {
        self.remember_input_widget(ctx);

        if self.keyboard_input_needed(ctx) {
            let response = Window::new("Keyboard")
                .frame(Frame::NONE.fill(ctx.style().visuals.extreme_bg_color))
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(Align2::CENTER_BOTTOM, [0., -5.])
                .fixed_size(vec2(ctx.available_rect().width() - 10., 200.))
                .order(Order::Foreground)
                .show(ctx, |ui| {
                    // We do not want any spacing between the keys.
                    ui.style_mut().spacing.item_spacing = Vec2::ZERO;

                    self.clipboard_button(ui);

                    let layout = if self.upper {
                        layouts::qwerty_upper()
                    } else {
                        layouts::qwerty()
                    };

                    for row in layout.iter() {
                        ui.columns(row.len(), |columns| {
                            for (n, key) in row.iter().enumerate() {
                                let ui = &mut columns[n];
                                ui.vertical_centered_justified(|ui| match key {
                                    Key::Text(text) => self.generic_key(ui, text),
                                    Key::Backspace => self.backspace_key(ui),
                                    Key::Upper => self.upper_layout_key(ui),
                                });
                            }
                        });
                    }
                });

            if let Some(response) = response {
                self.last_rect = response.response.rect;

                if response.response.contains_pointer() {
                    // Make sure Egui still thinks that we need the keyboard in the next frame.
                    self.focus_back_to_input_widget(ctx);
                }
            }

            // Prevent native keyboard from showing up.
            ctx.output_mut(|output| {
                output.ime = None;
            });
        }
    }

    fn clipboard_button(&mut self, ui: &mut Ui) {
        if let Some(text) = clipboard::get_text() {
            self.key(ui, &trim_text(&text, 20), Event::Text(text.to_string()));
            ui.separator();
        }
    }

    /// Remember which widget had focus before the keyboard was shown.
    fn remember_input_widget(&mut self, ctx: &Context) {
        if ctx.wants_keyboard_input() {
            self.input_widget = ctx.memory(|memory| memory.focused());
        }
    }

    /// Focus back to the previously focused widget.
    fn focus_back_to_input_widget(&mut self, ctx: &Context) {
        if let Some(focus) = self.input_widget {
            ctx.memory_mut(|memory| memory.request_focus(focus));
        }
    }

    fn key(&mut self, ui: &mut Ui, text: &str, event: Event) {
        if ui.add(button(text)).clicked() {
            self.events.push_back(event);
            self.focus_back_to_input_widget(ui.ctx());
        }
    }

    fn upper_layout_key(&mut self, ui: &mut Ui) {
        if ui.add(button("⏶")).clicked() {
            self.upper = !self.upper;
            self.focus_back_to_input_widget(ui.ctx());
        }
    }

    fn backspace_key(&mut self, ui: &mut Ui) {
        self.key(
            ui,
            "⏴",
            Event::Key {
                key: egui::Key::Backspace,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
                physical_key: None,
            },
        );
    }

    fn generic_key(&mut self, ui: &mut Ui, text: &str) {
        self.key(ui, text, Event::Text(text.to_string()));
    }

    fn keyboard_input_needed(&mut self, ctx: &Context) -> bool {
        let needed = if ctx.wants_keyboard_input() {
            self.needed = 20;
            true
        } else {
            self.needed = self.needed.saturating_sub(1);
            self.needed > 0
        };

        if needed {
            ctx.request_repaint();
        }

        needed
    }
}

/// Trim the text to the maximum length, and add ellipsis if needed.
fn trim_text(text: &str, max_length: usize) -> String {
    let mut result = String::new();
    for (n, c) in text.chars().enumerate() {
        if n >= max_length {
            result.push('…');
            break;
        }
        result.push(c);
    }
    result
}
