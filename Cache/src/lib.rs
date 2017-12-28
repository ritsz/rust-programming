use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::SystemTime;
use std::fmt;

macro_rules! vec_of_strings {
	($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[derive(Eq, PartialEq)]
struct State {
	refTime:SystemTime,
	parentPathName:String,
	listOfChild: Vec<String>,
}

impl Ord for State {
	fn cmp(&self, other: &State) -> Ordering {
		println!("Compare {} and {}", self.parentPathName, other.parentPathName);
		other.refTime.cmp(&self.refTime)
			 .then_with(||
			 other.parentPathName.cmp(&self.parentPathName))
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &State) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl fmt::Debug for State {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Reftime : {:?}, ParentPath {}, sizeofChildList {}.\n",
				self.refTime, self.parentPathName, self.listOfChild.len())
	}
}

struct Cache {
	buffer:BinaryHeap<State>, /* moved value by default in rust */
	cacheElements: usize,
	cacheSize: usize,
}

impl fmt::Debug for Cache {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Entries {} Total Size {}\n", self.cacheElements, self.cacheSize);
		for x in self.buffer.iter() {
			write!(f, "Cached: {:?}", x);
		}
		write!(f, "")
	}
}

impl Cache {
	fn new() -> Cache {
		Cache {
			buffer : BinaryHeap::new(),
			cacheElements : 0,
			cacheSize : 0
		}
	}

	fn cache(&mut self, parentPathName: &str, childPathList: Vec<String>) {
		self.cacheSize = self.cacheSize + childPathList.len();
		let cacheEntry = State{refTime:SystemTime::now(), parentPathName:parentPathName.to_string(), listOfChild:childPathList};
		self.buffer.push(cacheEntry);
		self.cacheElements = self.buffer.len();
	}

	fn dump(&self) {
		println!("{:?}", self);

		println!("Top element {:?}", self.buffer.peek());
	}
}



#[test]
fn test_insert() {
	let mut dentryCache = Cache::new();
	println!("Insert-1");
	dentryCache.cache("/home", vec_of_strings!["ritsz", "cvadmin", "cvbkp", "gbuilder"]);
	println!("Insert-2");
	dentryCache.cache("/var", vec_of_strings!["log", "tmp"]);
	println!("Insert-3");
	dentryCache.cache("/etc", vec_of_strings!["sshd", "config", "passwd", "interfaces"]);
	println!("Insert-4");
	dentryCache.cache("/usr", vec_of_strings!["lib", "local", "share"]);
	dentryCache.dump();
}
