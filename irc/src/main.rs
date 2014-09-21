extern crate green;
extern crate rustuv;

use std::io::{TcpStream, signal};
use std::io::signal::{Interrupt, Break, Quit};
use std::str;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
        green::start(argc, argv, rustuv::event_loop, main)
}
fn irc(rx: Receiver<Result<(),()>>) {
    let nick = "rust_test";
    let host = "irc.freenode.net";
    let port = 6667;
    let mut buff = [0u8, .. 1024];
    let mut sock = match TcpStream::connect(host, port) {
        Ok(s) => s,
        Err(e) => fail!("{}",e)
    };
    sock.write_line(format!("NICK {}",nick).as_slice()).unwrap();
    sock.write_line(format!("USER {} {} {}: {}",nick,nick,nick,nick).as_slice()).unwrap();
    loop {
        match sock.read(buff) {
            Ok(uint) => { println!("{}",str::from_utf8(buff)) },
            Err(e) => println!("ERROR: {}",e)
        };
        match rx.try_recv() {
            Err(Empty) => { },
            Ok(_) => { sock.write_line("QUIT"); break; }
        }
    }
    println!("QUIT");
}

fn main() {
    let mut sighandler = signal::Listener::new();
    match sighandler.register(Interrupt) {
        Ok(()) => { },
        Err(e) => { fail!(format!("{}",e)) }
    };
    match sighandler.register(Break) {
        Ok(()) => { },
        Err(e) => { fail!(format!("{}",e)) }
    };
    match sighandler.register(Quit) {
        Ok(()) => { },
        Err(e) => { fail!(format!("{}",e)) }
    };
    let (tx, rx) = channel::<Result<(),()>>();
    spawn(proc() {
        irc(rx);
    });
    loop {
        match sighandler.rx.recv() {
            Interrupt|Break|Quit => tx.send(Ok(())),
            _ => ()
        }
    }
}
