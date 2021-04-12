use core::fmt;
use std::{default, io::stdin, net::*};
use std::io::{Result,ErrorKind};
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr,AddrParseError};
use std::default::Default;
use ipnet::{IpSub, Ipv4AddrRange};
use ipnet::{Ipv4Net, PrefixLenError};
// 如果下抛错误的话，不同的错误该怎么处理

enum IpAddrType {
    Ipv4Addr,
    Ipv6Addr,
    NotFound,
}
impl Default  for IpAddrType {
    fn default() -> Self {
        IpAddrType::NotFound   
    }
}


#[derive(Default)]
struct Netmask{
    ip_addr : String,
    ip_type : IpAddrType
}


impl Netmask {

    fn new() -> Self{
        Default::default()
    }
}


fn main() -> Result<()>{
    let mut addr = String::new();
    let readed_size = std::io::stdin().read_line(&mut addr)?;



    // deal with type of the IP address
    if readed_size > 0 {
        addr = addr.trim().parse().unwrap();
        match addr.parse::<IpAddr>() {
            Ok(value) => {
                println!("{}",value)
            },
            Err(error) => {
                if let Ok(value) = addr.parse::<Ipv4Addr>(){
                    println!("{}",value) 
                } 
                if let  Ok(value) = addr.parse::<Ipv6Addr>(){
                    println!("{}",value) 
                }
                println!("{}",error)
            }
        
        }
    }


    // start  and end 
    // let hosts = Ipv4AddrRange::new(
    //     "10.0.0.0".parse().unwrap(),
    //     "10.0.0.3".parse().unwrap(),
    // );
    // println!("{:?}",hosts);

    // let net = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24).unwrap();
    // println!("{:?}",net);
    // println!("{:?}",net.addr());


    // let subnets = ipnet::Ipv4Subnets::new(
    //     "10.0.0.0".parse().unwrap(),
    //     "10.0.0.239".parse().unwrap(),
    //     26,
    // );
    // println!("{:?}", subnets);

    // let ip1: Ipv4Addr = "192.168.1.5".parse().unwrap();
    // println!("{:?}",ip1.saturating_sub(20));

    // let net: ipnet::IpNet = "10.0.0.0/24".parse().unwrap();

    // let nets = vec![
    //     "10.0.0.0/24".parse::<ipnet::IpNet>().unwrap(),
    //     "10.0.1.0/24".parse().unwrap(),
    //     "10.0.2.0/24".parse().unwrap(),
    //     "fd00::/18".parse().unwrap(),
    //     "fd00:4000::/18".parse().unwrap(),
    //     "fd00:8000::/18".parse().unwrap(),
    // ];
    // let res = ipnet::IpNet::aggregate(&nets);
    // println!("{:?}",res);
    

    // let ipnets = ipnet::Ipv4Net::new(IpAddr,27);
    

    // let mut hosts = ipnet::IpAddrRange::from(Ipv4AddrRange::new(
    //     "10.0.0.0".parse().unwrap(),
    //     "10.0.0.3".parse().unwrap(),
    // ));
    // for i in hosts{
    //     println!("{:?}",i)
    // }
    // 
    Ok(())
}

