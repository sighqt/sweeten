//! Demonstrates the enhanced button widget with focus/blur messages.
//!
//! This example shows:
//! - `on_focus(Message)` - emit a message when the button gains focus
//! - `on_blur(Message)` - emit a message when the button loses focus
//! - `on_press(Message)` - emit a message when clicked/activated
//! - Tab / Shift+Tab navigation between focus target buttons
//!
//! Run with: `cargo run --example button`

use iced::{
    Element, Fill, Subscription, Task, keyboard,
    widget::{Id, column, container, operation, text},
};

use sweeten::focusable_button;

#[derive(Debug, Clone)]
enum Message {
    Focus(Target),
    Blur(Target),
    Incremented,
    Decremented,
    Reset,
    TabPressed { shift: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    IncrementButton,
    DecrementButton,
    ResetButton,
}

struct App {
    focused: bool,
    count: i32,
    last_event: &'static str,
    focused_target: Target,
}

impl Default for App {
    fn default() -> Self {
        App {
            focused: false,
            count: 0,
            last_event: "none",
            focused_target: Target::IncrementButton,
        }
    }
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let label = if self.focused {
            "Focused"
        } else {
            "Not focused"
        };

        let reset = focusable_button(text("Reset"))
            .id(reset_id())
            .on_press(Message::Reset)
            .on_focus(Message::Focus(Target::ResetButton))
            .on_blur(Message::Blur(Target::ResetButton));

        let increment = focusable_button(text("+ 1"))
            .id(inc_id())
            .on_press(Message::Incremented)
            .on_focus(Message::Focus(Target::IncrementButton))
            .on_blur(Message::Blur(Target::IncrementButton));

        let decrement = focusable_button(text("- 1"))
            .id(dec_id())
            .on_press(Message::Decremented)
            .on_focus(Message::Focus(Target::DecrementButton))
            .on_blur(Message::Blur(Target::DecrementButton));

        container(
            column![
                text(format!("Focus state: {label}")),
                text(format!("Press count: {}", self.count)),
                text(format!("Last event: {}", self.last_event)),
                increment,
                decrement,
                reset,
            ]
            .spacing(12),
        )
        .center(Fill)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabPressed { shift } => {
                self.focused_target = next_target(self.focused_target, shift);
                self.last_event = "tabbed";

                return operation::focus(target_id(self.focused_target));
            }
            Message::Focus(t) => {
                self.focused_target = t;
                self.last_event = "focused";
            }
            Message::Blur(_t) => {
                self.last_event = "blurred";
            }
            Message::Incremented => {
                self.count += 1;
                self.last_event = "pressed";
            }
            Message::Decremented => {
                self.count -= 1;
                self.last_event = "pressed";
            }
            Message::Reset => {
                self.count = 0;
                self.last_event = "reset";
            }
        }
        Task::none()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("Sweeten - focusable button")
        .subscription(subscription)
        .centered()
        .run()
}

fn update(app: &mut App, message: Message) -> iced::Task<Message> {
    app.update(message)
}

fn view(app: &App) -> iced::Element<'_, Message> {
    app.view()
}

fn subscription(_app: &App) -> Subscription<Message> {
    keyboard::on_key_press(|key, modifiers| match key {
        keyboard::Key::Named(keyboard::key::Named::Tab) => {
            Some(Message::TabPressed {
                shift: modifiers.shift(),
            })
        }
        _ => None,
    })
}

fn dec_id() -> Id {
    Id::new("dec")
}

fn inc_id() -> Id {
    Id::new("inc")
}

fn reset_id() -> Id {
    Id::new("reset")
}

fn next_target(current: Target, shift: bool) -> Target {
    match (current, shift) {
        (Target::IncrementButton, false) => Target::DecrementButton,
        (Target::DecrementButton, false) => Target::ResetButton,
        (Target::ResetButton, false) => Target::IncrementButton,

        (Target::IncrementButton, true) => Target::ResetButton,
        (Target::DecrementButton, true) => Target::IncrementButton,
        (Target::ResetButton, true) => Target::DecrementButton,
    }
}

fn target_id(target: Target) -> Id {
    match target {
        Target::IncrementButton => inc_id(),
        Target::DecrementButton => dec_id(),
        Target::ResetButton => reset_id(),
    }
}
