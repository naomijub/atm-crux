use iced::{
    Align, Button, Column, Element, HorizontalAlignment, Length, Row, Space, Text,
};

use super::{Atm, Message};
pub struct Menu;

impl Menu {
    pub fn view(atm: &mut Atm) -> Element<Message> {
        Row::new()
            .padding(100)
            .align_items(Align::Center)
            .push(
                Button::new(
                    &mut atm.login_button,
                    Text::new("Login").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::LoginSelected)
                .padding(20)
                .width(Length::Units(150)),
            )
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(
                        Button::new(
                            &mut atm.withdraw_button,
                            Text::new("Withdraw").horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .on_press(Message::WithdrawSelected)
                        .padding(20)
                        .width(Length::Units(150)),
                    )
                    .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
                    .push(
                        Button::new(
                            &mut atm.deposit_button,
                            Text::new("Deposit").horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .on_press(Message::DepositSelected)
                        .padding(20)
                        .width(Length::Units(150)),
                    )
                    .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
                    .push(
                        Button::new(
                            &mut atm.statement_button,
                            Text::new("Statement")
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .on_press(Message::StatementSelected)
                        .padding(20)
                        .width(Length::Units(150)),
                    ),
            )
            .into()
    }
}
