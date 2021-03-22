// // struct People{
// //     name: String,
// //     age: i32,
// // }

// // impl People {
// //     fn todo(&self){
// //         println!("{}",self.age)
// //     }
// // }

// // impl People{
// //     #[inline]
// //     fn todo2(&self){
// //         println!("{}",self.name)
// //     }
// // }

// // #[derive(Default)]
// // struct Person{
// //     name: String,
// //     age: i32,
    
// // }

// // impl Person{
// //     fn todo2(&self){
// //         println!("{}",self.name)
// //     }
// // }

// // fn main(){
// //     let p2 = Person::default();
// //     let p3  = Person{
// //         name:"hzj".to_owned(),
// //         ..Default::default()
// //     };
// //     p2.todo2();
// // }

// // use clap::{Arg, App};

// // fn main() {
// //     let matches = App::new("My Super Program".to_owned())
// //         .version("1.0")
// //         .author("Kevin K. <kbknapp@gmail.com>")
// //         .about("Does awesome things")
// //         .arg(Arg::new("config")
// //             .short('c')
// //             .long("config")
// //             .value_name("FILE")
// //             .value_name("another file")
// //             .about("Sets a custom config file")
// //             .takes_value(true))
// //         .arg(Arg::new("INPUT")
// //             .about("Sets the input file to use")
// //             .required(true)
// //             .index(1))
// //         .arg(Arg::new("v")
// //             .short('v')
// //             .multiple(true)
// //             .takes_value(true)
// //             .about("Sets the level of verbosity"))
// //         .subcommand(App::new("test")
// //             .about("controls testing features")
// //             .version("1.3")
// //             .author("Someone E. <someone_else@other.com>")
// //             .arg(Arg::new("debug")
// //                 .short('d')
// //                 .about("print debug information verbosely")))
// //         .get_matches();

// //     // You can check the value provided by positional arguments, or option arguments
// //     if let Some(i) = matches.value_of("INPUT") {
// //         println!("Value for input: {}", i);
// //     }

// //     if let Some(c) = matches.value_of("config") {
// //         println!("Value for config: {}", c);
// //     }

// //     // You can see how many times a particular flag or argument occurred
// //     // Note, only flags can have multiple occurrences
// //     match matches.occurrences_of("v") {
// //         0 => println!("Verbose mode is off"),
// //         1 => println!("Verbose mode is kind of on"),
// //         2 => println!("Verbose mode is on"),
// //         3 | _ => println!("Don't be crazy"),
// //     }

// //     // You can check for the existence of subcommands, and if found use their
// //     // matches just as you would the top level app
// //     if let Some(ref matches) = matches.subcommand_matches("test") {
// //         // "$ myapp test" was run
// //         if matches.is_present("debug") {
// //             // "$ myapp test -d" was run
// //             println!("Printing debug info...");
// //         } else {
// //             println!("Printing normally...");
// //         }
// //     }

// //     // Continued program logic goes here...

// //     // 获取名字
// //     println!("{:?}",matches);
// // }




// /*
// use clap::App;
// use clap::Arg;

// fn main(){
//     let command = App::new("helloworld")
//             .author("author")
//             // option -c
//             .arg(Arg::new("config")
//                 .short('c')
//                 .long("config")
//                 .takes_value(false))
            
//             .get_matches();
// }


// USAGE:
//     xx [FLAGS]

// FLAGS:
//     -c, --config     
//     -h, --help       Prints help information
//     -V, --version    Prints version information
    
// */


// use clap::App;
// use clap::Arg;
// use env::{args_os, vars};

// use core::fmt;
// use std::env;


// fn main(){
//     // let command = App::new("helloworld")
//     //         .author("author")
//     //         // option -c
//     //         .arg(Arg::new("config")
//     //             .short('c')
//     //             .long("config")
//     //             .takes_value(false))
//     //         .arg(Arg::new("testx")
//     //             .short('t')
//     //             .long("testx")
//     //             .takes_value(true)
//     //             .required(true))
//     //         .get_matches();
//     // if let Some(c) = command.value_of("config"){
//     //     println!("{:?}",c);
//     // }

//     let current_dir = env::current_dir().unwrap();
//     println!("{:?}",current_dir);
    
//     let args = env::args_os();
//     println!("{:?}",args)
// }

