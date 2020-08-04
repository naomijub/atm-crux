pub fn extract_id_password(set: std::collections::BTreeSet<Vec<String>>) -> (String, String) {
    let id = set.iter().nth(0).unwrap().to_owned()[0].to_string();
    let pswd = set.iter().nth(0).unwrap().to_owned()[2].replace("\\", "");

    (id, pswd)
}

#[cfg(test)]
mod logic {
    use super::*;
    use std::collections::BTreeSet;

    fn set() -> BTreeSet<Vec<String>> {
        let mut s = BTreeSet::new();
        s.insert(vec![
            ":my-id".to_string(),
            "\\".to_string(),
            "this is a fucking long password\\".to_string(),
        ]);
        s
    }

    #[test]
    fn id_pswd_tuple() {
        let set = set();
        let (id, pswd) = extract_id_password(set);

        assert_eq!(id, ":my-id".to_string());
        assert_eq!(pswd, "this is a fucking long password".to_string());
    }
}
