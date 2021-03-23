fn main() {
    // 判断是否为分隔符号
    assert!(std::path::is_separator('/'));
    assert!(!std::path::is_separator('❤')); // !false
    
    // 获取长地址前缀
    let path = std::path::Path::new("./xx.txt");
    let prefix = path.parent();
    // file name
    let suffix = path.file_stem();
    // 文件后缀名
    let extension = path.extension();
    println!("{:?}",extension);
    println!("{:?}",prefix);    
    assert_eq!(prefix,Some(std::path::Path::new("./"))); // true


    // 拼接地址
    let path: std::path::PathBuf = ["c:\\", "windows", "system32.dll"].iter().collect();
    println!("{:?}",path);
    
    // 拼接地址
    let mut  path = std::path::Path::from("./xx.txt");
    path.push("xx");
    println!("{:?}",path);
}

