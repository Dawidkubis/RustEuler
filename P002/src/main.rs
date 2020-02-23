fn gen_fib(lim:usize) -> Vec<usize> {
	let mut cache:Vec<usize> = vec![];

	let mut a = 0;
	let mut b = 0;
	let mut c = 1;

	loop {
		if c > lim {
			break;
		}
		cache.push(c);
		a = b;
		b = c;
		c = a + b;
	}
	cache
}

fn main() {
	let i:usize = gen_fib(4_000_000)
		.into_iter()
		.filter(|x| x % 2 != 0)
		.sum();
	println!("{}", i);
}
