use enginessh::Config;
use ssh2::Session;
use std::io;
use std::{env, io::Read, net::TcpStream, path::Path};
use yaml_rust::{Yaml, YamlLoader};

// TODO: Add proper error handling and tracing info
fn main() {
    let yaml_path = env::var("CONFIG").expect("Could not find CONFIG environment variable");
    let yaml = get_yaml_from_file(&yaml_path);
    let config = Config::from_yaml(&yaml[0]);

    // Connect to the remote host
    let server = format!("{}:{}", config.host, config.port);
    let tcp = TcpStream::connect(server).expect("TCP connection failed");
    let mut sess = Session::new().expect("Could not create session");

    sess.set_tcp_stream(tcp);
    sess.handshake().expect("Failed to establish SSH session");

    // Authenticate using a PEM File
    sess.userauth_pubkey_file(
        &config.user,
        None,
        Path::new(&config.priv_key_file),
        Some(&config.password),
    )
    .expect("Failed to authenticate with key");

    // Execute command
    let mut channel = sess.channel_session().unwrap();
    channel.shell().expect("Failed to open shell");
    channel
        .exec(&config.remote_cmd)
        .expect("Failed to execute command");

    let mut line = String::new();
    let scan = io::stdin();

    loop {
        line.clear();
        let bytes = scan.read_line(&mut line).unwrap();
        let line = line.trim();

        match line {
            "exit" if line == "exit" || bytes == 0 => break,
            "" => continue,
            _ => channel.exec(line).expect("Failed to execute command"),
        }
    }
}

fn get_yaml_from_file(path: &str) -> Yaml {
    let mut file = std::fs::File::open(path).expect("FAIL: open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("FAIL: read file to string");

    let yaml = YamlLoader::load_from_str(&contents).expect("FAIL: parse YAML");
    yaml[0].clone()
}
