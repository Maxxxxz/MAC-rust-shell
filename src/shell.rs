use std::io::{self, Write};
use std::collections::HashMap;

// crashes when line is empty

pub fn run() -> bool
{

    // hashmap<str, fn()>

    let cmd: HashMap<&str, fn() -> ()>;

    let mut cmds: Vec<&str> = vec!["help", "quit"];
    // let user prefs = prefs
    // pass prefs and cmds into init

    init(&mut cmds);

    loop
    {
        let msg = getMessage();

        if msg == "quit"
        {
            break;
        }

        let _c = parseMessage(msg);

    }

    return true;

}

fn init(cmds: &mut Vec<&str>)// -> bool // some way to tell run() that init failed
{
    // run initialization code here
    // i.e. grab user prefs
}

fn getMessage() -> String
{
    print!("(\")> ");
    io::stdout().flush().unwrap();

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to get input!");
    let mut msg = inp;
    if msg.len() != 0
    {
        msg = msg[..msg.len()-1].to_string();
        if msg.ends_with('\r') // Catch windows \r\n
        {
            msg = msg[..msg.len()-1].to_string();
        }
    }

    return msg;

}

fn parseMessage(msg: String)// -> Option? Something to tell the loop what went wrong
{
    // split message by whitespace and parse via first command
    if msg.is_empty()
    {
        return; //return option, user probably just wants to clear the terminal
    }
    
    let inVec: Vec<&str> = msg.split_whitespace().collect();

    match inVec[0]
    {
        "help" => println!("this is help penguin <(\")"),
        _ => println!("Unknown or invalid command."),
    }
}