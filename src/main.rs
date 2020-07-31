mod ui;
mod db;
mod model;
mod logic;

use ui::Atm;
use db::db_main;
use iced::{Settings, Sandbox};


pub fn main() {
    // db_main().unwrap();
    Atm::run(Settings::default())
}
