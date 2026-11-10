mod inspect;
mod state;
mod sweep;
mod list;
#[tokio::main]


async fn main(){
    let HELP = r#"yorq - lightweight Docker container CLI manager

USAGE:
    yorq <COMMAND> [OPTIONS]

COMMANDS:
    inspect <id/name>  Show container state and configuration
    start   <id/name>  Start specified container
    stop    <id/name>  Stop specified container
    sweep              Interactively clean containers older than 24h"#;

    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2{
        println!("[X] Error! Use arguments\n{HELP}");
        return;
    }
    match args[1].as_str(){
        "inspect" if args.len() <  3    => println!("[X] Error! not enough args.\n{HELP}"),
        "inspect"                       => inspect::container_inspect(&args[2]).await,
        
        "start" if args.len() < 3       => println!("[X] Error! Not enough args.\n{HELP}"),
        "start"                         => state::start_cont(&args[2]).await, 
        
        "stop" if args.len() < 3        => println!("[X] Error! Not enough args.\n{HELP}"),
        "stop"                          => state::stop_cont(&args[2]).await,

        "help"                          => println!("{HELP}"),
        "sweep"                         => sweep::clean().await,
        
        "ls"                            => list::ls().await,
        _                               => println!("Undefined command\n{HELP}"),

    }


}