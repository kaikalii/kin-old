#![allow(unstable_name_collisions)]

mod ast;
mod num;
mod parse;

fn main() {
    color_backtrace::install();

    let input = std::fs::read_to_string("test.noot").unwrap();
    match parse::parse(&input) {
        Ok(items) => {
            println!("{:#?}", items);
            println!("{}", items);
        }
        Err(e) => println!("{}", e),
    }
}
