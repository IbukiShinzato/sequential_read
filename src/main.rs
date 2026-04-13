use memmap::Mmap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

const GB: usize = 1024 * 1024 * 1024;

fn main() -> Result<(), Error> {
    // create file
    let file = File::open("testfile")?;

    println!("1GB: {}", GB);
    println!("0.5GB: {}", GB / 2);

    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    println!("args: {:?}", args);

    if args.len() == 0 {
        eprintln!("Usage: ./target/release/sequential_read <mode>");
        return Ok(());
    }

    match args[0].as_str() {
        "mmap" => mmap(file),
        "read" => {
            if args.len() != 2 {
                eprintln!("Usage ./target/release/sequential_read read <buffer_size>");
                return Ok(());
            }

            let size: usize = FromStr::from_str(args[1].as_str()).unwrap();
            buf_read(file, size)
        }
        _ => {
            eprintln!("mode not found");
            return Ok(());
        }
    }?;

    Ok(())
}

fn mmap(file: File) -> Result<(), Error> {
    // cat file access from memory
    let mmap = unsafe { Mmap::map(&file)? };

    println!("mmap: {:?}", mmap);
    assert_eq!(GB, mmap.len());

    for (i, b) in mmap.iter().enumerate() {
        println!("i: {i}, b: {b}");
    }

    Ok(())
}

fn buf_read(mut file: File, size: usize) -> Result<(), Error> {
    // file read
    let mut buffer = vec![0; size];

    while let Some(byte) = file.read(buffer.as_mut_slice()).ok() {
        if byte == 0 {
            break;
        }
        println!("can read byte: {byte}");
    }

    Ok(())
}
