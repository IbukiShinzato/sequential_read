use memmap::Mmap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;

const PAGE: usize = 4 * 1024;
const GB: usize = 1024 * 1024 * 1024;

fn main() -> Result<(), Error> {
    // create file
    let file = File::open("testfile")?;

    let args: Vec<String> = env::args().into_iter().skip(1).collect();

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

// file read from memory
fn mmap(file: File) -> Result<(), Error> {
    let mmap = unsafe { Mmap::map(&file)? };
    let mut total = 0;

    assert_eq!(GB, mmap.len());

    for i in (0..mmap.len()).step_by(PAGE) {
        let _ = mmap[i];
        total += PAGE;
    }

    assert_eq!(total, GB, "mmap read size mismatch");

    Ok(())
}

// file read from disk
fn buf_read(mut file: File, size: usize) -> Result<(), Error> {
    let mut buffer = vec![0; size];
    let mut total = 0;

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        };

        for i in (0..n).step_by(PAGE) {
            let _ = buffer[i];
        }
        total += n;
    }

    assert_eq!(total, GB, "buf_read size mismatch");

    Ok(())
}
