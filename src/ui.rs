use iced::{button, text_input, Align, Button, Row, Column, Element, Sandbox, Text};
use crate::db::{create_account, withdraw};
use transistor::client::Crux;

#[derive(Default)]
pub struct Atm {
    value: i64,
    account_info: String,
    login_button: button::State,
    withdraw_button: button::State,
    user_ok_button: button::State,
    create_user_button: button::State,
    state: State
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Login,
    User,
    Menu,
    Cashed,
}

impl Default for State {
    fn default() -> Self { State::Menu}
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    LoginSelected,
    WithdrawSelected,
    CreatingUser,
    UserOk,
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
            Message::UserOk => {
                self.state = State::Menu;
            },
            Message::CreatingUser => {
                let client = Crux::new("localhost","3000").docker_client();
                let account_info = create_account(&client, String::from("naomijub"), 123456u32, 1029384756u32, 300i64).unwrap_or("error".to_string());
                self.account_info = account_info;
                self.state = State::User;
            },
            Message::WithdrawSelected => {
                let client = Crux::new("localhost","3000").docker_client();
                let money = withdraw(&client, 123456u32, 1029384756u32, 50i64).unwrap_or(0i64);
                self.value = money;
                self.state = State::Cashed;
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
                .push(Text::new("User Account Page".to_string()).size(50))
                .push(Text::new(self.account_info.clone()).size(50))
                .push(
                    Button::new(&mut self.user_ok_button, Text::new("Ok"))
                        .on_press(Message::UserOk),
                )
                .into(),
            State::Cashed => Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("Money Cashed Page".to_string()).size(50))
                .push(Text::new(self.value.to_string()).size(50))
                .push(
                    Button::new(&mut self.user_ok_button, Text::new("Ok"))
                        .on_press(Message::UserOk),
                )
                .into(),
            State::Menu => Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.login_button, Text::new("Login"))
                        .on_press(Message::LoginSelected),
                )
                .push(
                    Button::new(&mut self.withdraw_button, Text::new("Withdraw"))
                        .on_press(Message::WithdrawSelected),
                )
                .into()
        }
    }
}
