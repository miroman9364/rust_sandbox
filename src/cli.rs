/* https://youtu.be/zF34dRivLOw?t=6184
*/

use std::env;

enum Command {
    Hello,
    Greeting,
    Message,
    Usage,
}

fn to_command(command: &String) -> Command {
    let arg: &str = command.as_str();

    match arg {
        "" => Command::Hello,
        "--hello" => Command::Hello,
        "-h" => Command::Hello,
        "--greeting" => Command::Greeting,
        "-g" => Command::Greeting,
        "--message" => Command::Message,
        "-m" => Command::Message,
        &_ => Command::Usage,
    }
}

fn usage(args: &Vec<String>) {
    let path: String = args[0].clone();
    let (_, app_name) = path.rsplit_once('\\').unwrap();

    println!("Usage: {} [command [options]]", app_name);
}

fn do_hello_world() {
    print!("[do_hello_world] :: ");
    print!("Hello, world! ");
}

fn do_greeting(words: &Vec<String>) {
    print!("[do_greeting] :: ");
    for word in words.iter() {
        if !word.is_empty() {
            print!("{} ", word);
        }
    }
}

fn do_message(words: &Vec<String>) {
    print!("[do_message] :: ");
    do_hello_world();
    do_greeting(words);
}

pub fn run() {
    let mut args: Vec<String> = env::args().collect();
    args.push("".to_string());
    args.push("".to_string());

    let command: Command = to_command(&args[1]);
    let slice: &[String] = &args[2..];
    let words: Vec<String> = slice.to_vec();

    match command {
        Command::Hello => do_hello_world(),
        Command::Greeting => do_greeting(&words),
        Command::Message => do_message(&words),
        Command::Usage => usage(&args),
    }

    println!();
}
