mod db;
mod logic;
mod model;
mod ui;

use iced::{Sandbox, Settings};
use ui::Atm;

pub fn main() {
    Atm::run(Settings::default())
}
