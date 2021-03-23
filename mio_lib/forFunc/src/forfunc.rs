
pub mod for_function {

    pub fn root(){
        for_index_value()
    }

    fn for_index_value(){
        let item:[i32;100] = [1;100];
        for (index, value ) in item.iter().enumerate(){
            println!("{},{}",index,value)
        }
    }
}