//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-1.3.0
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-21

use std::fmt;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

struct Duck {
	name: String,
	weight: f64,
}
impl Duck {
	fn new(name: String, weight: f64) -> Duck {
		Duck { name: name, weight: weight }
	}
}
impl fmt::Display for Duck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "The duck {} weights {:.2} kg.", self.name, self.weight)
	}
}
// To order a Vec, we need to implement Ord to call sort()
// http://doc.rust-lang.org/std/vec/struct.Vec.html
// http://doc.rust-lang.org/std/cmp/trait.Ord.html
impl PartialEq for Duck {
    #[inline]
    fn eq(&self, other: &Duck) -> bool {
        match self.cmp(other) { Equal => true, _ => false }
    }
}
impl Eq for Duck {}
impl PartialOrd for Duck {
    #[inline]
    fn lt(&self, other: &Duck) -> bool {
        match self.cmp(other) { Less => true, _ => false}
    }
    fn partial_cmp(&self, other: &Duck) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Duck {
	#[inline]
	fn cmp(&self, other: &Duck) -> Ordering {
		if self.weight < other.weight {
			return Less;
		}
		if self.weight > other.weight {
			return Greater;
		}
		if self.name < other.name {
			return Less;
		}
		if self.name > other.name {
			return Greater;
		}
		return Equal;
    }
}
fn main() {
	let mut ducks = vec!(
		Duck::new("Daffy".to_string(), 8f64),
		Duck::new("Dewey".to_string(), 2f64),
		Duck::new("Howard".to_string(), 7f64),
		Duck::new("Louie".to_string(), 2f64),
		Duck::new("Donald".to_string(), 10f64),
		Duck::new("Huey".to_string(), 2f64)
	);

	println!("Before sorting:");
	display(&ducks);

	ducks.sort();

	println!("After sorting:");
	display(&ducks);
}
fn display<'a>(ducks: &'a Vec<Duck>) {
	for duck in ducks.iter() {
		println!("{}", duck);
	}
}
