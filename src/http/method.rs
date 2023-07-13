pub enum Method {
    GET(String),
    POST(u64),
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
