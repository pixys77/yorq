
use bollard::Docker;

pub async fn container_inspect(name:&str){



    let docker = Docker::connect_with_socket_defaults()
        .expect("[!] Error! Failed to connect to the Docker socket");

    let inspect_result = docker.inspect_container(name, None).await;

    match inspect_result {
        Ok(info)    =>{

        if let Some(state) = info.state {
            let is_running = state.running.unwrap_or_default();
            let pid = state.pid.unwrap_or_default();
            let oom_kill = state.oom_killed.unwrap_or_default();
            let container_name = info.name.unwrap_or_default();
            let container_start = state.started_at.unwrap_or_default();
            let container_hash = info.id.unwrap_or_default();
            
            //OOM kill check
            if oom_kill {
                println!("[!] Container OOM Killed!");
            }

            // container name check
            if container_name.len() > 0 {
                println!("[+] Container name: {}",container_name.trim_start_matches("/"));
            }else{
                println!("[!] Error!");
            }
            // running state check
            if is_running {
                println!("[+] Running status is: {}", "Running");    
            }else  {
                println!("[+] Docker container is not running");
            }
            
            //container time stamp
            if container_start.len() > 0 {
                println!("Container start at: {}", container_start);
            }else{
                println!("[!] Error!");
            }


            // container hash
            if container_hash.len() > 0 {
                println!("[+] Container hash: {}", container_hash);
            }

            

            // PID Check!
            if pid > 0{
                println!("[+] Procces PID: {}", pid)
            }else if pid == 0{
                println!("[+] Procces PID is 0. Container is stopped/exited, or never started.")
            }
        
        }
    }
        Err(e)      => println!("[!] Error: {}", e),
}
}