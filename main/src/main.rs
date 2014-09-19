enum Ordering {
	Less,
	Equal,
	Greater,
}

fn cmp(a: int, b: int) -> Ordering {
	if a<b { Less }
	else if a>b { Greater }
	else { Equal }
}

fn main() {
	let i: int = 2;
	let (s, v): (&str, int) = ("x", if i >= 5i { i*i } else { i } );
	let x: (&str, int) = ("x", if i >= 5i { i*i } else { i } );
	println!("{}: {}",s,match cmp(v,i) {
		Less => "less",
		Greater => "greater",
		Equal => "equal",
	})
	for e in x {
		println!("{}",e)
	}
}
