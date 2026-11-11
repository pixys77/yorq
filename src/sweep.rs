use std::io;
use bollard::container::RemoveContainerOptions;
use std::time::{SystemTime, UNIX_EPOCH};
use bollard::container::ListContainersOptions;
use bollard::Docker;
use std::io::Write;

pub async fn sweep(){

    let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs() as i64;
    let mut container_data = Vec::new();
    let docker = Docker::connect_with_socket_defaults()
    .expect("Failed to connect to the Docker socket");
    let docker_list = docker
    .list_containers(Some(ListContainersOptions::<String>{all: true, ..Default::default()})).await;
    let container = docker_list
    .expect("Failed to retrieve the list");

    for data in container {
        let id = data.id.unwrap_or_default();
        let name = data.names.unwrap_or_default().join(", ");
        let timestamp = data.created.unwrap_or_default();
        let time = now - timestamp;
        container_data.push(Some((id, name, time)));
    }
        println!("{:<3}  {:<1}  {:<25}  {:<12}", "#", "Container ID", "Name", "Created");
        println!("{}", "-".repeat(58));
    for (number, item) in container_data.iter().enumerate() {
        let (container_id, container_name, time) = item.as_ref().unwrap();
        let short_id: String = container_id.chars().take(6).collect();
        println!("{:<3} {:<12} {:<25} {:<12}",number + 1, short_id, container_name, time);
    }
    let mut input = String::new();
    loop {

        input.clear();
        println!("[!] Enter number! Enter 'q' to cancel");
        io::stdout().flush().expect("[!] Error: Failed to flush stdout");

        if io::stdin().read_line(&mut input).is_err(){
            println!("[!] Error: Failed to read from stdin");
            break
        }
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") || trimmed == "exit" {
            println!("[!] Cancel! Exiting...");
            break;
        }
        match trimmed.parse::<usize>(){
            Ok(num) if num > 0 => {
                let index = num - 1;
                match container_data.get(index){
                    Some(Some((target_id, target_name, _))) => {
                        println!("[!] Deleting container: {}\nID:\n{}", target_name, target_id);
                        let mut confirm = String::new();
                        println!("[?] Are you sure? y/n");
                        io::stdin().read_line(&mut confirm).expect("Failed to read");
                        let clean_confirm = confirm.trim();
                        if clean_confirm == "y" {
                            let _ = docker.remove_container(target_id, None::<RemoveContainerOptions>).await;
                            container_data[index] = None;
                            println!("[+] Succesful");
                        }else if clean_confirm == "n" {
                            break;
                        }
                    }
                
                Some(None) => println!("[!] Container #{} already deleted", num),
                None => println!("[!] Error: Number {} doesn`t exist. (All:{})", num, container_data.len()),  
                }

            }

        _ => println!("[!] Erorr: Incorrect input"),

        }

    }

}
