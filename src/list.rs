use bollard::Docker;
use bollard::container::ListContainersOptions;

pub async fn ls(){
    let docker = Docker::connect_with_socket_defaults()
    .expect("Failed to connect to the Docker socket");
    
    let docker_list = docker.list_containers(Some(ListContainersOptions::<String>{all: true, ..Default::default()})).await;
    let containers = docker_list.expect("Failed to retrieve the list");
    
    let mut list = Vec::new();

    for data in containers {
        let state = data.state.unwrap_or_default();
        let id = data.id.unwrap_or_default();
        let name = data.names.unwrap_or_default().join(",");
        let timestamp = data.created.unwrap_or_default();
        list.push((id, name, state, timestamp));

    }
    println!("{:<10} {:>20} {:>20} {:>20}", "Container ID", "Container name", "Container Status", "Timestamp");
    println!("--------------------------------------------------------------------------------------------------");
    
    for (target_id, target_name, target_state, target_timestamp) in list{
    let id_hash: String = target_id.chars().take(6).collect();
    let container_name = target_name.trim_start_matches("/");
    println!("{:>8} {:>24} {:>17} {:>27}", id_hash, container_name, target_state, target_timestamp);    

 

    }


}