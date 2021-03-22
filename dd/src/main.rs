// fn main(){
//     let s1:&str = "xx";
//     {
//         let s2:&str = "xxx";
//         let s6 = the_longest(s1, s2);
//         print!("{}",s6);
//     }
// }

// fn the_longest(s3: &str, s4: &str) -> &str{
//     if s3.len() > s4.len() {s3} else {s4}
// }

// trait Shape {
//     fn area(&self) -> f64;
//     fn width() -> f64;
// }

// fn main(){

// }

// struct Circle{
//     radius: f64
// }

// // 定义方法集
// trait ShapeAction{
//     fn area(&self) -> f64;
//     fn echo(){
//         print!("helloworld");
//     }
// }

// // 实现方法集
// impl ShapeAction for Circle{
//     fn area(&self) -> f64 {
//         std::f64::consts::PI *self.radius*self.radius
//     }
//     fn echo() {
//         print!("实现 Circle")
//     }

// }

// // 匿名trait
// impl Circle{
//     fn get_radius(&self) -> f64 { self.radius}
// }

// fn main(){
//     let c1 = Circle{
//         radius:1f64,
//     };
//     let c1_area = c1.area();
//     print!("{}",c1_area);

//     c1.get_radius(); // 默认实现

//     Circle::echo();
// }

// use core::fmt;
// use std::process::Command;

// struct Circle{
//     x: f64,
//     y: f64,
//     redius:f64,
// }

// struct CircleBuilder{
//     x:f64,
//     y:f64,
//     redius:f64,
// }

// impl Circle{
//     fn area(&self) -> f64{
//         std::f64::consts::PI *(self.redius *self.redius)
//     }
//     fn new() -> CircleBuilder{
//         CircleBuilder{
//             x:0.0,
//             y:0.0,
//             redius:1.0,
//         }
//     }
// }

// impl CircleBuilder{
//     fn gen_x(&mut self, coordinate: f64) -> &mut CircleBuilder{
//         self.x = coordinate;
//         self
//     }
//     fn gen_y(&mut self, coordinate: f64) -> &mut CircleBuilder{
//         self.x = coordinate;
//         self
//     }
//     fn gen_redius(&mut self, coordinate: f64) -> &mut CircleBuilder{
//         self.x = coordinate;
//         self
//     }
//     fn build(&self) -> Circle{
//         Circle{
//             x: self.x,
//             y: self.y,
//             redius: self.redius
//         }
//     }
// }

// fn main(){
//     let c = Circle::new()
//     .gen_x(20.0)
//     .gen_y(20.0)
//     .gen_redius(10.0)
//     .build();
//     Command::new("ls")
//     .arg("-l")
//     .arg("-a")
//     .spawn()
//     .expect("ls command failed to start");
//     print!("{}",c.area());
// }




