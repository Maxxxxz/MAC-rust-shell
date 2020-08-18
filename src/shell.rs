use std::io::{self, Write};

// crashes when line is empty

pub fn run() -> bool
{

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

fn getMessage() -> String
{
    print!("(\")> ");
    io::stdout().flush().unwrap();

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to get input!");
    let mut msg = inp[..inp.len()-1].to_string();
    if msg.ends_with('\r') // Catch windows \r\n
    {
        msg = msg[..msg.len()-1].to_string();
    }

    return msg;

}

fn parseMessage(msg: String)// -> Option? Something to tell the loop what went wrong
{
    // split message by whitespace and parse via first command
    let inVec: Vec<&str> = msg.split_whitespace().collect();

    match inVec[0]
    {
        "help" => println!("this is help penguin <(\")"),
        _ => println!("Unknown or invalid command."),
    }
}