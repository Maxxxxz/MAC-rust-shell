mod shell;

fn main() {
    println!("Hello, world!");

    match shell::run()
    {
        true => println!("Exited Successfully!"),
        false => println!("Exited Unsuccessfully! :("),
    }

}
