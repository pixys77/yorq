
use bollard::Docker;

pub async fn container_inspect(name:&str){

let true_running: &str   = "Running";
let false_running: &str  = "Not running";


    let docker = Docker::connect_with_socket_defaults()
        .expect("Не удалось подключиться к Docker сокету");

    let inspect_result = docker
    .inspect_container(name, None)
    .await;

    match inspect_result {
        Ok(info)    =>{

        if let Some(state) = info.state {
            let is_running = state.running.unwrap_or_default();
            let pid = state.pid.unwrap_or_default();
            let oom_kill = omm_killed.unwrap_or_default();


            if is_running {
                println!("Running status is: {:?}", true_running);    
            }else  {
                println!("Running status is: {:?}", false_running);
            }


            
            if pid > 0{
                println!("Procces PID: {:?}", pid)

            }else if pid == 0{
                println!("Procces PID: 0. Container is stopped/exited, or never started.")


            }
        }
    }
        Err(e)      => println!("Error:{:?}", e),
}
}