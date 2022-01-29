

enum IpAddrKind {
    v4,
    v6,
}
enum IpNewAddr {
    v4(u8,u8,u8,u8),
    v6(String),
}
struct IpStuc {
    kind: IpAddrKind,
    address: String,

}

fn main() {
    println!("Hello, world!");
    let for4 = IpAddrKind::v4;
    let fo6 = IpAddrKind::v6;
    route(for4);
    route(fo6);
    let ghome = IpStuc{
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1")
    };

}

fn route(ip_kind : IpAddrKind) {

}
