struct Data {
    text: String,
}

impl Data {
    fn new(text: String) -> Data {
        Data {
            text: text
        }
    }
}

fn main() {
    // The variable named boxed will be stored in the stack. This variable
    // holds the instance of the struct Data stored in the heap.
    let boxed = Box::new(Data::new(String::from("boxed data")));
    println!("{}", boxed.text);
}
