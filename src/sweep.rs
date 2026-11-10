use std::io;
use bollard::container::RemoveContainerOptions;
use std::time::{SystemTime, UNIX_EPOCH};
use bollard::container::ListContainersOptions;
use bollard::Docker;

pub async fn clean(){
    let docker = Docker::connect_with_socket_defaults()
    .expect("Не удалось подключиться к Docker сокету");

    let docker_list = docker.list_containers(Some(ListContainersOptions::<String>{all: true, ..Default::default()})).await;
    let containers = docker_list.expect("Не удалось получить список");
    
    
    let one_hour = 3600;
    let one_day = 10;
    
    let present_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;

    let mut targets = Vec::new();
    
    for data in containers {
        let id = data.id.unwrap_or_default();
        let state = data.state.unwrap_or_default();
        let txt_data = data.status.unwrap_or_default();
        let timestamp = data.created.unwrap_or_default();
        let name = data.names.unwrap_or_default().join(", ");
        let container_time = present_time - timestamp;
        if container_time > one_day {
            targets.push((id, name, container_time));
        }
    }
    
    for (target_id, target_name, target_time) in targets {
    let mut input = String::new();
    println!("Container {} has exist more then 24 hours. {target_time} seconds elapsed.\nContainer ID: {target_id}\nDelete? (y/n)", target_name.trim_start_matches("/"));

    io::stdin().read_line(&mut input).expect("Cannot read line");
    
    if input.trim() == "y"{
        let options = None::<RemoveContainerOptions>;
        let result = docker.remove_container(&target_id, options).await;
        match result {
            Ok(_) => println!("[+] Container {target_name} removed"),
            Err(error) => eprintln!("Failed to remove {target_name}: {error}"),
    
        }
    }else{
        println!("Container {} will not be deleted", target_name.trim_start_matches("/"));
    }
}
}

    