/*
fn main() {
    let mut Str_xx = String::from("helloworld"); // 0x7fbfb1402e40
                                                 // 获取堆中字节序列的指针地址
    print!("{:p}\n", Str_xx.as_ptr()); // 0x7ffee2cf74f8
                                       // 获取栈中字节序列的指针地址
    print!("{:p}\n", &Str_xx);
    // 获取堆中字节序列的字节长度
    print!("{}", Str_xx.len()); // 10
                                // 获取堆中容量的长度
    print!("{}", Str_xx.capacity()); // 10
                                     // 添加分配容量
    Str_xx.reserve(20); // 20 + 10 = 30
    assert_eq!(Str_xx.capacity(), 30);

    // -----------

    let string: String = String::new(); // 创建空字符串 容量为0
    println!("{}", string.capacity());
    assert_eq!("", string);
    let string: String = String::from("hello"); // 创建字符串并赋值
    let string: String = String::with_capacity(20); // 创建容量为20的空字符串
    let str: &'static str = "x  x"; // 创建字符串常量
    let string: String = str 
                    .chars() // 转换成utf8的迭代器
                    .filter(|c| !c.is_whitespace())// 判断每一个元素如果不是空格就返回
                    .collect(); // 迭代器的collect⽅法来⽣成String类型的字符串
    println!("{:?}",string);
    let string: String = str.to_owned(); // 比 to_string 快 to_owned⽅法利⽤&str 切⽚字节序列⽣成新的String字符串
    let string: String = str.to_string(); // 对 String::from()的包装 String::from 去调用 to_owned
    let str :&str = &string[0..string.len()];
    assert_eq!("x  x",str);


    // 返回charts 迭代器
    let string = "helloworld";
    let mut chars = string.chars();
    print!("{:?}",chars); // Chars(['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'])
    for i in string.chars(){
        println!("{}",i);
    }
    for i in string.bytes(){
        println!("{}",i)
    }
    let string  = String::from("xxxxx");
    println!("{:?}",string.bytes()); // 迭代器
    println!("{:?}",string.into_bytes()); // 数组


    // 获取字符串切片
    let string = "helloworld";
    let string_slice =  string.get(0..4);
    // 判断值为some还是none
    // let flag_bool = string.get(1..4).is_none();
    // let flag_bool = string.get(1..4).is_some();
    // 获取可变
    let mut xx = string.to_owned();
    let xx_mut_slice = xx.get_mut(0..4);
    match xx_mut_slice {
        Some(value) => println!("{}",value), // hell
        None => println!("{}","None"),
    }

    // 根据索引位切割
    let string  = String::from("foobar");
    let (foo, bar) = string.split_at(3); //  会在第三位后分割
    assert_eq!(foo,"foo");
    assert_eq!(bar,"bar");


    let mut string: String = String::from("foobar");
    let (mut foo, mut bar ) = string.split_at_mut(3);
    foo.to_uppercase();
    bar.to_uppercase();
    assert_eq!(foo,"foo");
    assert_eq!(bar,"bar");

    // 检验某index是否为合法边界
    let flag_bool = string.is_char_boundary(20);
    assert_eq!(flag_bool,false);

    // 按字符串切割
    let string  = String::from("foo|bar");
    let mut res = string.split("|");
    println!("{:?}",res.next().expect("None")); // foo
    println!("{:?}",res.next().unwrap());// bars

    // 字符串拼接
    let mut ss = String::from("xx");
    ss.push_str("x"); // xxx
    assert_eq!(ss,"xxx");
    ss.push('x'); // xxxx
    assert_eq!(ss,"xxxx");

    // 通过迭代器添加
    let mut string = String::from("hell");
    string.extend(['o','!'].iter());
    assert_eq!(string,"hello!");
    let string_extend = String::from("rust");
    string.extend(string_extend.chars());
    assert_eq!("hello!rust",string);
    
    // 指定位置插入字符串
    let mut string = String::with_capacity(20);
    string.insert(0, '1');
    string.insert_str(1, "xx");
    print!("{}",string);

    
    /*
        pub fn insert_str(&mut self, idx: usize, string: &str) {
        assert!(self.is_char_boundary(idx));

        unsafe {
            self.insert_bytes(idx, string.as_bytes());
        }
    }
    */
    // 内部调用self.is_char_boundary(index) 看是否超出界限

    // + += 拼接字符串
    let mut left:String =  String::from("left-");
    let right:&str = "right";
    left += "!";
    let left_right = left+right; // move
    println!("{}",left_right);

}

*/

/*
fn main(){
    let xx = "foobar".to_owned();
    assert_eq!(xx.contains("foo"),true)
}
*/

// struct Circle{
//     radius: f64
// }

// // 定义接口
// trait ShapeAction{
//     fn area(&self) -> f64; // 方法
//     fn echo() ; // 静态函数
// }

// // 实现接口
// impl ShapeAction for Circle{
//     fn area(&self) -> f64 {
//         std::f64::consts::PI *self.radius*self.radius
//     }
//     fn echo(){
//         println!("None");
//     }
// }

// fn main(){
//     let c1 = Circle{
//         radius:1f64,
//     };
//     let c1_area = c1.area();
//     assert_eq!(1f64,c1.radius);
//     Circle::echo(); // None
// }

