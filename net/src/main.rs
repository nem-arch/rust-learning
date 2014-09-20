use std::io;
use std::rand::{task_rng, Rng};
use std::io::timer;
use std::time::Duration;


fn r_n (n: uint) -> Vec<u8> {
    // generate vector of random u8
    task_rng()
        .gen_iter::<u8>()
        .take(n)
        .collect()
}

fn main() {
	let host = "192.168.0.23";
	let port = 1234;
	let mut stream = match io::TcpStream::connect(host, port) {
        Ok(s) => s,
        Err(e) => fail!("{}",e)
    };
    loop {
        let p: Vec<u8> = r_n(2u);
        let c: Vec<String> = r_n(3u)
            .iter()
            .map(|&x| format!("{:X}",x) )
            .collect();
        let s = format!("PX {} {} {}{}{}",p[0],p[1],c[0],c[1],c[2]);
        match stream.write_line(s.as_slice()) {
            Err(e) => fail!("Write Error: {}", e),
            Ok(()) => { }
        }
        timer::sleep(Duration::milliseconds(5));
    }
}

