mod menu;
mod result_pages;
mod login_input;

use iced::{
    button, text_input, Align, Button, Column, Container, Element, Length, Row, Sandbox,
    Text, TextInput, VerticalAlignment,
};

use crate::db::{create_account, deposit, statement, withdraw};
use menu::Menu;
use result_pages as pages;
use login_input as login;
use transistor::client::Crux;

#[derive(Default)]
pub struct Atm {
    value: i64,
    account_info: String,
    login_button: button::State,
    withdraw_button: button::State,
    deposit_button: button::State,
    statement_button: button::State,
    user_ok_button: button::State,
    create_user_button: button::State,
    statement: Vec<String>,
    state: State,

    user_input: text_input::State,
    user_value: String,
    account_input: text_input::State,
    account_value: String,
    password_input: text_input::State,
    password_value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Login,
    User,
    Menu,
    Cashed,
    NewBalance,
    Statement,
}

impl Default for State {
    fn default() -> Self {
        State::Menu
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LoginSelected,
    WithdrawSelected,
    DepositSelected,
    StatementSelected,
    CreatingUser,
    UserOk,

    InputChanged(String),
    AccountInputChanged(String),
    PasswordInputChanged(String),
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
            }
            Message::UserOk => {
                self.state = State::Menu;
            }
            Message::CreatingUser => {
                let client = Crux::new("localhost", "3000").docker_client();
                let account_info = create_account(
                    &client,
                    self.user_value.clone(),
                    self.account_value.clone().parse::<u32>().unwrap_or(0),
                    self.password_value.clone().parse::<u32>().unwrap_or(0),
                    300i64,
                )
                .unwrap_or("error".to_string());
                self.account_info = account_info;
                self.state = State::User;
            }
            Message::WithdrawSelected => {
                let client = Crux::new("localhost", "3000").docker_client();
                let money = withdraw(&client, 123456u32, 1029384756u32, 50i64).unwrap_or(0i64);
                self.value = money;
                self.state = State::Cashed;
            }
            Message::DepositSelected => {
                let client = Crux::new("localhost", "3000").docker_client();
                let money = deposit(&client, 123456u32, 1029384756u32, 100i64).unwrap_or(0i64);
                self.value = money;
                self.state = State::NewBalance;
            }
            Message::StatementSelected => {
                let client = Crux::new("localhost", "3000").docker_client();
                let statement = statement(&client, 123456u32).unwrap_or(Vec::new());
                self.statement = statement;
                self.state = State::Statement;
            },
            Message::InputChanged(user) => self.user_value = user,
            Message::AccountInputChanged(account) => self.account_value = account,
            Message::PasswordInputChanged(pswd) => self.password_value = pswd,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(match self.state {
            State::Login => Column::new().push(login::Login::view(self)),
            State::User => Column::new().push(pages::User::view(self)),
            State::Cashed => Column::new().push(pages::Cashed::view(self)),
            State::NewBalance => Column::new().push(pages::NewBalance::view(self)),
            State::Statement => Column::new().push(pages::Statement::view(self)),
            State::Menu => Column::new()
                .spacing(20)
                .padding(100)
                .align_items(Align::Center)
                .push(Text::new("Naomi Bank ATM").size(50))
                .push(Menu::view(self)),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
