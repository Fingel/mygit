#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;

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
        _ => println!("unknown command: {}", args[1]),
    }
}

fn path_to_object(id: &str) -> String {
    format!(".git/objects/{}/{}", &id[0..2], &id[2..])
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
