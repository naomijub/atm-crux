use iced::{Align, Button, Column, Element, Row, Text, TextInput, VerticalAlignment};

use super::{Atm, Message};

pub struct Operation;

impl Operation {
    pub fn view(atm: &mut Atm, msg: Message) -> Element<Message> {
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

        let operation_input = TextInput::new(
            &mut atm.operation_input,
            "Amount",
            &atm.operation_value,
            Message::OperationInputChanged,
        )
        .padding(10)
        .size(20);

        Column::new()
            .align_items(Align::Center)
            .push(Text::new(match msg {
               Message::Deposited =>  "Deposit Input Page".to_string(),
               _                  =>  "Withdraw Input Page".to_string(),
            }).size(50))
            .spacing(20)
            .padding(25)
            .push(
                Column::new()
                    .spacing(10)
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(
                                Text::new("Account")
                                    .size(30)
                                    .vertical_alignment(VerticalAlignment::Center),
                            )
                            .push(account_input),
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
                            .push(password_input),
                    )
                    .push(
                        Row::new()
                            .spacing(10)
                            .align_items(Align::Start)
                            .push(
                                Text::new("Amount")
                                    .size(30)
                                    .vertical_alignment(VerticalAlignment::Center),
                            )
                            .push(operation_input),
                    ),
            )
            .push(
                Button::new(&mut atm.confirm_button, Text::new("Confirmar"))
                    .padding(25)
                    .on_press(msg),
            )
            .into()
    }
}
