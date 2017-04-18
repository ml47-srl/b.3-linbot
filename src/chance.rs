use rand;
use rand::{Rng, thread_rng};
use std::cmp;

pub fn chance<T>(vec : Vec<(i32, &Fn() -> T)>) -> T {
	let mut rng = thread_rng();
	let mut count = 0;
	for thing in &vec {
		count += thing.0;
	}
	let mut n = rng.gen_range(0, count);
	for thing in &vec {
		if n > thing.0 {
			n -= thing.0;
		} else {
			return thing.1();
		}
	}
	panic!("chance: snh")
}

pub fn gen_range<T>(a : T, b : T) -> T where T : rand::distributions::range::SampleRange + cmp::PartialOrd {
	let mut rng = thread_rng();
	rng.gen_range(a, b)
}

pub fn gen_bool() -> bool {
	gen_range(0, 2) == 0
}
