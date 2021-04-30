
use clap::*;
use ipnet::{IpNet, Ipv4Net};

fn main(){
    let matcher = clap::App::new("cird")
        .author("hzjsea")
        .version("1.0")
        .about("display ipaddr range")
        .after_help("Welcome to use")
        .arg(
            Arg::with_name("ipv4")
            .long("ipv4")
            .takes_value(true)
            .multiple(true)
            .require_equals(true)
        )
        .arg(
            Arg::with_name("ipv6")
            .long("ipv6")
            .takes_value(true)
            .multiple(true)
            .require_equals(true)
        )
        .get_matches();
    

        // [CIDR]
        // Host address		- 10.0.6.55
        // Host address (decimal)	- 167773751
        // Host address (hex)	- A000637
        // Network address		- 10.0.6.52
        // Network mask		- 255.255.255.252
        // Network mask (bits)	- 30
        // Network mask (hex)	- FFFFFFFC
        // Broadcast address	- 10.0.6.55
        // Cisco wildcard		- 0.0.0.3
        // Addresses in network	- 4
        // Network range		- 10.0.6.52 - 10.0.6.55
        // Usable range		- 10.0.6.53 - 10.0.6.54
    

    let ipv4_value = matcher.value_of("ipv4").unwrap();
    parse_ipv4(ipv4_value);

    let ipv6_value = matcher.value_of("ipv6").unwrap();
    parse_ipv6(ipv6_value);



    // let res = ip_value.parse::<ipnet::Ipv6Net>().expect("is not ipv6 addr" );
    // println!("{:?}",res)
    
}

fn parse_ipv4(ip_value:&str){
    let ipv4 = ip_value.parse::<Ipv4Net>().expect("is not ipv4 address");
    let ipv4_range = ipv4.hosts().collect::<Vec<std::net::Ipv4Addr>>();
    
    let start = ipv4_range[0];
    let end = ipv4_range[ipv4_range.len()-1];
    let network = ipv4.hostmask();
    let host = ipv4.network();
    let mask = ipv4.netmask();
    let broadcast = ipv4.broadcast();

    println!("
    host: {}
    network: {}
    netmask: {}
    broadcast: {}
    Network range: {} {}
    ", host,network,mask,broadcast,start,end)
    
}


fn parse_ipv6(ip_value:&str){
    let ipv6 = ip_value.parse::<ipnet::Ipv6Net>().expect("is not ipv6 address");
    let ipv6_range = ipv6.hosts().collect::<Vec<std::net::Ipv6Addr>>();
    
    let start = ipv6_range[0];
    let end = ipv6_range[ipv6_range.len()-1];
    let network = ipv6.hostmask();
    let host = ipv6.network();
    let mask = ipv6.netmask();
    let broadcast = ipv6.broadcast();

    println!("
    host: {}
    network: {}
    netmask: {}
    broadcast: {}
    Network range: {} {}
    ", host,network,mask,broadcast,start,end)
    
}


// fn main() {
//     let t = 10; // 0x7ffee3c77388
//     let t2 = t; // 0x7ffee3c7738c
//     println!("{:p}, {:p}", &t, &t2);
// }

// fn main(){
//     let mut total = 1;
//     loop {
//         let (n,flag) = monkey_eat(total,true);
//         if flag && n==1{
//             println!("{}",total);
//             break;
//         } else{
//             total +=1
//         }
//     }
// }

// fn monkey_eat(total:i32,flag:bool) -> (i32,bool){
//     let mut n = total;
//     for _i in 0..9{
//         if n % 2 == 0 && n > 0{
//             n = n - n/2 -1
//         } else {
//             return (n,false)
//         }
//     }
//     (n,flag)
// }