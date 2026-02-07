pub struct Server {
    adress: (String, u16),
    log_level: String,
}

impl Server {
    pub fn new(adress: (String, u16), log_level: String) -> Self {
        Server { adress, log_level }
    }

    pub fn adress(&self) -> (String, u16) {
        self.adress.clone()
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ascii(self))
    }
}

pub fn from_env() -> Server {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8111".to_string())
        .parse()
        .unwrap_or(8111);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    Server::new(("0.0.0.0".to_string(), port), log_level)
}

fn ascii(server: &Server) -> String {
    let (root, port) = server.adress();

    let d = format!("http://{root}:{port}");
    format!(
        "
        __| |_________________| |__
        __   _________________   __
          | |                 | |       Catch your http requests
          | | ╔═╗┌─┐┌┬┐┌─┐┬ ┬ | |
          | | ║  ├─┤ │ │  ├─┤ | |       https://github.com/SilenLoc/catch
          | | ╚═╝┴ ┴ ┴ └─┘┴ ┴ | |
        __| |_________________| |__     {d}
        __   _________________   __
          | |                 | |
        ",
    )
}
