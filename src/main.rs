mod ui;
mod db;
mod model;
mod logic;

use ui::Atm;
use iced::{Settings, Sandbox};


pub fn main() {
    Atm::run(Settings::default())
}
