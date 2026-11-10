mod inspect;

#[tokio::main]


async fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2{
        println!("[X] Error!");
        return;
    }
    match args[1].as_str(){
        "inspect" if args.len() <  3    => println!("[X] Error! not enough args."),
        "inspect"                       => inspect::container_inspect(&args[2]).await,
        
        _                               => println!("Undefined command")
    }


}