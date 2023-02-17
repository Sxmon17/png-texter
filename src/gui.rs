use iced::theme::{self, Theme};
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, progress_bar, radio,
    row, scrollable, slider, text, text_input, toggler, vertical_rule,
    vertical_space,
};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};
use crate::commands::encode_from_url;

pub fn main() -> iced::Result {
    Gui::run(Settings::default())
}

#[derive(Default)]
pub struct Gui {
    theme: Theme,
    input_value: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ButtonPressed,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        Gui::default()
    }

    fn title(&self) -> String {
        String::from("png-texter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {
                encode_from_url(
                    &self.input_value,
                    "png_tests/guitest.png",
                    "rust",
                    "Hello world!",
                ).expect("Failed to encode from url");
                println!("Encoded successfully");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let text_input = text_input(
            "Enter URL here...",
            &self.input_value,
            Message::InputChanged,
        )
            .padding(10)
            .size(20);

        let button = button("Submit")
            .padding(10)
            .on_press(Message::ButtonPressed);

        let content = column![
            row![text_input, button].spacing(10),
            horizontal_rule(38)
        ]
            .spacing(20)
            .padding(20)
            .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}