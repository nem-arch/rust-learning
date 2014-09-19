use std::time::Duration;
use std::io::timer;
fn main() {
	let (tx, rx) = channel();
	spawn(proc() {
        println!("sending");
        match tx.send_opt("ehlo") {
            Err(e) => println!("error: {}",e),
            Ok(()) => println!("ok")
        }
    });
    //println!("{}",rx.recv());
    //timer::sleep(Duration::seconds(1));
    println!("closing");
}
