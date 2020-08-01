use iced::{Align, Button, Column, Element, Length, Space, Text};

use super::{Atm, Message};

pub struct User;

impl User {
    pub fn view(atm: &mut Atm) -> Element<Message> {
        Column::new()
            .padding(100)
            .align_items(Align::Center)
            .push(Text::new("User Account Page".to_string()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(Text::new(atm.account_info.clone()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(
                Button::new(&mut atm.user_ok_button, Text::new("Ok").size(30))
                    .padding(20)
                    .on_press(Message::UserOk),
            )
            .into()
    }
}

pub struct Cashed;

impl Cashed {
    pub fn view(atm: &mut Atm) -> Element<Message> {
        Column::new()
            .padding(100)
            .align_items(Align::Center)
            .push(Text::new("Money Cashed Page".to_string()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(Text::new(atm.value.to_string()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(
                Button::new(&mut atm.user_ok_button, Text::new("Ok").size(30))
                    .padding(20)
                    .on_press(Message::UserOk),
            )
            .into()
    }
}

pub struct NewBalance;

impl NewBalance {
    pub fn view(atm: &mut Atm) -> Element<Message> {
        Column::new()
            .padding(100)
            .align_items(Align::Center)
            .push(Text::new("New Balance Page".to_string()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(Text::new(atm.value.to_string()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(
                Button::new(&mut atm.user_ok_button, Text::new("Ok").size(30))
                    .padding(20)
                    .on_press(Message::UserOk),
            )
            .into()
    }
}

pub struct Statement;

impl Statement {
    pub fn view(atm: &mut Atm) -> Element<Message> {
        Column::new()
            .padding(100)
            .align_items(Align::Center)
            .push(Text::new("Bank Statement Page".to_string()).size(50))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(Text::new(atm.statement.join("\n")).size(30))
            .push(Space::new(Length::Units(10u16), Length::Units(10u16)))
            .push(
                Button::new(&mut atm.user_ok_button, Text::new("Ok").size(30))
                    .padding(20)
                    .on_press(Message::UserOk),
            )
            .into()
    }
}
