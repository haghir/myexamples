use std::env;
use std::result::Result;
use ldap3::{LdapConnAsync, Scope, SearchEntry, LdapError};

fn get_uri() -> Result<String, LdapError> {
    match env::var("LDAP_EXAMPLE_URI") {
        Ok(uri) => Ok(uri),
        Err(_) => Err(LdapError::UnrecognizedCriticalExtension(String::from(
            "The environment variable LDAP_EXAMPLE_URI must be set.")))
    }
}

fn get_admin_dn() -> Result<String, LdapError> {
    match env::var("LDAP_EXAMPLE_ADMIN_DN") {
        Ok(dn) => Ok(dn),
        Err(_) => Err(LdapError::UnrecognizedCriticalExtension(String::from(
            "The environment variable LDAP_EXAMPLE_ADMIN_DN must be set.")))
    }
}

fn get_admin_passwd() -> Result<String, LdapError> {
    match env::var("LDAP_EXAMPLE_ADMIN_PASSWD") {
        Ok(passwd) => Ok(passwd),
        Err(_) => Err(LdapError::UnrecognizedCriticalExtension(String::from(
            "The environment variable LDAP_EXAMPLE_ADMIN_PASSWD must be set.")))
    }
}

#[tokio::main]
async fn main() -> ldap3::result::Result<()> {
    let (conn, mut ldap) = LdapConnAsync::new(&get_uri()?).await?;
    ldap3::drive!(conn);

    ldap.simple_bind(
        &get_admin_dn()?,
        &get_admin_passwd()?
    ).await?.success()?;

    let (rs, _res) = ldap.search(
        "ou=people,dc=example,dc=com",
        Scope::Subtree,
        "(objectClass=inetOrgPerson)",
        vec!["cn", "objectClass"]
    ).await?.success()?;

    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry));
    }

    Ok(ldap.unbind().await?)
}
