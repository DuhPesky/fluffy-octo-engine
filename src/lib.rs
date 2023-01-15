use yaml_rust::Yaml;

pub struct Config {
    pub user: String,
    pub host: String,
    pub port: String,
    pub remote_cmd: String,
    pub password: String,
    pub priv_key_file: String,
    pub log_file: String,
}

impl Config {
    pub fn from_yaml(config: &Yaml) -> Self {
        Self {
            user: String::from(config["user"].as_str().expect("Could not parse user")),
            host: String::from(config["host"].as_str().expect("Could not parse host")),
            port: String::from(config["port"].as_str().expect("Could not parse port")),
            remote_cmd: String::from(
                config["remoteCommand"]
                    .as_str()
                    .expect("Could not parse remoteCommand"),
            ),
            password: String::from(
                config["password"]
                    .as_str()
                    .expect("Could not parse password"),
            ),
            priv_key_file: String::from(
                config["publicKeyFile"]
                    .as_str()
                    .expect("Could not parse publicKeyFile"),
            ),
            log_file: String::from(config["logFile"].as_str().expect("Could not parse logFile")),
        }
    }
}
