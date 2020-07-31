pub fn extrat_id_password(set: std::collections::BTreeSet<Vec<String>>) -> (String, String) {
    let id = set.iter().nth(0).unwrap().to_owned()[0].to_string();
    let pswd = set.iter().nth(0).unwrap().to_owned()[2].replace("\\","");

    (id, pswd)
}