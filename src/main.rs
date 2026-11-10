//use bollard::Docker;
//
//#[tokio::main]
//async fn main() {
//
//    let docker = Docker::connect_with_socket_defaults()
//        .expect("Не удалось подключиться к Docker сокету");
//    let docker_ping = docker.ping()
//    .await;
//
//    match docker_ping{ 
//        Ok(response)    => println!("Ответ успешный: {:?}", response),
//        Err(error)      => println!("Ошибка:{:?}", error),
//
//    }
//}

use bollard::Docker;

#[tokio::main]
async fn main(){
    let docker = Docker::connect_with_socket_defaults()
        .expect("Не удалось подключиться к Docker сокету");

    let inspect_result = docker
    .inspect_container("test-box", None)
    .await;

    match inspect_result {
        Ok(info)    =>{ println!("Container found");

        if let Some(state) = info.state {
            let is_running = state.running.unwrap_or_default();
            let pid = state.pid.unwrap_or_default();

            println!("Running status:{:?}", is_running);
            println!("Procces PID:{:?}", pid);
        }
    }
        Err(e)      => println!("Error:{:?}", e),
    }
}