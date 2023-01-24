use std::io;

fn main() -> io::Result<()> {
    println!("Hello, Phil!");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(())
}