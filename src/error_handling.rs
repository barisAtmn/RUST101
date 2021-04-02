use std::fs::File;
use std::io::{Read, Error};

fn errorH5() -> Result<std::io::Result<usize>, Error> {
    let mut f = File::open("/Users/baris.ataman/Desktop/Projects/hello-rust/resources/foo")?;
    let mut s = String::new();
    let k= f.read_to_string(
        &mut s
    );
    if k.is_ok() {Ok(k)} else {Err((Error::last_os_error()))}
}

// it will return panic when error
fn errorH4() -> Result<File, ()> {
    File::open("/Users/baris.ataman/Desktop/Projects/hello-rust/resources/foot")
        .map_err(|e| -> () {println!("error from errorH4: {}", e.to_string())})
}


fn errorH3() -> File {
    let f =  File::open("ferris.txt");
    let file = match f {
        Ok(filez) => filez,
        Err(_) => {
            let a = File::open("/Users/baris.ataman/Desktop/Projects/hello-rust/resources/foo");
            let d = match a {
                Ok(files) => files,
                Err(er) => panic!("{}",er)
            };
            d
        }
    };
    file
}

fn errorH2() {
    File::open("/Users/baris.ataman/Desktop/Projects/hello-rust/resources/foo").map(
     |mut arg| -> () { let mut buffer = String::new(); arg.read_to_string(&mut buffer); println!("{}", buffer )}
    );
}

// it will return panic when error
fn errorH1() {
    File::open("/Users/baris.ataman/Desktop/Projects/hello-rust/resources/foo").expect("couldnt open file");
}

// it will return panic when error
fn errorH() {
    File::open("/Users/baris.ataman/Desktop/Projects/hello-rust/resources/foo").unwrap();
}


pub fn callErrors() {
    errorH5().unwrap();
    errorH4();
    let mut buffer = String::new();
    errorH3().read_to_string(&mut buffer);
    println!("{}", buffer);
    errorH2();
    errorH1();
    errorH();
}