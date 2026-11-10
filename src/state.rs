use bollard::Docker;
use bollard::container::{StartContainerOptions, StopContainerOptions};

pub async fn start_cont(name:&str){

    let docker = Docker::connect_with_socket_defaults()
    .expect("Не удалось подключиться к Docker сокету");

    let start = docker.start_container(name, None::<StartContainerOptions<String>>).await;

    match start {
        Ok(started) => println!("[+] Container '{}', started", name),
        Err(error)  => println!("[X] Failed to start containter: {:?}", error),
    }
}

pub async fn stop_cont(name:&str){
    let docker = Docker::connect_with_socket_defaults()
    .expect("Не удалось подключиться к Docker сокету");
    
    let stop = docker.stop_container(name, None::<StopContainerOptions>).await;

    match stop {
        Ok(started) => println!("[+] Container '{}', stopped", name),
        Err(error)  => println!("[X] Failed to stop containter: {:?}", error),
    }

  
}