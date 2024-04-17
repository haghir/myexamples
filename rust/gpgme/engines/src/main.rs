fn main() {
    let gpgme = gpgme::init();

    for ei in &gpgme.engine_info().unwrap() {
        println!("{:?}", ei);
    }
}
