use crate::commands::encode_from_url;
use iced::theme::Theme;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, progress_bar, radio, row, scrollable,
    slider, text, text_input, toggler, vertical_rule, vertical_space,
};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Gui::run(Settings::default())
}

#[derive(Default)]
pub struct Gui {
    theme: Theme,
    url: String,
    msg: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    MsgChanged(String),
    EncodePressed,
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
            Message::UrlChanged(value) => self.url = value,
            Message::MsgChanged(value) => self.msg = value,
            Message::EncodePressed => {
                encode_from_url(&self.url, "png_tests/guitest.png", "rust", &self.msg)
                    .expect("Failed to encode from url");
                println!("Encoded successfully");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let url_input = text_input("Enter URL here...", &self.url, Message::UrlChanged)
            .padding(10)
            .size(20);

        let msg_input = text_input("Enter message here...", &self.msg, Message::MsgChanged)
            .padding(10)
            .size(20);

        let button = button("Encode")
            .padding(10)
            .on_press(Message::EncodePressed);

        let content = column![url_input, msg_input, button]
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
