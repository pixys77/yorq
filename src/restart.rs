use std::io;
use bollard::container::RestartContainerOptions;
use bollard::Docker;

pub async fn restart(containers: &[String]){

      let docker = match Docker::connect_with_socket_defaults(){
            Ok(result)   =>{       
            println!("[+] Succesful connect to docker");
            result          
            },
            Err(e) =>{
            println!("[!] Error!{e}");
            return;
            },
        };
    
    println!("[?] Are you sure? y/n");
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("[!] Error! Can`t read line");
    let confirm_string: String = confirm.trim().trim_start_matches("/").to_string();
    if confirm_string == "y" { 
        for name in containers{
            let options = RestartContainerOptions {t: 0};

            let restart_container = docker.restart_container(name, Some(options)).await;
        match restart_container {
            Ok(_) => println!("[+] Container restarted succesful!"),
            Err(error) => eprintln!("[!] Error! Container was not restarted: {error}"),

        }
        }
    } else {
        println!("[?] Something wrong...");
    }
    
    
}