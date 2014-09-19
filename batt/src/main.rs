use std::io::File;

fn main() {
    match
        File::create(&Path::new("blub.txt"))
        .write_str("ehlo")
    {
            Err(e) => println!("{}",e),
            _ => { }
    }
}
