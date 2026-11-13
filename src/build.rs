use std::io;
use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions};

pub async fn container_run(){
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
    println!("Enter the image name:");
    let mut container_image_name = String::new();
    io::stdin().read_line(&mut container_image_name).expect("[!] Error! Can`t read line");
    let confirm_image: String = container_image_name.trim().trim_start_matches("/").to_string();
    
    println!("Enter the container (or containers) name");
    let mut container_name = String::new();
    io::stdin().read_line(&mut container_name).expect("[!] Error! Can`t read line");
    let confirm_name: String = container_name.trim().trim_start_matches("/").to_string();


    let options = Some(CreateContainerOptions{
    name: confirm_name.clone(),
    ..Default::default()});

    let config = Config {
    image: Some(confirm_image.to_string()),
    ..Default::default()
    };
    
    match docker.create_container(options, config).await{
        Ok(_)       => println!("[+] Container {container_image_name} created"),
        Err(error)  => println!("[!] Error! Failed to create container!{}", error),
    }
}