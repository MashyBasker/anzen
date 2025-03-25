#![allow(unused)]

use chrono::offset::Utc;
use chrono::DateTime;
use gpgme::{Context, Protocol};

#[derive(Debug, Clone)]
pub struct GPGKey {
    pub id: String,
    fingerprint: String,
    pub username: String,
    pub email: Option<String>,
    pub creation_date: Option<String>,
    pub expiry_date: Option<String>,
    pub can_sign: bool,
    can_encrypt: bool,
    pub algorithm: String,
}

pub fn get_keys() -> Result<Vec<GPGKey>, Box<dyn std::error::Error>> {
    let mut key_list: Vec<GPGKey> = Vec::new();
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;

    for key in ctx.keys()? {
        let key = key?;
        let primary_subkey = key.primary_key().unwrap();
        let mut gpg_key = GPGKey {
            id: key.id().unwrap_or("unknown").to_string(),
            fingerprint: key.fingerprint().map(|f| f.to_string()).unwrap(),
            email: key
                .user_ids()
                .next()
                .unwrap()
                .email()
                .unwrap()
                .to_string()
                .try_into()?,
            username: key
                .user_ids()
                .next()
                .unwrap()
                .name()
                .unwrap()
                .to_string()
                .try_into()?,
            creation_date: None,
            expiry_date: None,
            can_sign: primary_subkey.can_sign(),
            can_encrypt: primary_subkey.can_encrypt(),
            algorithm: primary_subkey.algorithm_name().unwrap(),
        };

        let creation_time = primary_subkey.creation_time().unwrap();
        let creation_date_time: DateTime<Utc> = creation_time.into();
        let creation_dt = creation_date_time.format("%d/%m/%Y %T");
        let mut cdate = String::new();
        let _ = creation_dt.write_to(&mut cdate);
        gpg_key.creation_date = Some(cdate);

        let expiry_date = primary_subkey.expiration_time().unwrap();
        let expiry_date_time: DateTime<Utc> = expiry_date.into();
        let expiry_dt = expiry_date_time.format("%d/%m/%Y %T");
        let mut edate = String::new();
        let _ = expiry_dt.write_to(&mut edate);
        gpg_key.expiry_date = Some(edate);

        key_list.push(gpg_key);
    }

    Ok(key_list)
}
