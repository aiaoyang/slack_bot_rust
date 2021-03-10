use std::collections::HashMap;

use openldap::*;

pub fn get_users(url: &str, base_dn: &str) -> HashMap<String, String> {
    ldap_users(url, base_dn)
}

fn ldap_users(url: &str, base_dn: &str) -> HashMap<String, String> {
    let uri = format!("ldap://{}", url);
    let ldap_c = RustLDAP::new(&uri).unwrap();
    ldap_c.set_option(
        codes::options::LDAP_OPT_PROTOCOL_VERSION,
        &codes::versions::LDAP_VERSION2,
    );

    let res = ldap_c
        .simple_search(base_dn, codes::scopes::LDAP_SCOPE_SUB)
        .unwrap();
    let users_db = res
        .into_iter()
        .map(|item| {
            let mut hm: HashMap<String, String> = HashMap::new();
            let mut display_name: String = String::new();
            let mut user_name: String = String::new();

            for (k, v) in item {
                if k == "displayName" {
                    display_name = v.get(0).unwrap().to_owned();
                }
                if k == "userName" {
                    user_name = v.get(0).unwrap().to_owned();
                }
            }
            hm.insert(user_name, display_name);
            hm
        })
        .collect::<Vec<HashMap<String, String>>>();

    // println!("{:?}", &users_db);
    let mut db: HashMap<String, String> = HashMap::new();

    for item in users_db.into_iter() {
        for (user_name, display_name) in item {
            db.insert(user_name, display_name);
        }
    }

    db
}

#[test]
fn test() {
    ldap_users("", "");
}
