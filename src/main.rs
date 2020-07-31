mod ui;
mod db;

// use ui::Atm;
use db::db_main;

pub fn main() {
    db_main().unwrap();
    // Atm::run(Settings::default())
}
