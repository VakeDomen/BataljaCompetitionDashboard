use ldap3::{Scope, SearchEntry, LdapConnAsync};
use ldap3::result::Result;
use std::env;

pub async fn ldap_login(username: i32, password: String) -> Result<Option<String>> {
    let (conn, mut ldap_conn) = LdapConnAsync::new(&env::var("LDAP_SERVER").expect("$LDAP_SERVER is not set")).await?;
    ldap3::drive!(conn);

    // Search for the user in the directory
    let (rs, _res) = ldap_conn.search(
        "dc=upr,dc=si",
        Scope::Subtree,
        format!("(uid={})", username).as_str(),
        vec!["dn", "sn", "cn"]
    ).await?.success()?;

    let mut user_entry = None;
    // there should only be one entry in the array or results
    for entry in rs {
        let entry = SearchEntry::construct(entry);
        match ldap_conn.simple_bind(&entry.dn, &password).await {
            /*
                LdapError has a variant called RC(u32, String). This variant 
                represents an error with a specific error code (rc) returned 
                by the LDAP server.
                The error code (rc) is a numerical value that indicates the 
                type of error that occurred. LDAP defines a set of standard 
                error codes that can be used to indicate different types of 
                errors. For example, error code 49 is used to indicate that 
                the provided credentials (username or password) are invalid
            */
            Ok(r) => if r.rc == 0 { user_entry = Some(entry.dn); },
            Err(e) => println!("Error binding to ldap: {:?}", e)
        }
    }
    Ok(user_entry)
}