// fn main(){
//     let x:Option<i32> = Some(1);
//     let x:Option<bool> = Some(true);
//     let x:Option<&str> = Some("x");
//     let x:Option<&str> = None;
//     match x {
//         Some(value) => println!("{:?}",value),
//         None => println!("is None"),
//     }
    
// }

// struct S<T>{
//     data: T
// }
// fn main(){
//     let d1:S<i32>  = S{
//         data:2,
//     };
//     let d2:S<&str> = S::<&str>{
//         data:"xx"
//     };
//     assert_eq!(2,d1.data);
//     assert_eq!("xx",d2.data);
// }

// fn main(){
//     let op1 = Some(1);
//     let op2 = Some(2);
//     let flag = ops(op1,op2);
//     println!("{}",flag);
//     let s = "xx";
//     s.contains(pat)
    
// }

// fn ops<T>(o1: Option<T>, o2:Option<T>) ->bool {
//     let flag = match (o1,o2) {
//         (Some(v1),Some(v2)) => true,
//         (None,None) => true,
//         _ => false,
//     };
//     flag
// }       


// trait Foo : Sized { // all types that implement foo will be sized, so 
//     // self has to be sized
//     fn show(&self);
// }

// struct Bar{}

// impl Foo for Bar { // Bar is now considered sized, at least when calling show()
//     fn show(&self) {
//         println!("42");
//     }
// }

// fn main() {
//     let x = &Bar{};
//     x.show(); // Bam! Problem is here, right?
//     let xs = "xx";
//     xs.contains(pat)
// }

// use std::fmt::Debug;

// fn dbg<T>(t: T) where T: Debug { // T: Debug + Sized
//     println!("{:?}", t);
// }

// fn main() {
//     dbg("my str".to_owned()); // &T = &str, T = str, str: Debug + !Sized ❌
// }
// use std::iter::Iterator
// fn main(){
//     let xx = "xx";
//     let count = xx.chars().next();
//     let qq = vec![1,2,3];
//     qq.iter().next();
// }   

// use std::iter::Iterator;
// use std::fmt::Debug;

// fn use_iter<ITER>(mut iter: ITER)
//     where ITER: Iterator,
//         ITER::Item: Debug,
// {
//     while let Some(i) = iter.next(){
//         println!("{:?}",i)
//     }
// }

// fn main(){
//     let v1:Vec<i32> = Vec::<i32>::new();
//     let v2:Vec<String> = Vec::with_capacity(200);
//     let v3:Vec<i32> = vec![1,2,3];
    
//     // 插入数据
//     let mut v4:Vec<i32> = Vec::new();
//     println!("{}",v4.capacity()); // 0
//     v4.push(1);
//     assert_eq!(v4.capacity(),1);
//     v4.extend_from_slice(&[1,2,3,4,5]);
//     println!("{}",v4.capacity()); // 6
//     v4.insert(2,100);
//     println!("{}",v4.capacity()); // 12

//     // 访问数据
//     v4[5] = 5;
//     let i = v4[5];
//     assert_eq!(i,5);
//     // Index 运算符直接访问,如果越界则会造成 panic,而 get 方法不会,因为它返回一个 Option<T>
//     if let Some(i) = v4.get(6){
//         println!("{}",i)
//     }
//     let slice = &v4[4..];
//     println!("{:?}",slice);
// }

// use std::collections::VecDeque;
// fn main(){
//     let mut queue = VecDeque::with_capacity(64);
//     for i in 1..10{
//         queue.push_back(i)
//     }
//     while let Some(i) = queue.pop_front(){
//         println!("{}",i)
//     }
// }

// use std::iter::Iterator;

// struct Name{
//     first: String,
//     second: String
// }

// impl Name {
//     fn new() -> Self {
//         Name{
//             first: "Default".to_owned(),
//             second: "default".to_owned(),
//         }
//     }
// }

// impl Iterator for Name{
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut value = self.first.to_owned();
//         if value != ""{
//             return  Some(value+ &self.second);
//         }else{
//             return None
//         }
//     }
// }

