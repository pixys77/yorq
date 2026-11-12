use std::io;
use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions};

pub async fn container_create(){
    let docker = Docker::connect_with_socket_defaults()
    .expect("[!] Error! Failed to connect to the docker socket");

    println!("Enter the image name:");
    let mut container_image = String::new();
    io::stdin().read_line(&mut container_image).expect("[!] Error! Can`t read line");
    let confirm_image: String = container_image.trim().trim_start_matches("/").to_string();
    
    println!("Enter the container (or containers) name");
    let mut container_name = String::new();
    io::stdin().read_line(&mut container_name).expect("[!] Error! Can`t read line");
    let confirm_name: String = container_name.trim().trim_start_matches("/").to_string();


    let options = Some(CreateContainerOptions{
    name: confirm_name.clone(),
    ..Default::default()});
    println!("{confirm_image} first");

    let config = Config {
    image: Some(confirm_image.to_string()),
    ..Default::default()
    };
    println!("{confirm_image} first");
    
    match docker.create_container(options, config).await{
        Ok(_)       => println!("[+] Container {container_image} created"),
        Err(error)  => println!("[!] Failed to create container!{}", error),
    }
}