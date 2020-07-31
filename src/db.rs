use transistor::client::Crux;
use transistor::types::{error::CruxError, CruxId, query::Query};
use transistor::docker::{Action, DockerClient};

use transistor::edn_rs::{Edn, ser_struct, Serialize};
use bcrypt::{DEFAULT_COST, hash, verify};


pub fn db_main() -> Result<(), CruxError> {
    let client = Crux::new("localhost","3000").docker_client();
    create_account(&client, String::from("naomijub"), 123456u32, 1029384756u32, 300i64)?;

    let money = withdraw(&client, 123456u32, 1029384756u32, 50i64)?;

    println!("{:?}", money);

    Ok(())
}

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

pub fn withdraw(client: &DockerClient, account: u32, password: u32, amount: i64) -> Result<i64, CruxError>{

    let query = Query::find(vec!["?user","?p"])?.where_clause(vec!["?user :account ?a", "?user :password ?p"])?
        .args(vec![&format!("?a {}", account)])?
        .build()?;

    let resp = client.query(query)?;
    let key = resp.iter().nth(0).unwrap().to_owned()[0].replace(":", "");
    let _password = resp.iter().nth(0).unwrap().to_owned()[2].clone().replace("\\","");

    let verifieded_pswd = verify(format!("{}", password), &_password).unwrap();

    let account = Account::new(key.clone(), amount);
    let entity = client.entity(account.clone().crux__db___id.serialize()).unwrap();
    let account = Account::from(entity);
    match (verifieded_pswd, account.value > amount) {
        (true, true) => {
            let tx_account = Account::new(key, account.value - amount);
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




