mod inspect;
mod state;
mod list;
mod sweep;
mod restart;

#[tokio::main]


async fn main(){
 let help = r#"
    yorq - lightweight Docker container CLI manager

USAGE:
    yorq <COMMAND> [OPTIONS]

COMMANDS:
ls                         List all containers with status and timestamps
inspect <id/name>          Show container state
start   <ids/names...>     Start specified container(s)
stop    <ids/names...>     Stop specified container(s)
restart <ids/names...>     Restart specified container(s)
sweep                      Interactively clean containers"#;
let inspect_help = r#"
[!] Error!
USAGE:
    yorq inspect <id>
    yorq inspect <name>"#;
let start_help = r#"
[!] Error!
USAGE:
    yorq start <id|name>...
    yorq start <id1> <id2> [name3]..."#;
let stop_help = r#"    
[!] Error!
USAGE:
    yorq stop <id|name>...
    yorq stop <id1> <id2> [name3]..."#;
let restart_help = r#"
[!] Error!
USAGE:
    yorq restart <id|name>...
    yorq restart <id1> <id2> [name3]..."#;
let sweep_help = r#"
[!] Error!
USAGE:
    yorq sweep"#;
let ls_help = r#"
[!] Error!
USAGE:
    yorq ls"#;

    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2{
        println!("[X] Error! Use arguments\n{help}");
        return;
    }
    match args[1].as_str(){
        "help"                          => println!("{help}"),


        "inspect" if args.len() > 3     => println!("[X] Error! To much args.\n{inspect_help}"),
        "inspect" if args.len() < 3     => println!("[X] Error! not enough args.\n{inspect_help}"),
        "inspect"                       => inspect::container_inspect(&args[2]).await,

        "start" if args.len() < 3       => println!("[X] Error! Not enough args.\n{start_help}"),
        "start"                         => state::start_cont(&args[2..]).await, 

        "stop" if args.len() < 3        => println!("[X] Error! Not enough args.\n{stop_help}"),
        "stop"                          => state::stop_cont(&args[2..]).await,

        "sweep" if args.len() > 2       => println!("[X] Error. Too much args.\n{sweep_help}"),
        "sweep"                         => sweep::sweep().await,
        
        "restart"                       => restart::restart(&args[2..]).await,

        "ls"                            => list::ls().await,
        _                               => println!("Undefined command\n{help}"),

    }


}
//