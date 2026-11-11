use std::io;
use bollard::container::RestartContainerOptions;
use bollard::Docker;

pub async fn restart(containers: &[String]){

    let docker = Docker::connect_with_socket_defaults()
    .expect("[!] Error! Failed to connect to the Docker socket");

    
    println!("Are you sure? y/n");
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("[!] Error! Can`t read line");
    let confirm_string: String = confirm.trim().trim_start_matches("/").to_string();
    if confirm_string == "y" { 
        for name in containers{
            let options = RestartContainerOptions {t: 0};

            docker.restart_container(name, Some(options)).await;
        match docker.restart_container(name, Some(options)).await {
            Ok(_) => println!("succesful"),
            Err(error) => eprintln!("fail"),

        }
        }
    } else {
        println!("[?] Something wrong...");
    }
    
    
}