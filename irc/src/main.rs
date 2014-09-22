use std::io::TcpStream;
use std::io;
use std::str;
use std::io::timer;
use std::time::Duration;

fn main() {
    let nick = "rust_test_";
    let host = "irc.freenode.net";
    let port = 6667;
    let mut sock = match TcpStream::connect(host, port) {
        Ok(s) => s,
        Err(e) => fail!("SOCKERROR: {}",e)
    };
    sock.write_line(format!("NICK {}",nick).as_slice()).unwrap();
    sock.write_line(format!("USER {} {} {}: {}",nick,nick,nick,nick).as_slice()).unwrap();
    println!("SENT LOGIN");
    let (cmdtx, cmdrx) = channel::<Result<String, ()>>();
    let mut sock_write = sock.clone();
    let mut sock_read = sock.clone();
    let mut buff = [0u8, .. 512];
    spawn(proc() {
        loop {
            sock_read.set_read_timeout(Some(10));
            match sock_read.read(buff) {
                Ok(i) => {
                    if i<buff.len() {
                        for j in range(i, buff.len()) {
                            buff[j] = 0;
                        }
                    }
                    print!("{}",str::from_utf8(buff).unwrap());
                },
                Err(ref e) => {
                    //println!("ERROR READING BUFFER: {}",e.desc);
                    match e.kind {
                        io::TimedOut => timer::sleep(Duration::seconds(1)),
                        io::EndOfFile => break,
                        _ => ()
                    }
                }
            };
        }
    });
    spawn(proc() {
        loop {
            match cmdrx.recv() {
                Ok(s) => {
                    println!("COMMAND {}",s);
                    if s.as_slice() == "quit\n" {
                        sock_write.write_line("QUIT").unwrap();
                        break;
                    }
                },
                _ => ()
            }
        }
    });
    let mut cmd: String = "none".to_string();
    loop {
        match io::stdin().read_line() {
            Ok(s) => {
                cmd = s.clone();
                cmdtx.send(Ok(s));
            },
            _ => {}
        };
        if cmd.as_slice() == "quit\n" {
            break;
        }
}
println!("END");
}
