#[derive(Debug)]
enum IPAddType {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddress {
    iPAddType: IPAddType,
    address: String,
}

fn main() {
    let ip1 = IPAddress {
        iPAddType: IPAddType::V4,
        address: String::from("127.0.0.1"),
    };

    let ip2 = IPAddress {
        iPAddType: IPAddType::V6,
        address: String::from("::1"),
    };

    dbg!(ip1);
    dbg!(ip2);
}
