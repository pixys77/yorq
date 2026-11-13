use bollard::Docker;
use bollard::container::{StartContainerOptions, StopContainerOptions};
use std::io;


pub async fn start_cont(containers: &[String]){
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

    let mut confirm = String::new();
    println!("Are you sure y/n");
    io::stdin().read_line(&mut confirm).expect("[!] Error! Can`t read line");
    let confirm_string: String = confirm.trim().trim_start_matches("/").to_string();
    if confirm_string == "y" {
            for name in containers{

                    let start = docker.start_container(name, None::<StartContainerOptions<String>>).await;

                match start {
                    Ok(_) => println!("[+] Container '{}', started", name),
                    Err(error)  => println!("[!] Failed to start containter: {:?}", error),
                }
            }
        }
    
    }   

pub async fn stop_cont(containers: &[String]){
    let docker = Docker::connect_with_socket_defaults()
    .expect("Failed to connect to the Docker socket");

    let mut confirm = String::new();
    println!("Are you sure y/n");
    io::stdin().read_line(&mut confirm).expect("[!] Error! Can`t read line");
    let confirm_string: String = confirm.trim().trim_start_matches("/").to_string();
    if confirm_string == "y" {
            for name in containers{

                    let stop = docker.stop_container(name, None::<StopContainerOptions>).await;

                match stop {
                    Ok(_) => println!("[+] Container '{}', stopped", name),
                    Err(error)  => println!("[!] Failed to stop containter: {:?}", error),
                }
            }
        }
    
    }   