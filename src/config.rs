use std::env::var;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub domain: String,
    pub base_url: String,
}

lazy_static! {
    pub static ref HOST: String = var("host").unwrap_or_else(|_| "127.0.0.1".to_owned());
    pub static ref PORT: u16 = var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse::<u16>()
        .unwrap();
    pub static ref PROTOCOL: String = var("PROTOCOL").unwrap_or_else(|_| "https".to_owned());
    pub static ref DOMAIN: String = var("DOMAIN").unwrap_or_else(|_| format!("{}:{}", *HOST, *PORT));
    pub static ref BASE_URL: String = var("BASE_URL").unwrap_or_else(|_| {
        if (*PORT == 80 && *PROTOCOL == "http") || (*PORT == 443 && *PROTOCOL == "https") {
            format!("{}://{}", *PROTOCOL, *HOST)
        } else {
            format!("{}://{}:{}", *PROTOCOL, *HOST, *PORT)
        }
    });
    pub static ref CONFIG: Config = Config {
        host: HOST.to_string(),
        port: *PORT,
        domain: DOMAIN.to_string(),
        base_url: BASE_URL.to_string(),
    };
}
