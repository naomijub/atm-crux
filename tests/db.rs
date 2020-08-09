use atm::db::create_account;
use mockito::mock;
use transistor::client::Crux;

#[test]
fn account_already_exists() {
    let _m = mock("POST", "/query")
        .with_status(200)
        .with_header("content-type", "application/edn")
        .with_body("#{[:user]}")
        .create();
    let mut crux = Crux::new("localhost", "3000");

    let account = create_account(
        &crux.http_mock(),
        String::from("test"),
        1234u32,
        162534u32,
        30i64,
    )
    .unwrap();

    assert_eq!(account, "Account already exists");
}

#[test]
fn account_created_with_id() {
    let _m_query = mock("POST", "/query")
        .with_status(200)
        .with_header("content-type", "application/edn")
        .with_body("#{}")
        .create();

    let _m_tx_log1 = mock("POST", "/tx-log")
        .with_status(200)
        .expect_at_least(2)
        .with_header("content-type", "application/edn")
        .with_body("{:crux.tx/tx-id 8, :crux.tx/tx-time #inst \"2020-07-16T21:53:14.628-00:00\"}")
        .create();

    let mut crux = Crux::new("localhost", "3000");

    let account = create_account(
        &crux.http_mock(),
        String::from("thoho-est"),
        221234u32,
        162534u32,
        30i64,
    )
    .unwrap();

    assert_eq!(account, "221234");
}
