use std::io::{self, Read as _};
mod binder;
mod binder_tests;
mod parse_tests;
mod parser;

use parser::extract;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let (tasks, _) = extract(&buffer);
    println!("{:?}", tasks);
    Ok(())
}
