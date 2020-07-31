use transistor::client::Crux;
use transistor::types::{error::CruxError, query::Query};
use transistor::docker::{DockerClient};
use transistor::edn_rs::{Serialize};


use bcrypt::{verify};

use crate::model::{Login, Account};
use crate::logic::extrat_id_password;


pub fn db_main() -> Result<(), CruxError> {
    let client = Crux::new("localhost","3000").docker_client();
    create_account(&client, String::from("naomijub"), 123456u32, 1029384756u32, 300i64)?;

    let money = withdraw(&client, 123456u32, 1029384756u32, 50i64)?;

    println!("{:?}", money);

    Ok(())
}

pub fn withdraw(client: &DockerClient, account: u32, password: u32, amount: i64) -> Result<i64, CruxError>{

    let query = Query::find(vec!["?user","?p"])?.where_clause(vec!["?user :account ?a", "?user :password ?p"])?
        .args(vec![&format!("?a {}", account)])?
        .build()?;

    let resp = client.query(query)?;
    let (id, pswd) = extrat_id_password(resp);

    let is_password_valid = verify(format!("{}", password), &pswd).unwrap();

    let account = Account::new(id.clone(), amount);
    let entity = client.entity(account.clone().crux__db___id.serialize()).unwrap();
    let account = Account::from(entity);
    match (is_password_valid, account.value > amount) {
        (true, true) => {
            let tx_account = Account::new(id, account.value - amount);
            client.tx_log(tx_account.transact())?;

            Ok(amount)
        },
        _ => Ok(0,)
    }
}

pub fn create_account(client: &DockerClient , user: String, account: u32, password: u32, amount: i64) -> Result<(), CruxError>{
    let login = Login::new(user.clone(), account, password);
    client.tx_log(login.register())?;

    let account = Account::new(user, amount);
    client.tx_log(account.transact())?;

    Ok(())
}



