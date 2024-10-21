#[derive(Debug)]
enum ipvaddr {
    ipv4,
    ipv6
}

#[derive(Debug)]
struct ipaddr{
    kind: ipvaddr,
    address: String
}

fn main(){

    let home = ipaddr{
        kind: ipvaddr::ipv4,
        address: String::from("127.0.0.1"),
    };

    let loopback = ipaddr{
        kind: ipvaddr::ipv6,
        address: String::from("::1")
    };
    
    println!("{:?}", home);
    println!("{:?}", loopback);

}
