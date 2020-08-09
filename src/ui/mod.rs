mod login_input;
mod menu;
mod operation_input;
mod result_pages;

use iced::{button, text_input, Align, Column, Container, Element, Length, Sandbox, Text};

use crate::db::{create_account, deposit, statement, withdraw};
use login_input as login;
use menu::Menu;
use operation_input as op;
use result_pages as pages;
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
    confirm_button: button::State,
    statement: Vec<String>,
    state: State,

    user_input: text_input::State,
    user_value: String,
    account_input: text_input::State,
    account_value: String,
    password_input: text_input::State,
    password_value: String,
    operation_input: text_input::State,
    operation_value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Login,
    Operation(usize),
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
    Withdrawn,
    Deposited,
    UserOk,

    InputChanged(String),
    AccountInputChanged(String),
    PasswordInputChanged(String),
    OperationInputChanged(String),
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
                self.value = 0;
            }
            Message::CreatingUser => {
                let client = Crux::new("localhost", "3000").http_client();
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
                self.state = State::Operation(0);
            }
            Message::Withdrawn => {
                let client = Crux::new("localhost", "3000").http_client();
                let money = withdraw(
                    &client,
                    self.account_value.clone().parse::<u32>().unwrap_or(0),
                    self.password_value.clone().parse::<u32>().unwrap_or(0),
                    -(self.operation_value.clone().parse::<u32>().unwrap_or(0) as i64),
                )
                .unwrap_or(0i64);
                self.password_value = String::new();
                self.value = money;
                self.state = State::Cashed;
            }
            Message::DepositSelected => {
                self.state = State::Operation(1);
            }
            Message::Deposited => {
                let client = Crux::new("localhost", "3000").http_client();
                let money = deposit(
                    &client,
                    self.account_value.clone().parse::<u32>().unwrap_or(0),
                    self.password_value.clone().parse::<u32>().unwrap_or(0),
                    self.operation_value.clone().parse::<u32>().unwrap_or(0) as i64,
                )
                .unwrap_or(0i64);
                self.password_value = String::new();
                self.value = money;
                self.state = State::NewBalance;
            }
            Message::StatementSelected => {
                let client = Crux::new("localhost", "3000").http_client();
                let statement = statement(&client, self.account_value.clone().parse::<u32>().unwrap_or(0)).unwrap_or(Vec::new());
                self.statement = statement;
                self.state = State::Statement;
            }
            Message::InputChanged(user) => self.user_value = user,
            Message::AccountInputChanged(account) => self.account_value = account,
            Message::PasswordInputChanged(pswd) => self.password_value = pswd,
            Message::OperationInputChanged(v) => self.operation_value = v,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(match self.state {
            State::Login => Column::new().push(login::Login::view(self)),
            State::Operation(v) if v == 0usize => {
                Column::new().push(op::Operation::view(self, Message::Withdrawn))
            }
            State::Operation(_) => {
                Column::new().push(op::Operation::view(self, Message::Deposited))
            }
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
