use std::fs;

#[derive(Clone)]
pub struct AppState {
    pub public_key: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            public_key: fs::read_to_string("./public.pem")
                .expect("Should be able to read public key"),
        }
    }
}
