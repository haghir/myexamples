use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    // Check whether the option "h" was set.
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return;
    }

    // Get a value passed to the option "o".
    let output = matches.opt_str("o");

    // Get the first free option.
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&args[0], opts);
        return;
    };

    println!("input: {}", input);

    match output {
        Some(content) => println!("output: {}", content),
        None => println!("No output"),
    }
}
