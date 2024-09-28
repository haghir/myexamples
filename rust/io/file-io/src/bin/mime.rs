use mime_guess;

fn guess_mime_from_path(path: &str) {
    let guess = mime_guess::from_path(path);
    if let Some(guessed) = guess.first() {
        println!("{}", guessed);
    } else {
        println!("failed to guess");
    }
}

fn main() {
    guess_mime_from_path("/a/b/c.d.jpg");
    guess_mime_from_path("/a/b/c.d.jpeg");
    guess_mime_from_path("/a/b/c.d.png");
    guess_mime_from_path("/a/b/c.d.gif");
    guess_mime_from_path("/a/b/c.d.bmp");
    guess_mime_from_path("/a/b/c.d.mp3");
    guess_mime_from_path("/a/b/c.d.mp4");
}
