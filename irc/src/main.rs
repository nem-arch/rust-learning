use std::io::TcpStream;
use std::io;
use std::str;

fn main() {
    let nick = "rust_test_";
    let host = "irc.freenode.net";
    let port = 6667;
    let mut sock = match TcpStream::connect(host, port) {
        Ok(s) => s,
        Err(e) => fail!("{}",e)
    };
    sock.write_line(format!("NICK {}",nick).as_slice()).unwrap();
    sock.write_line(format!("USER {} {} {}: {}",nick,nick,nick,nick).as_slice()).unwrap();
    println!("SENT LOGIN");
    let (cmdtx, cmdrx) = channel::<String>();
    let mut sock_write = sock.clone();
    let mut sock_read = sock.clone();
    spawn(proc() {
        loop {
            // hogs cpu
            match sock_read.read_byte() {
                Ok(s) => {
                    print!("{}",str::from_utf8(&[s]).unwrap());
                },
                Err(e) => {
                    println!("ERROR READING BUFFER: {}",e);
                    break;
                }
            };
        }
    });
    spawn(proc() {
        loop {
            match cmdrx.try_recv() {
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
        println!("{}",sock.read_to_string().as_slice());
    });
    let mut cmd: String = "none".to_string();
    loop {
        match io::stdin().read_line() {
            Ok(s) => {
                cmd = s.clone();
                cmdtx.send(s);
            },
            _ => {}
        };
        if cmd.as_slice() == "quit\n" {
            break;
        }
}
println!("END");
}
