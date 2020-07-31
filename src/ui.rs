use iced::{button, text_input, Align, Button, Row, Column, Element, Sandbox, Text};
use crate::db::create_account;
use transistor::client::Crux;

#[derive(Default)]
pub struct Atm {
    value: i64,
    user: String,
    login_button: button::State,
    user_ok_button: button::State,
    create_user_button: button::State,
    state: State
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Login,
    User,
    Menu,
}

impl Default for State {
    fn default() -> Self { State::Menu}
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    LoginSelected,
    CreatingUser,
    UserCreated,
}

impl Sandbox for Atm {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Atm with Crux")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoginSelected => {
                self.state = State::Login;
            },
            Message::UserCreated => {
                self.state = State::Menu;
            },
            Message::CreatingUser => {
                let client = Crux::new("localhost","3000").docker_client();
                let user = create_account(&client, String::from("naomijub"), 123456u32, 1029384756u32, 300i64).unwrap_or("error".to_string());
                self.user = user;
                self.state = State::User;
            }

        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.state {
            State::Login => Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("Login Page".to_string()).size(50))
                .push(
                    Button::new(&mut self.create_user_button, Text::new("New User"))
                        .on_press(Message::CreatingUser),
                )
                .into(),
            State::User => Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("User Id Page".to_string()).size(50))
                .push(Text::new(self.user.clone()).size(50))
                .push(
                    Button::new(&mut self.user_ok_button, Text::new("Ok"))
                        .on_press(Message::UserCreated),
                )
                .into(),
            State::Menu => Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.login_button, Text::new("Login"))
                        .on_press(Message::LoginSelected),
                )
                .into()
        }
    }
}
