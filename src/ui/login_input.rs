use iced::{
    Align, Button, Column, Element, Row,
    Text, TextInput, VerticalAlignment,
};

use super::{Atm, Message};

pub struct Login;

impl Login {
    pub fn view(atm: &mut Atm) -> Element<Message> {
        let user_input = TextInput::new(
            &mut atm.user_input,
            "Your username",
            &atm.user_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20);

        let account_input = TextInput::new(
            &mut atm.account_input,
            "Your account",
            &atm.account_value,
            Message::AccountInputChanged,
        )
        .padding(10)
        .size(20);

        let password_input = TextInput::new(
            &mut atm.password_input,
            "Your password",
            &atm.password_value,
            Message::PasswordInputChanged,
        )
        .padding(10)
        .size(20);
    
        Column::new()
            .align_items(Align::Center)
            .push(Text::new("Login Page".to_string()).size(50))
            .spacing(20)
            .padding(25)
            .push(Column::new()
                .spacing(10)
                .push(
                    Row::new()
                    .spacing(10)
                    .push(
                        Text::new("User")
                            .size(30)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .push(user_input)
                    .push(
                        Text::new("Account")
                            .size(30)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .push(account_input)
                )
                .push(
                    Row::new()
                    .spacing(10)
                    .align_items(Align::Start)
                    .push(
                        Text::new("Password")
                            .size(30)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .push(password_input)
                )
            )
            .push(
                Button::new(&mut atm.create_user_button, Text::new("Create User")).padding(25)
                    .on_press(Message::CreatingUser),
            )
            .into()
    }
}