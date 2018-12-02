use str_to_enum_derive::StrToEnum;

#[derive(StrToEnum, Debug, PartialEq, Eq)]
enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

fn main() {
    assert_eq!(HTTPMethod::GET, "GET".parse::<HTTPMethod>().unwrap());
    assert_eq!(HTTPMethod::HEAD, "HEAD".parse::<HTTPMethod>().unwrap());
}
