mod inspect;
mod state;
mod list;
mod sweep;

#[tokio::main]


async fn main(){
    let help = r#"yorq - lightweight Docker container CLI manager

USAGE:
    yorq <COMMAND> [OPTIONS]

COMMANDS:
    ls                 List all containers with status and timestamps
    inspect <id/name>  Show container state
    start   <id/name>  Start specified container
    stop    <id/name>  Stop specified container
    sweep              Interactively clean containers"#;

    let inspect_help = r#"
    
    [!] Error!
    USAGE:
    
        yorq inspect <id>
        yorq inspect <name>

    "#;
    let start_help = r#"
    
    [!] Error!
    USAGE:

        yorq start <id>
        yorq start <name>
    
    "#;
    let stop_help = r#"
    
    [!] Error!
    USAGE:

        yorq stop <id>
        yorq stop <name>
    
    "#;
    let sweep_help = r#"
    
    [!] Error!
    USAGE:

        yorq sweep

    "#;

    let ls_help = r#"
    
    [!] Error!
    USAGE:

        yorq ls

    "#;

    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2{
        println!("[X] Error! Use arguments\n{help}");
        return;
    }
    match args[1].as_str(){
        "help"                          => println!("{help}"),


        "inspect" if args.len() < 3    => println!("[X] Error! not enough args.\n{inspect_help}"),
        "inspect" if args.len() > 3    => println!("[X] Error! To much args.\n{inspect_help}"),
        "inspect"                      => inspect::container_inspect(&args[2]).await,

        "start" if args.len() < 3       => println!("[X] Error! Not enough args.\n{start_help}"),
        "start" if args.len() > 3       =>println!("[X] Error! To much args.\n{start_help}"),
        "start"                         => state::start_cont(&args[2]).await, 

        "stop" if args.len() > 3        => println!("[X] Error! Too much args.\n{stop_help}"),
        "stop" if args.len() < 3        => println!("[X] Error! Not enough args.\n{stop_help}"),
        "stop"                          => state::stop_cont(&args[2]).await,

        "sweep"                         => sweep::sweep().await,
        "sweep" if args.len() > 2       => println!("[X] Error. Too much args.\n{sweep_help}"),
        
        "ls"                            => list::ls().await,
        _                               => println!("Undefined command\n{help}"),

    }


}
