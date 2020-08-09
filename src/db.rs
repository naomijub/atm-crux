use transistor::edn_rs::Serialize;
use transistor::http::{HttpClient, Order};
use transistor::types::{error::CruxError, query::Query};

use bcrypt::verify;

use crate::logic::extrat_id_password;
use crate::model::{Account, Transaction, User, StatementElement};

pub fn withdraw(
    client: &HttpClient,
    account: u32,
    password: u32,
    amount: i64,
) -> Result<i64, CruxError> {
    let query = Query::find(vec!["?user", "?p"])?
        .where_clause(vec!["?user :account ?a", "?user :password ?p"])?
        .args(vec![&format!("?a {}", account)])?
        .build()?;

    let resp = client.query(query)?;
    let (id, pswd) = extract_id_password(resp);

    let is_password_valid = verify(format!("{}", password), &pswd).unwrap();

    let account_id = Account::account_id(id.clone());
    let entity = client.entity(account_id.serialize()).unwrap();
    let account = Account::from(entity);
    match (is_password_valid, account.value >= amount) {
        (true, true) => {
            let mut tx_account = Account::new(id, account.value - amount);
            tx_account.transact_value = -amount;
            client.tx_log(tx_account.transact(Transaction::Withdraw))?;

            Ok(amount)
        }
        _ => Ok(0),
    }
}

pub fn create_account(
    client: &HttpClient,
    user: String,
    account: u32,
    password: u32,
    amount: i64,
) -> Result<String, CruxError> {
    let query = Query::find(vec!["?user"])?
        .where_clause(vec!["?user :account ?a"])?
        .args(vec![&format!("?a {}", account)])?
        .build()?;

    let resp = client.query(query)?;
    if resp.is_empty() {
        let login = User::new(user.clone(), account, password);
        client.tx_log(login.clone().register())?;

        let account = Account::new(user, amount);
        client.tx_log(account.transact(Transaction::CreateAccount))?;

        Ok(login.account.to_string())
    } else {
        Ok("Account already exists".to_string())
    }
}

pub fn deposit(
    client: &HttpClient,
    account: u32,
    password: u32,
    amount: i64,
) -> Result<i64, CruxError> {
    let query = Query::find(vec!["?user", "?p"])?
        .where_clause(vec!["?user :account ?a", "?user :password ?p"])?
        .args(vec![&format!("?a {}", account)])?
        .build()?;

    let resp = client.query(query)?;
    let (id, pswd) = extract_id_password(resp);

    let is_password_valid = verify(format!("{}", password), &pswd).unwrap();

    let account_id = Account::account_id(id.clone());
    let entity = client.entity(account_id.serialize()).unwrap();
    let account = Account::from(entity);
    match is_password_valid {
        true => {
            let mut tx_account = Account::new(id, account.value + amount);
            tx_account.transact_value = amount;
            client.tx_log(tx_account.transact(Transaction::Deposit))?;

            Ok(account.value + amount)
        }
        false => Ok(0),
    }
}

pub fn statement(client: &HttpClient, account: u32) -> Result<Vec<String>, CruxError> {
    let query = Query::find(vec!["?user", "?p"])?
        .where_clause(vec!["?user :account ?a", "?user :password ?p"])?
        .args(vec![&format!("?a {}", account)])?
        .build()?;

    let resp = client.query(query)?;
    let (id, _) = extract_id_password(resp);
    let account_id = Account::account_id(id.clone());
    Ok(client
        .entity_history(account_id.serialize(), Order::Desc, true)
        .unwrap()
        .history
        .iter()
        .map(|e| StatementElement::from(e.db__doc.clone().unwrap()))
        .map(|e| e.to_string()) 
        .collect::<Vec<String>>())

}
