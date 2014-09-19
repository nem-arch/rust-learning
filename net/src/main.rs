use std::io;
use std::rand::{task_rng, Rng};
use std::num;
use std::io::timer;
use std::time::Duration;

fn gen_dots(x: int, y: int) -> (int, int) {
    let mut rng = task_rng();
    return ( num::div_rem(rng.gen(),255i).val1(), num::div_rem(rng.gen(),255i).val1() )
}

fn handle_write(mut stream: io::TcpStream) -> io::IoResult<()> {
    let (x,y) = gen_dots(1,1);
    let (u,v) = gen_dots(x,y);
    let s = format!("PX {} {} {}{}00",x,y,format!("{:X}",u),format!("{:X}",v));
	stream.write_line(s.as_slice())
}

fn main() {
	let host = "192.168.0.23";
	let port = 1234;
	let stream = match io::TcpStream::connect(host, port) {
        Ok(s) => s,
        Err(e) => fail!("{}",e)
    };
	loop {
		match handle_write(stream.clone()) {
			Err(e) => println!("Write Error: {}", e),
			//Ok(()) => handle_readback(stream.clone())
            _ => { timer::sleep(Duration::milliseconds(5))}
		}
	}
}

