
use transistor::types::{CruxId, Action};

use bcrypt::{hash, DEFAULT_COST};
use transistor::edn_rs::{ser_struct, Edn, Serialize};

ser_struct! {
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    pub struct User {
        crux__db___id: CruxId,
        account: u32,
        password: String,
    }
}

ser_struct! {
    #[derive(Debug, Clone, PartialEq)]
    #[allow(non_snake_case)]
    pub struct Account {
        crux__db___id: CruxId,
        value: i64,
        transact_value: i64,
        transaction_type: Transaction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Transaction {
    CreateAccount,
    Withdraw,
    Deposit,
    Transfer,
    Error,
}

impl Serialize for Transaction {
    fn serialize(self) -> std::string::String {
        match self {
            Transaction::CreateAccount => String::from(":transaction/create-account"),
            Transaction::Withdraw => String::from(":transaction/withdraw"),
            Transaction::Deposit => String::from(":transaction/deposit"),
            Transaction::Transfer => String::from(":transaction/transfer"),
            Transaction::Error => String::from(":error"),
        }
    }
}

impl From<Edn> for Transaction {
    fn from(edn: Edn) -> Self {
        match edn {
            Edn::Key(k) => match &k[..] {
                ":transaction/create-account" => Transaction::CreateAccount,
                ":transaction/withdraw" => Transaction::Withdraw,
                ":transaction/deposit" => Transaction::Deposit,
                ":transaction/transfer" => Transaction::Transfer,
                _ => Transaction::Error,
            },
            _ => Transaction::Error,
        }
    }
}

impl From<Edn> for Account {
    fn from(edn: Edn) -> Self {
        Self {
            crux__db___id: CruxId::new(&edn[":crux.db/id"].to_string()),
            value: edn[":value"].to_string().parse::<i64>().unwrap(),
            transact_value: edn[":transact-value"].to_string().parse::<i64>().unwrap(),
            transaction_type: Transaction::from(edn[":transaction-type"].clone()),
        }
    }
}

impl User {
    pub fn new(user: String, account: u32, password: u32) -> Self {
        let hashed_pswd = hash(password.to_string(), DEFAULT_COST).unwrap();

        User {
            crux__db___id: CruxId::new(&user),
            account: account,
            password: hashed_pswd.to_string(),
        }
    }

    pub fn register(self) -> Vec<Action> {
        let action = Action::Put(self.serialize(), None);
        vec![action]
    }
}

impl Account {
    pub fn new(user: String, amount: i64) -> Account {
        Self {
            crux__db___id: CruxId::new(&format!("transaction/{}", user)),
            value: amount,
            transact_value: 0,
            transaction_type: Transaction::CreateAccount,
        }
    }

    pub fn account_id(user: String) -> CruxId {
        CruxId::new(&format!("transaction/{}", user))
    }

    pub fn transact(mut self, transaction: Transaction) -> Vec<Action> {
        self.transaction_type = transaction;
        let action = Action::Put(self.serialize(), None);
        vec![action]
    }
}

#[cfg(test)]
mod user {
    use super::User;
    use transistor::types::CruxId;

    #[test]
    fn new_user() {
        let user = User::new("naomijub".to_string(), 123456u32, 1029384756u32);

        assert_eq!(user.crux__db___id, CruxId::new("naomijub"));
    }

    #[test]
    fn register_user() {
        let user = User::new("naomijub".to_string(), 123456u32, 1029384756u32);

        assert!(user.register().get(0).is_some());
    }
}

#[cfg(test)]
mod account {
    use super::{Account, Transaction};
    use transistor::edn_rs::Edn;
    use transistor::types::CruxId;

    #[test]
    fn new_account() {
        let account = Account::new("naomijub".to_string(), 300i64);
        let expected = Account {
            crux__db___id: CruxId::new("transaction/naomijub"),
            value: 300i64,
            transact_value: 0,
            transaction_type: Transaction::CreateAccount,
        };

        assert_eq!(account, expected);
    }

    #[test]
    fn account_id_value() {
        let id = Account::account_id("naomijub".to_string());
        let expected = CruxId::new("transaction/naomijub");

        assert_eq!(id, expected);
    }

    #[test]
    fn transact_from() {
        assert_eq!(
            Transaction::from(Edn::Key(":transaction/create-account".to_string())),
            Transaction::CreateAccount
        );
        assert_eq!(
            Transaction::from(Edn::Key(":bleb".to_string())),
            Transaction::Error
        );
        assert_eq!(
            Transaction::from(Edn::Key(":transaction/transfer".to_string())),
            Transaction::Transfer
        );
        assert_eq!(
            Transaction::from(Edn::Key(":transaction/withdraw".to_string())),
            Transaction::Withdraw
        );
        assert_eq!(
            Transaction::from(Edn::Key(":transaction/deposit".to_string())),
            Transaction::Deposit
        );
    }
}


pub struct StatementElement {
    value: isize,
    tx_type: String,
    balance: usize,
}

impl From<Edn> for StatementElement {
    fn from(edn: Edn) -> Self { 
        Self {
            value: edn[":transact-value"].to_int().unwrap_or(0isize),
            tx_type: edn[":transaction-type"].to_string(),
            balance: edn[":value"].to_uint().unwrap_or(0usize),
        }
    }
}

impl StatementElement {
    pub fn to_string(&self) -> String {
        let tx_name = match &self.tx_type[..] {
            ":transaction/withdraw" => "Withdraw",
            ":transaction/deposit" => "Deposit",
            _ => "Account Creation"
        };

        format!("Transaction: {}, Value: {}. Balance at TX: {}", tx_name, self.value, self.balance)
    }
}