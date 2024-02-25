struct Foo<'a> {
    value1: &'a str,
    value2: &'a str
}

fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a Foo<'a>, y: &'a Foo<'a>) -> &'a str {
    let mut ret = x.value1;

    if x.value2.len() > ret.len() {
        ret = x.value2;
    }

    if y.value1.len() > ret.len() {
        ret = y.value1;
    }

    if y.value2.len() > ret.len() {
        ret = y.value2;
    }

    ret
}

fn main() {
    println!("{}", shortest("123", "4567"));

    let str1 = String::from("abc");
    let ret;
    {
        let str2 = String::from("defg");
        let str3 = String::from("hijkl");
        let str4 = String::from("mnopqr");

        let x = Foo { value1: str1.as_str(), value2: str2.as_str() };
        let y = Foo { value1: str3.as_str(), value2: str4.as_str() };

        ret = longest(&x, &y);
        println!("{}", ret);
    }

    // println!("{}", ret); // NG
}