// fn main(){
//     let mut n1 = Name::new();
//     let mut xx = "xx".chars();
//     if let Some(i) = xx.next(){
//         println!("{}",i)
//     }

// }

// 


// fn main(){
//     let v = vec![1,2,3,4,5,6];
//     let mut vector = v.iter().take(2);
//     println!("{:?}",vector); // Take { iter: Iter([1, 2, 3, 4, 5, 6]), n: 2 }
    
//     let v = vec![1,2,3,4,5,6];
//     let mut vector = v.iter()
//         .take(2) // Take { iter: Iter([1, 2, 3, 4, 5, 6]), n: 2 }
//         .filter(|&x| x %2 == 0) // Filter { iter: Take { iter: Iter([1, 2, 3, 4, 5, 6]), n: 2 } }
//         .map(|&x|x*x) // Map { iter: Filter { iter: Take { iter: Iter([1, 2, 3, 4, 5, 6]), n: 2 } } }
//         .enumerate(); // Enumerate { iter: Enumerate { iter: Map { iter: Filter { iter: Take { iter: Iter([1, 2, 3, 4, 5, 6]), n: 2 } } }, count: 0 }, count: 0 }
//     if let Some((i,v)) = vector.next() {
//         println!("{}{}",i,v);
//     }

// }


// macro_rules! hashmap {
//     ($($key:expr => $val: expr),*) => {{
//         let mut map = ::std::collections::HashMap::new();
//         $(map.insert($key,$val);)*
//         map;
//     }};
// }
// fn main(){
//     let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
//     // 实际创建
//     let counts =
//         {
//             let mut map = ::std::collections::HashMap::new();
//             map.insert('A', 0);
//             map.insert('C', 0);
//             map.insert('G', 0);
//             map.insert('T', 0);
//             map
//         };
// }

// use std::convert::From;

// struct Number{
//     value :i32
// }


// impl From<i32> for Number {
//     fn from(item:i32) -> Self {
//         Number{
//             value: item
//         }
//     }
// }

// fn main(){
//     let x = "xx";
//     let xx = String::from(x);
//     let yy:String = x.into();

//     let n1 = Number::from(20);
//     let n2:Number = 20.into();
//     let n3:Number = 30.into();


//     // let xx = String::from("xx");
//     // let xxx = String::from(xx); // String
//     // let xx = String::from("xx");
//     // let xxx_into:String = xx.into();
//     // assert_eq!(xxx,xxx_into);

//     // let xx = String::from("xx"); // &str
//     // let xx_into:String = "xx".into();
//     // assert_eq!(xx,xx_into);

//     // let mut x = "xx";
//     // let xx  = String::from(x); // &mut str
//     // let mut x = "xx";
//     // let xx_into:String  = x.into();
// }


// fn main(){
//     let xx = "xx";
//     is_hello_str(xx);

//     let yy = "xx".to_owned();
//     is_hello_String(yy);
// }


// fn is_hello_str(s:&str)
// {
//     assert_eq!("xx",s)
// }

// fn is_hello_String(s:String)
// {
//     assert_eq!("xx",s.as_str());
// }



// fn main(){
//     let xx = Some(32);
//     let yy = xx.expect("xx");
//     let x: Result<i32, &str> = Ok(-3);
//     let y = x.expect("xx");
//     x.unwrap_err()
// }

// fn main(){
//     let res = get_res(100);
//     print!("{:?}",res.is_err())
    
// }

// fn get_res(index:i32) -> Result<i32, MathError>{
//     let res = is_ok(index)?;
//     Ok(res)
// }

// #[derive(Debug)]
// enum MathError {
//     _DivisionByZero,
//     _NegativeLogarithm,
//     TooBig

// }

// fn is_ok(n1:i32) -> Result<i32,MathError>
// {
//     if n1 > 20{
//         Err(MathError::TooBig)
//     }  else {
//         Ok(32)
//     }
// }

/*
    [dependencies]
    serde_json = "1.0"
*/



