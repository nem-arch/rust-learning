use std::io;
use irc::{Net, User};

mod irc {
    use std::io;
    use std::io::TcpStream;
    use std::io::timer;
    use std::time::Duration;
    use std::str;
    #[deriving(Clone)]
    pub struct Net {
        pub host: String,
        pub port: u16,
    }
    #[deriving(Clone)]
    pub struct User {
        pub nick: String,
    }
    struct Sock {
        pub cmd_rx: Receiver<Result<String, ()>>,
        pub net: TcpStream
    }
    impl Net {
        pub fn output(&mut self, net: &mut TcpStream) {
            let mut buff = [0u8, .. 512];
            loop {
                net.set_read_timeout(Some(10));
                match net.read(buff) {
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
        }
    }
    impl User {
        pub fn connect(&mut self, net: &mut Net) -> Result<Sender<Result<String, ()>>, String> {
            let (tx, rx) = channel::<Result<String,()>>();
            let mut sock = Sock {
                net: match TcpStream::connect(net.host.as_slice(), net.port) {
                    Ok(stream) => stream,
                    Err(e) => return Err(format!("{}",e).to_string()),
                },
                cmd_rx: rx,
            };
            let mut n = sock.net.clone();
            self.apply(&mut sock);
            let mut user_in = self.clone();
            spawn(proc() {
                user_in.input(&mut sock);
            });
            let mut net_out = net.clone();
            spawn(proc() {
                net_out.output(&mut n);
            });
            Ok(tx)
        }
        pub fn apply(&mut self, sock: &mut Sock) {
            sock.net.write_line(format!("NICK {}",
                                        self.nick
                                       ).as_slice()
                               ).unwrap();
            sock.net.write_line(format!("USER {} {} {}: {}",
                                        self.nick,
                                        self.nick,
                                        self.nick,
                                        self.nick
                                       ).as_slice()
                                ).unwrap();
        }
        pub fn input(&mut self, sock: &mut Sock) {
            loop {
                match sock.cmd_rx.recv() {
                    Ok(s) => {
                        println!("COMMAND {}",s);
                        if s.as_slice() == "/quit\n" {
                            sock.net.write_line("QUIT").unwrap();
                            break;
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}

fn main() {
    let mut irc_net = irc::Net {
        host: "irc.freenode.net".to_string(),
        port: 6667,
    };
    let mut irc_user = irc::User {
        nick: "rust_test".to_string(),
    };
    let tx: Sender<Result<String, ()>> = match irc_user.connect(&mut irc_net) {
        Ok(s) => s,
        Err(e) => fail!(e),
    };
    let mut cmd: String = "none".to_string();
    loop {
        match io::stdin().read_line() {
            Ok(s) => {
                cmd = s.clone();
                tx.send(Ok(s));
            },
            _ => {}
        };
        if cmd.as_slice() == "/quit\n" {
            break;
        }
    }
}
