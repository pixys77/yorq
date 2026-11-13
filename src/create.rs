use bollard::Docker;
use bollard::image::{CreateImageOptions};
use futures_util::StreamExt;

pub async fn create_image(name_image: &[String]){
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

    let confirm_image = match name_image.first() {
        Some(image) => image.trim(),
        None => {
            println!("[!] Image name not provided");
            return;
        }
    };

    let (image_name, tag) = match confirm_image.split_once(":"){
        Some((image_name, tag)) => (image_name, tag),
        None => {
            println!("[!] Incorrect format");
            return;
        }
    };

    let options = Some(CreateImageOptions{
        from_image: image_name,
        tag,
        ..Default::default()});
    
    let mut stream = docker.create_image(options, None, None);
    while let Some(result) = stream.next().await {
        match result{
            Ok(info)    => {
                println!("Processing...\n{info:?}");
            },

            Err(error)  => {
                println!("Error!\n{error}");
            },
        
        }
    }
}