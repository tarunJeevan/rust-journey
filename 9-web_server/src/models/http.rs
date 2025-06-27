pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Options,
    None,
}

pub enum HttpProtocol {
    Default(String),
}

impl Default for HttpProtocol {
    fn default() -> Self {
        HttpProtocol::Default("HTTP/1.1".to_string())
    }
}
