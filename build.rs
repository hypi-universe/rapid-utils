use std::fs::File;
use std::io::ErrorKind;
use std::{env, fs};

fn main() {
    //CARGO_MANIFEST_DIR is outside OUT_DIR so it gets rejected
    //If the package has a build script, this is set to the folder where the build script should place its output.
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = format!("{}", out_dir);
    let remote_protos = [
        "https://gist.githubusercontent.com/zcourts/72146d43a4cad99b7b9e2987076040fb/raw/525795ea3e29f8e6d55367e17aef8c0d8a5b5f15/rapid_plugin.proto",
    ];
    let proto_dir = format!("{}/proto", out_dir);
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
        let file_name = format!("{}/proto/{}", out_dir, file_name);
        protos.push(file_name.clone());
        fs::write(file_name, resp).unwrap();
    }

    //PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(format!("{}/plugin_descriptor.bin", out_dir))
        .include_file("plugin.rs")
        .out_dir(out_dir.clone())
        .compile(protos.as_slice(), &[format!("{}/proto", out_dir)])
        .unwrap();
}
