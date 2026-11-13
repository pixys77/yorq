mod inspect;
mod state;
mod list;
mod sweep;
mod restart;
mod remove;
mod build;
mod create;
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
sweep                      Interactively clean containers
run
yorq remove <id|name>...
yorq remove <id1> <id2> [name3]..."#;

let inspect_help = r#"
USAGE:
    yorq inspect <id>
    yorq inspect <name>"#;
let start_help = r#"
USAGE:
    yorq start <id|name>...
    yorq start <id1> <id2> [name3]..."#;
let stop_help = r#"    
USAGE:
    yorq stop <id|name>...
    yorq stop <id1> <id2> [name3]..."#;
let restart_help = r#"
USAGE:
    yorq restart <id|name>...
    yorq restart <id1> <id2> [name3]..."#;
let sweep_help = r#"
USAGE:
    yorq sweep"#;
let ls_help = r#"
USAGE:
    yorq ls"#;
let run_help =r#"
USAGE:
    yorq run"#;
let remove_help = r#"[!] Error!
USAGE:
    yorq remove <id|name>...
    yorq remove <id1> <id2> [name3]..."#;
    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2{
        println!("[!] Error! Use arguments\n{help}");
        return;
    }
    match args[1].as_str(){
        "help"                          => println!("{help}"),

        "remove" if args.len() < 3      => println!("[!] Error! Not enough args.\n{remove_help}"),
        "remove"                        => remove::remove(&args[2..]).await,

        "rm" if args.len() < 3          => println!("[!] Error! Not enough args.\n{remove_help}"),
        "rm"                            => remove::remove(&args[2..]).await,
        

        "inspect" if args.len() > 3     => println!("[!] Error! To much args.\n{inspect_help}"),
        "inspect" if args.len() < 3     => println!("[!] Error! Not enough args.\n{inspect_help}"),
        "inspect"                       => inspect::container_inspect(&args[2]).await,

        "start" if args.len() < 3       => println!("[!] Error! Not enough args.\n{start_help}"),
        "start"                         => state::start_cont(&args[2..]).await, 

        "stop" if args.len() < 3        => println!("[!] Error! Not enough args.\n{stop_help}"),
        "stop"                          => state::stop_cont(&args[2..]).await,

        "sweep" if args.len() > 2       => println!("[!] Error. Too much args.\n{sweep_help}"),
        "sweep"                         => sweep::sweep().await,
        
        "restart" if args.len() < 3     => println!("[!] Error! Not enough args.\n{restart_help}"),
        "restart"                       => restart::restart(&args[2..]).await,
    
        "pull"                          => create::create_image(&args[2..]).await,

        "run" if args.len() > 2         => println!("[!] Error! Too much args.\n{run_help}"), 
        "run"                           => build::container_run().await,

        "ls" if args.len() > 2          => println!("[!] Error! Too much args.\n{ls_help}"),
        "ls"                            => list::ls().await,

        "ps" if args.len() > 2          => println!("[!] Error! Too much args.\n{ls_help}"),
        "ps"                            => list::ls().await,
        


        _                               => println!("Undefined command\n{help}"),

    }
}