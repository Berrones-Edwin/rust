use std::{
    fs::File,
    io::ErrorKind,
    io::{Error, Read},
};

fn main() {
    //  RUST_BACKTRACE=1 cargo
    //run see log
    // let x = vec![1,2,3,4,5];
    // let z  = x[99];
    // println!("{}", z);

    // panic!("crash and burn");

    // let f = File::open("myfile.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Tried to create file but there was a problem {:?}", e),
    //     },
    //     Err(error) => {
    //         panic!("Tried to create file but there was a problem {:?}", error)
    //     }
    // };

    //    let f = File::open("file.txt").expect("Failed to open hello.txt");
    //    let f = File::open("file.txt").unwrap();

    let f = read_username_from_file();
    match f {
        Ok(s) => print!("{}", s),
        Err(e) => print!("{:?}", e),
    }
}

fn read_username_from_file() -> Result<String, Error> {
    // let f = File::open("hello.txt");

    // Option 1
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // Option 2
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Option 3

    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
