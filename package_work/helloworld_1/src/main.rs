fn main() {
    // 相对路径
    say::hello();
    // 绝对路径调用 // src即为根目录
    crate::say::hello();

    // 无法调用
    // say::hello_2(); // error , because it's not public

    say::hi::hi_1();
    say::hi::hi_2();
    // 使用 use 后就可以这么调用
    use crate::say::hi;
    hi::hi_1();

    // 重导出名称
    people::hi::hi_1();
    people::hello();
    // 但是不能 
    // people::say::hello();

    people_2::people::hello();
    people_2::info::name();
}

mod say {
    pub fn hello() {
        println!("Hello, world!");
    }
    fn hello_2() {
        println!("hello")
    }
    pub mod hi {
        pub fn hi_1() {
            super::hello_2();
        }
        pub fn hi_2() {
            println!("hi there");
        }
    }
}

pub mod people {
    // 调用pub的就用pub声明 
    pub use crate::say::hi;
    // 调用不是pub的就不能用pub声明
    use crate::say;
    pub fn hello() {
        say::hello();
    }
    pub mod info {
        pub fn name() {
            println!("hzjsea");
        }
    }
}


mod people_2 {
    // 调用局部 self 代指 people mod下面的所有方法，而非指所有方法
    pub use crate::people::{self, info};
    pub fn hello() {
        info::name();
    }
}
