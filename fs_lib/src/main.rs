use core::fmt;
use std::{io::{Read, Write}, ops::Try};

fn main() -> std::io::Result<()> {

    //  std::fs::DirBuilder
    //  创建文件或者文件夹
    let file_builder = std::fs::DirBuilder::new()
                                .recursive(true) // 是否递归
                                .create("xx/yy/zz/")
                                .unwrap();
    
    //  std::fs::DirEntry
    //  
    for entry in std::fs::read_dir(".")?{
        let dir = entry?;
        println!("{:?}",dir.path())
    }

    // std::fs::File
    let mut file  = std::fs::File::create("./y.txt")?;
    file.write_all(b"buf")?;
    

    let mut file = std::fs::File::open("y.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "buf");


    // std::fs::openOption
    use std::fs::OpenOptions;

    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("foo.txt");

    // std::fs::FileType
    let  file = std::fs::File::open("y.txt")?;
    let flag = file.metadata()?.is_file();
    assert_eq!(flag,true);

    // std::fs::metadata
    let file_type = std::fs::File::open("y.txt")?.metadata()?;
    println!("{:?}",file_type);

    // normal
    
    // 获取文件或者文件夹信息
    let file_metadata = std::fs::metadata("xx/yy/zz").unwrap();
    println!("{:?}",file_metadata.is_dir());

    // 创建文件夹
    // std::fs::create_dir("xx")?;
    // 递归创建文件夹
    // std::fs::create_dir_all("xx/xx/xx")?;
    // println!("Hello, world!");

    // 获取所有文件的后缀名
                        
    Ok(())


    

    
}
