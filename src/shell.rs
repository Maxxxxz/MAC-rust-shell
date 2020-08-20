use std::io::{self, Write};
use std::collections::HashMap;
use std::any::Any;

type inType = Vec<String>;
type outType = bool;
type shellType = fn(inType) -> outType;

pub fn run() -> bool
{

    // hashmap<str, fn()>

    // let mut cmd: HashMap<&str, &Any> = HashMap::new();
    let mut cmd: HashMap<&str, shellType> = HashMap::new();

    // builtin functions map
    cmd.insert("", mac_und);
    cmd.insert("ls", mac_ls);
    cmd.insert("cd", mac_cd);
    cmd.insert("help", mac_help);
    cmd.insert("quit", mac_quit);

    let mut cmds: Vec<&str> = vec!["help", "quit"];
    // let user prefs = prefs
    // pass prefs and cmds into init

    init(&mut cmds);

    loop
    {
        let msg = getMessage();

        // if msg == "quit"
        // {
        //     break;
        // }

        let mut _c: Vec<String> = parseMessage(msg);

        // &_c[0] -> get first word in command (will be the function to run)
        // [..] -> turn String into &str slice
        // println!("{}", _c[0][..].to_string());

        let func/*: fn(Vec<&str>) -> ()*/ =
            cmd.get(&_c[0][..]);
            // .unwrap();
            // .downcast_ref::<shellType>();
            // .unwrap();
            // let func = func.downcast_ref::<shellType>().unwrap();
        let continueLoop = match func
        {
            Some(f1) => f1(_c),
            None => mac_unk(_c),
        };

        // let continueLoop = func(&mut _c);

        match continueLoop
        {
            false => break,
            _ => (),
        }

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

fn parseMessage(msg: String) -> Vec<String>
{
    if msg.is_empty()
    {
        return vec!["".to_string()];
    }
    
    let retVec: Vec<String> = msg.split_whitespace().map(|st| st.to_string()).collect();

    return retVec;
}

fn _parseMessage(msg: String)// -> Option? Something to tell the loop what went wrong
{
    // split message by whitespace and parse via first command
    if msg.is_empty()
    {
        return; //return option, user probably just wants to clear the terminal
    }
    
    let inVec: Vec<&str> = msg.split_whitespace().collect();

    // match inVec[0]
    // {
    //     "help" => println!("this is help penguin <(\")"),
    //     _ => println!("Unknown or invalid command."),
    // }
}

fn mac_unk(cmds: Vec<String>) -> outType
{
    println!("Unknown command: {}", cmds[0]);
    return true;
}

fn mac_und(cmds: Vec<String>) -> outType
{
    return true;
}

fn mac_ls(cmds: Vec<String>) -> outType
{
    println!("ls");
    return true;
}

fn mac_cd(cmds: Vec<String>) -> outType
{
    println!("cd");
    return true;
}

fn mac_help(cmds: Vec<String>) -> outType
{
    println!("this is help penguin <(\")");
    return true;
}

fn mac_quit(cmds: Vec<String>) -> outType
{
    return false;
}