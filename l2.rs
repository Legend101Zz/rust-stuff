enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address : String
}

fn main() {
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

let localhost = IpAddrKind::V4("127.0.0.1".to_string());
}