/* 
use core::fmt;

use fmt::Error;
use serde_json::{Result, Value, error, json};
use serde_json::Deserializer;

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}


#[derive(Debug)]
struct Data<'a>{
    name:&'a str,
    age: i32,
    phones:i32,
}

fn main() -> Result<()>{
    untyped_example().unwrap();



    let xx = json!({
        "code":"xx",
        "name":"hzj",
        "foo":"bar",
    });
    println!("{}",xx);


    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    // let jsoned_data = gen_json_code(&data).unwrap();
    // println!("{:?}",jsoned_data);
    let json_data: Value = serde_json::from_str(data)?;
    println!("{}",json_data);


    let js_data = serde_json::from_value(value)
    Ok(())
}


*/


// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "UPPERCASE")]
// struct Point {
//     // #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]     // 反序列化后key值为x 序列化后key值为a
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let point = Point{
//         x:1,
//         y:2,
//     };
//     let ser_point = serde_json::to_string(&point).unwrap();
//     println!("{}",ser_point); //  {"X":1,"Y":2}
// }

// use clap::{Arg, App};

// fn main() {
//     let matches = App::new("My Super Program".to_owned())
//         .version("1.0")
//         .author("Kevin K. <kbknapp@gmail.com>")
//         .about("Does awesome things")
//         .arg(Arg::new("config")
//             .short('c')
//             .long("config")
//             .value_name("FILE")
//             .value_name("another file")
//             .about("Sets a custom config file")
//             .takes_value(true))
//         .arg(Arg::new("INPUT")
//             .about("Sets the input file to use")
//             .required(true)
//             .index(1))
//         .arg(Arg::new("v")
//             .short('v')
//             .multiple(true)
//             .takes_value(true)
//             .about("Sets the level of verbosity"))
//         .subcommand(App::new("test")
//             .about("controls testing features")
//             .version("1.3")
//             .author("Someone E. <someone_else@other.com>")
//             .arg(Arg::new("debug")
//                 .short('d')
//                 .about("print debug information verbosely")))
//         .get_matches();

//     // You can check the value provided by positional arguments, or option arguments
//     if let Some(i) = matches.value_of("INPUT") {
//         println!("Value for input: {}", i);
//     }

//     if let Some(c) = matches.value_of("config") {
//         println!("Value for config: {}", c);
//     }

//     // You can see how many times a particular flag or argument occurred
//     // Note, only flags can have multiple occurrences
//     match matches.occurrences_of("v") {
//         0 => println!("Verbose mode is off"),
//         1 => println!("Verbose mode is kind of on"),
//         2 => println!("Verbose mode is on"),
//         3 | _ => println!("Don't be crazy"),
//     }

//     // You can check for the existence of subcommands, and if found use their
//     // matches just as you would the top level app
//     if let Some(ref matches) = matches.subcommand_matches("test") {
//         // "$ myapp test" was run
//         if matches.is_present("debug") {
//             // "$ myapp test -d" was run
//             println!("Printing debug info...");
//         } else {
//             println!("Printing normally...");
//         }
//     }

//     // Continued program logic goes here...

// }




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

// use std::borrow::Borrow;

// use clap::{App, Arg, Error};
// use std::result::{Result};

// enum Action{
//     Do(),
//     Echo(),
//     Print(),
// }

// struct Pet{
//     name:String,
//     age:i32
// }

// impl Pet{
//     fn run(&self,s:Action){
//         match s {
//             Action::Do() => println!("do"),
//             Action::Echo() => println!("echo"),
//             Action::Print() => println!("print"),
//             _ => println!("else")
//         }
//     }
// }

// fn command_to_action() -> Result<Action,()>{
//     let command = App::new("acionForPet")
//         .author("hzj")
//         .about("command to action")
//         .name("command")
//         .override_usage("usage")

//         .arg(Arg::new("do")
//             .short('d')
//             .long("do")
//             .about("do action"))

//         .arg(Arg::new("echo")
//             .short('e')
//             .long("echo")
//             .about("echo action"))

//         .get_matches();
    
//     if let Some(key) = command.value_of("do"){
//         return Ok(Action::Do());
//     } 
//     else if let Some(key) = command.value_of("echo"){
//         return Ok(Action::Echo())
//     }
//     else {
//         return Ok(Action::Print());
//     }
// }

// pub(crate) fn main() -> Result<(),()> {

//     let p1 = Pet{
//         name:"xxxx".to_owned(),
//         age:23
//     };
//     p1.run(command_to_action()?);


//     Ok(())
// }


// fn main(){
//     // 整型
//     let a:i32 = 30;
//     // 浮点型
//     let b:f32 = 30.1;
//     // 布尔型
//     let c:bool = true;
//     // 字符串字面量
//     let d:&'static str = "xx";
//     // 字符串
//     let e:String =  String::from("xx");
//     // 元祖
//     let tup:(i32,i32) = (2,30);
//     assert_eq!(tup.0 , 2);
//     assert_eq!(tup.1 , 30);
    
//     // 静态数组
//     let f:[i32;3] = [1,2,3];
//     assert_eq!(f[0] , 1);
//     assert_eq!(f[0] , 2);
// }

// #[derive(Debug)]
// struct Peoson{
//     name:String,
//     age:i32
// }



// fn main(){
//     let p2 = Peoson{
//         age:32,
//         name:"xx".to_owned(),
//     };
//     let p3 = Peoson{
//         ..p2 // p2 moved here
//     };
//     println!("{:?}",p2);
// }

// enum MathMethods {
//     Add(i32,i32),
//     Sub(i32,i32)
// }

// impl MathMethods {
//     fn do_action(&self) -> i32{
//         match self {
//             MathMethods::Add(x,y) =>  x + y,
//             MathMethods::Sub(x,y) => x - y
//         }
//     }
// }

// struct People{
//     name:String,
//     age:i32
// }

// impl People {
//     fn run(&self, action: MathMethods){
//         action.do_action();
//     }
// }

// fn main(){
//     let p1 = People{
//         name:"xx".to_owned(),
//         age:30,
//     };
//     if p1.age > 30{
//         p1.run(MathMethods::Add(2,20))
//     } else {
//         p1.run(MathMethods::Sub(2,30))
//     }
    
// }


// enum Class {
//     People {name:String,age:i32},

// }

// impl Drop for Class {
//     fn drop(&mut self) {
//         println!("i will drop")
//     }
// }

// fn main(){
//     let c1 = Class::People{
//         name:"xx".to_owned(),
//         age:32,
//     };
// }


/// -----------------------------------------------------------------------------
// use std::env;

// use env::current_dir;
// use std::path::Path;
// fn main(){
//     // println!("{:?}",env::args().into_iter().next());
//     println!("{:?}",env::args().into_iter()); // 编译后文件夹 + 执行程序名+ 命令行参数

//     // 返回当前所在文件夹 io::Result<PathBuf>
//     println!("{:?}",env::current_dir().unwrap());
    
//     // 返回当前所在文件夹
//     println!("{:?}",env::current_exe().unwrap());

//     // 拼接文件路径
//     let current_path  = env::current_dir().unwrap();
//     let all_path = env::join_paths([
//         Path::new("xx/"),
//         Path::new("yy/")
//     ].into_iter()).unwrap();

//     println!("{:?}",all_path);
//     println!("{:?}",env::current_dir().unwrap());
    
// }
/// -----------------------------------------------------------------------------


// fn main() -> std::io::Result<()>{

//     // writer info to file
//     std::fs::write("xx.txt", "xxx")?;
//     std::fs::write("yy.txt", "yyy")?;


//     // copy file_a  content to file_b content
//     std::fs::copy("yy.txt", "xx.txt")?;

//     Ok(())
// }


// use std::io::{self, Write};
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// fn write_green() -> io::Result<()> {
//     let mut stdout = StandardStream::stdout(ColorChoice::Always);
//     stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
//     writeln!(&mut stdout, "green text!")?;
//     Ok(())
// }
// fn main() -> io::Result<()>{
//     write_green()
// }

use colored::*;

// test the example with `cargo run --example most_simple`
fn main() {
    // TADAA!
    println!("{} {} !", "it".green(), "works".blue().bold(),"test".truecolor(0, 255, 136));
}