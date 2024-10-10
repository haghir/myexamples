use aes_gcm::{
    aead::{AeadMut, Nonce, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = Aes256Gcm::generate_key(OsRng);
    let mut cipher = Aes256Gcm::new(&key);

    let encrypted = encrypt(&mut cipher, &"Hello, AES!".to_string())?;
    println!("{}", encrypted);

    let plain = decrypt(&mut cipher, &encrypted)?;
    println!("{}", plain);

    Ok(())
}

fn encrypt(cipher: &mut Aes256Gcm, plain: &String) -> Result<String, Box<dyn std::error::Error>> {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let encrypted = match cipher.encrypt(&nonce, plain.as_bytes()) {
        Ok(encrypted) => encrypted,
        Err(e) => return Err(e.to_string().into()),
    };
    let mut ret: Vec<u8> = nonce.into_iter().collect();
    ret.extend(&encrypted);
    let ret = BASE64_STANDARD_NO_PAD.encode(&ret);
    Ok(ret)
}

fn decrypt(cipher: &mut Aes256Gcm, recv: &String) -> Result<String, Box<dyn std::error::Error>> {
    let recv = BASE64_STANDARD_NO_PAD.decode(&recv)?;
    let nonce: Vec<u8> = recv.clone().into_iter().take(12).collect();
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);
    let encrypted: Vec<u8> = recv.into_iter().skip(12).collect();
    let plain = match cipher.decrypt(&nonce, encrypted.as_ref()) {
        Ok(plain) => plain,
        Err(e) => return Err(e.to_string().into()),
    };
    let plain = String::from_utf8(plain)?;
    Ok(plain)
}
