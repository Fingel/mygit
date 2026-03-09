use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{Read, Write};

use flate2::read::ZlibDecoder;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        "cat-file" => {
            let id = &args[3]; // ignore -p for now
            let path = path_to_object(id);
            let contents = read_object(&path);
            print!("{}", String::from_utf8_lossy(&contents));
        }
        "hash-object" => {
            let path = &args[3]; // ignore -w for now
            let hash = hash_object(path);
            print!("{}", hash);
        }
        _ => println!("unknown command: {}", args[1]),
    }
}

fn path_to_object(id: &str) -> String {
    format!(".git/objects/{}/{}", &id[0..2], &id[2..])
}

fn hash_object(path: &str) -> String {
    match fs::read(path) {
        Ok(contents) => {
            let len = contents.len();
            let header = format!("blob {}\0", len);
            let contents: Vec<u8> = [header.as_bytes(), &contents].concat();

            let mut hasher = Sha1::new();
            hasher.update(&contents);
            let result = hasher.finalize();
            let hash = format!("{:x}", result).to_string();

            // write the compressed object to the correct path
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(&contents).unwrap();
            let compressed = e.finish().unwrap();
            let object_path = path_to_object(&hash);
            fs::create_dir_all(format!(".git/objects/{}", &hash[0..2])).unwrap();
            fs::write(object_path, compressed).unwrap();

            hash
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            String::new()
        }
    }
}

fn read_object(path: &str) -> Vec<u8> {
    let compressed = match fs::read(path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading object: {}", e);
            vec![]
        }
    };
    let mut d = ZlibDecoder::new(&compressed[..]);
    let mut b = Vec::new();
    d.read_to_end(&mut b).unwrap();
    // only return the content after the null byte
    // If we need the header later we can do that here
    let null_index = b.iter().position(|&x| x == 0).unwrap();
    b.drain(0..null_index + 1);
    b
}
