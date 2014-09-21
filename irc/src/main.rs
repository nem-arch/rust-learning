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
    let nick = "rust_test_";
    let host = "irc.freenode.net";
    let port = 6667;
    let mut buff = [0u8, .. 128];
    let mut sock = match TcpStream::connect(host, port) {
        Ok(s) => s,
        Err(e) => fail!("{}",e)
    };
    sock.write_line(format!("NICK {}",nick).as_slice()).unwrap();
    sock.write_line(format!("USER {} {} {}: {}",nick,nick,nick,nick).as_slice()).unwrap();
    println!("SENT LOGIN");

    loop {
        match sock.read(buff) {
            Ok(i) => {
                for i in range(i, 128) {
                    if i<128 {
                        buff[i] = 0;
                    }
                }
                print!("{}",str::from_utf8(buff).unwrap());
            },
            Err(e) => println!("ERROR READING BUFFER: {}",e)
        };
        match rx.try_recv() {
            Ok(_) => {
                match sock.write_line("QUIT") {
                    Err(e) => println!("ERROR SENDING QUIT: {}",e),
                    Ok(()) => {
                        println!("QUIT SENT");
                        break;
                    }
                };
            },
            _ => ()
        }
    }
    sock.read(buff).unwrap();
    println!("{}",str::from_utf8(buff).unwrap());
}

fn main() {
    let mut sighandler = signal::Listener::new();
    match sighandler.register(Interrupt) {
        Err(e) => { fail!(format!("{}",e)) },
        _ => ()
    };
    match sighandler.register(Break) {
        Err(e) => { fail!(format!("{}",e)) },
        _ => ()
    };
    match sighandler.register(Quit) {
        Err(e) => { fail!(format!("{}",e)) },
        _ => ()
    };
    let (tx, rx) = channel::<Result<(),()>>();
    spawn(proc() {
        irc(rx);
    });
    loop {
        match sighandler.rx.recv() {
            Interrupt|Break|Quit => { tx.send(Err(())); break; },
            _ => ()
        }
    }
    println!("END");
}
