//! Virtual keyboard for touch screens.

use egui::{
    vec2, Align2, Button, Context, Event, Frame, Id, Modifiers, Order, Ui, Vec2, WidgetText, Window,
};
use std::collections::VecDeque;

enum Key {
    Text(&'static str),
    Backspace,
    Upper,
}

#[derive(Default)]
pub struct Keyboard {
    input_widget: Option<Id>,
    events: VecDeque<Event>,
    upper: bool,

    /// How much keyboard is needed. It's a number so we can implement this as some sort of
    /// hysteresis to avoid flickering.
    needed: u32,
}

fn button(text: &str) -> Button {
    let text = WidgetText::from(text).heading();
    Button::new(text).min_size(Vec2::new(10., 40.))
}

mod layouts {
    use super::Key;

    pub fn qwerty() -> Vec<Vec<Key>> {
        vec![
            vec![
                Key::Text("1"),
                Key::Text("2"),
                Key::Text("3"),
                Key::Text("4"),
                Key::Text("5"),
                Key::Text("6"),
                Key::Text("7"),
                Key::Text("8"),
                Key::Text("9"),
                Key::Text("0"),
            ],
            vec![
                Key::Text("q"),
                Key::Text("w"),
                Key::Text("e"),
                Key::Text("r"),
                Key::Text("t"),
                Key::Text("y"),
                Key::Text("u"),
                Key::Text("i"),
                Key::Text("o"),
                Key::Text("p"),
            ],
            vec![
                Key::Text("a"),
                Key::Text("s"),
                Key::Text("d"),
                Key::Text("f"),
                Key::Text("g"),
                Key::Text("h"),
                Key::Text("j"),
                Key::Text("k"),
                Key::Text("l"),
                Key::Text(":"),
            ],
            vec![
                Key::Upper,
                Key::Text("z"),
                Key::Text("x"),
                Key::Text("c"),
                Key::Text("v"),
                Key::Text("b"),
                Key::Text("n"),
                Key::Text("m"),
                Key::Text(","),
                Key::Text("."),
                Key::Text("/"),
                Key::Backspace,
            ],
            vec![Key::Text(" ")],
        ]
    }

    pub fn qwerty_upper() -> Vec<Vec<Key>> {
        vec![
            vec![
                Key::Text("!"),
                Key::Text("@"),
                Key::Text("#"),
                Key::Text("$"),
                Key::Text("%"),
                Key::Text("^"),
                Key::Text("&"),
                Key::Text("*"),
                Key::Text("("),
                Key::Text(")"),
            ],
            vec![
                Key::Text("Q"),
                Key::Text("W"),
                Key::Text("E"),
                Key::Text("R"),
                Key::Text("T"),
                Key::Text("Y"),
                Key::Text("U"),
                Key::Text("I"),
                Key::Text("O"),
                Key::Text("P"),
            ],
            vec![
                Key::Text("A"),
                Key::Text("S"),
                Key::Text("D"),
                Key::Text("F"),
                Key::Text("G"),
                Key::Text("H"),
                Key::Text("J"),
                Key::Text("K"),
                Key::Text("L"),
                Key::Text(";"),
            ],
            vec![
                Key::Upper,
                Key::Text("Z"),
                Key::Text("X"),
                Key::Text("C"),
                Key::Text("V"),
                Key::Text("B"),
                Key::Text("N"),
                Key::Text("M"),
                Key::Text("<"),
                Key::Text(">"),
                Key::Text("?"),
                Key::Backspace,
            ],
            vec![Key::Text(" ")],
        ]
    }
}

impl Keyboard {
    /// Inject text events into Egui context. This function needs to be called before any widget is
    /// created, otherwise the key presses will be ignored.
    pub fn pump_events(&mut self, ctx: &Context) {
        ctx.input_mut(|input| input.events.extend(std::mem::take(&mut self.events)));
    }

    /// Shows the virtual keyboard if needed.
    pub fn show(&mut self, ctx: &Context) {
        self.remember_input_widget(ctx);

        if self.keyboard_input_needed(ctx) {
            let response = Window::new("Keyboard")
                .frame(Frame::NONE)
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(Align2::CENTER_BOTTOM, [0., -5.])
                .fixed_size(vec2(ctx.available_rect().width() - 10., 200.))
                .order(Order::Foreground)
                .show(ctx, |ui| {
                    // We do not want any spacing between the keys.
                    ui.style_mut().spacing.item_spacing = Vec2::ZERO;

                    #[cfg(target_os = "android")]
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

    #[cfg(target_os = "android")]
    fn clipboard_button(&mut self, ui: &mut Ui) {
        if let Ok(text) = android_clipboard::get_text() {
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
#[cfg(target_os = "android")]
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
