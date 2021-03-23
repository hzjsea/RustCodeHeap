

// // An Echo program:
// // SENDER -> sends a message.
// // ECHOER -> listens and prints the message received.

// use std::time::Duration;

// use mio::{Events, Poll, Token};
// // use std::{ffi::IntoStringError, time::Duration};
// use mio:: Interest;

// const SENDER: Token = Token(0);
// const ECHOER: Token = Token(1);


// fn main() -> std::io::Result<()>{

//     let mut events = Events::with_capacity(128);
//     // create a poll instance
//     let address = "127.0.0.1:0".parse().unwrap();
//     let mut server = mio::net::UdpSocket::bind("127.0.0.1:0".parse().unwrap())?;
//     // let mut server = mio::net::UdpSocket::bind(address)?;
//     let mut client =  mio::net::UdpSocket::bind(address)?;

//     client.connect(server.local_addr()?)?;


//     let mut poll = Poll::new()?;
//     poll.registry().register(&mut server, SENDER, Interest::READABLE | Interest::WRITABLE)?;
//     poll.registry().register(&mut client, ECHOER, Interest::READABLE | Interest::WRITABLE)?;

//     let msg_to_send = [9; 9];
//     let mut buffer = [0; 9];


//     loop {
//         poll.poll(&mut events, Some(Duration::from_millis(100)))?;
//         for event in events.iter() {
//             match event.token() {
//                 // Our SENDER is ready to be written into.
//                 SENDER => {
//                     let bytes_sent = client.send(&msg_to_send)?;
//                     println!("sent {:?} -> {:?} bytes", msg_to_send, bytes_sent);
//                 },
//                 // Our ECHOER is ready to be read from.
//                 ECHOER => {
//                     let num_recv = server.recv(&mut buffer);
//                     println!("echo {:?} -> {:?}", buffer, num_recv);
//                     buffer = [0; 9];
//                 }
//                 _ => unreachable!()
//             }
//         }
//     }
//     // Ok(())

// }


fn main(){
}


