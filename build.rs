use std::fs::File;
use std::io::ErrorKind;
use std::{env, fs};

fn main() {
    let pwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = format!("{}/src", pwd);
    let remote_protos = [
        "https://gist.githubusercontent.com/zcourts/72146d43a4cad99b7b9e2987076040fb/raw/8d3ff0b9fc6ba39d32e611b70518fefd6de5bd9b/rapid_plugin.proto",
    ];
    let proto_dir = format!("{}/proto", pwd);
    match File::open(proto_dir.clone()) {
        Ok(_) => {
            println!("proto dir exists")
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("proto dir doesn't exist, creating");
                fs::create_dir(proto_dir).unwrap();
            }
            _ => {
                panic!("Unexpected error checking if proto dir exists. {}", e)
            }
        },
    }
    let mut protos = vec![];
    for proto_url in remote_protos {
        println!("Fetching proto file: {}", proto_url);
        let resp = reqwest::blocking::get(proto_url).unwrap().text().unwrap();
        let file_name = proto_url.split("/").last().unwrap();
        let file_name = format!("{}/proto/{}", pwd, file_name);
        protos.push(file_name.clone());
        fs::write(file_name, resp).unwrap();
    }

    //PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(format!("{}/plugin_descriptor.bin", out_dir))
        .include_file("plugin.rs")
        .out_dir(out_dir)
        .compile(protos.as_slice(), &[format!("{}/proto", pwd)])
        .unwrap();
}
