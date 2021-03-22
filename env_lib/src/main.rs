use std::path;

fn main() -> std::io::Result<()>{
    // 获取所有环境变量
    let env_vars = std::env::vars();

    for (key,value ) in env_vars{
        println!("{}{}",key,value)
    };
    
    println!("\n");
    // 获取所有命令行参数
    let command_args = std::env::args();
    
    for (index,value) in command_args.enumerate(){
        println!("{}-{}",index,value)
    }

    println!("\n");
    // 返回当前路径 current_dir,
    let file_path = std::env::current_dir();
    match file_path {
        Ok(path_buf) => println!("{:?}",path_buf.into_os_string()),
        Err(_) => println!("is err")
    }

    // 拼接文件路径
    let file_path = std::env::join_paths([std::path::Path::new("/xx/xx"),std::path::Path::new("/yy/yy")].into_iter());
    match file_path {
        Ok(osS) => println!("{:?}",osS),
        Err(_) => println!("is err")
    }

    let key = "PATH";
        match std::env::var_os(key) {
        Some(paths) => {
            for path in std::env::split_paths(&paths) {
                println!("'{}'", path.display());
            }
        }
        None => println!("{} is not defined in the environment.", key)
    }

    Ok(())
}
