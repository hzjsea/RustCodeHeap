fn main() {
    helloworld::echo();
    println!("Hello, world!");
}


mod helloworld{
    pub fn echo(){
        println!("helloworld_echo")
    }
}
