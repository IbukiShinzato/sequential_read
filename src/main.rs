use memmap::Mmap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("testfile")?;
    let mmap = unsafe { Mmap::map(&file)? };

    println!("mmap: {:?}", mmap);
    assert_eq!(1024 * 1024 * 1024, mmap.len());

    for (i, b) in mmap.iter().enumerate() {
        println!("i: {i}, b: {b}");
    }

    Ok(())
}
