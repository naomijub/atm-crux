use transistor::types::{CruxId};
use transistor::docker::{Action,};

use transistor::edn_rs::{Edn, ser_struct, Serialize};
use bcrypt::{DEFAULT_COST, hash};

ser_struct! {
    #[derive(Debug)]
    #[allow(non_snake_case)]
    pub struct Login {
        crux__db___id: CruxId, 
        account: u32,
        password: String,
    }
}

ser_struct! {
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    pub struct Account {
        crux__db___id: CruxId, 
        value: i64,
    }
}

impl From<Edn> for Account {
    fn from(edn: Edn) -> Self {
        Self {
            crux__db___id: CruxId::new(&edn[":crux.db/id"].to_string()),
            value: edn[":value"].to_string().parse::<i64>().unwrap(),
        }
    }
}

impl Login {
    pub fn new(user: String, account: u32, password: u32) -> Self {
        let hashed_pswd = hash(format!("{}", password), DEFAULT_COST).unwrap();

        Login {
            crux__db___id: CruxId::new(&user),
            account: account,
            password: format!("{:?}", hashed_pswd),
        }   
    }

    pub fn register(self) -> Vec<Action> {
        let action = Action::Put(self.serialize());
        vec![action]
    }
}

impl Account {
    pub fn new(user: String, amount: i64) -> Account {
        Self {
            crux__db___id: CruxId::new(&format!("transaction/{}", user)),
            value: amount
        }
    }

    pub fn transact(self) -> Vec<Action> {
        let action = Action::Put(self.serialize());
        vec![action]
    }
}
