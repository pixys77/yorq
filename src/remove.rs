use bollard::Docker;
use bollard::container::RemoveContainerOptions;
use std::io;


pub async fn remove(containers: &[String]){

    let docker = Docker::connect_with_socket_defaults()
    .expect("[!] Error! Failed to connect to the Docker socket");    

    println!("[?] Are you sure? y/n");
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).expect("[!] Error! Can`t read line");
    let confirm_string: String = confirm.trim().trim_start_matches("/").to_string();
    
    if confirm_string == "y" {

        for name in containers{
            
            
            let remove_container = docker.remove_container(name, None::<RemoveContainerOptions>).await;
            match remove_container{
                Ok(_) => println!("[+] Succesful, container {name} delete"),
                Err(error) => eprintln!("[!] Error!: Failed: {error}"),
         
            };
        }
    }else {println!("[?] Something wrong...")}
}
