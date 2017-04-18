use super::lpath::LocatedCellPath;
use super::super::chance::chance;
use super::super::cond::Condition;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum SpecStep {
	Which(Condition),
	Parent,
	ParentR,
	ParentRE,
	Child(Condition),
	ChildR(Condition),
	ChildRE(Condition)
}

impl SpecStep {
	pub fn translate(&self, vec : Vec<LocatedCellPath>) -> Vec<LocatedCellPath> {
		let mut new_vec = vec![];
		for x in vec {
			for y in self.translate_single(x) {
				new_vec.push(y);
			}
		}
		new_vec
	}

	fn translate_single(&self, p : LocatedCellPath) -> Vec<LocatedCellPath> {
		match &self {
			&&SpecStep::Which(ref cond) => {
				if p.matches(cond) {
					return vec![p];
				} else {
					return vec![];
				}
			},
			&&SpecStep::Parent => {
				match p.get_parent() {
					Ok(x) => vec![x],
					Err(_) => vec![]
				}
			},
			&&SpecStep::ParentR => {
				let mut vec = vec![p.clone()];
				let mut newest = p;
				loop {
					match newest.get_parent() {
						Ok(x) => { vec.push(x.clone()); newest = x; }
						Err(_) => return vec
					}
				}
			},
			&&SpecStep::ParentRE => {
				let mut p_new = p;
				loop {
					match p_new.get_parent() {
						Ok(x) => p_new = x,
						Err(_) => return vec![p_new]
					}
				}
			},
			&&SpecStep::Child(ref cond) => {
				let mut vec = vec![];
				for child in p.get_children() {
					if child.matches(cond) {
						vec.push(child);
					}
				}
				return vec;
			},
			&&SpecStep::ChildR(ref cond) => {
				let mut vec = vec![p];
				let mut done = false;
				while !done {
					done = true;
					for v in vec.clone() {
						for child in v.get_children() {
							if child.matches(&cond) && !vec.contains(&child) {
								done = false;
								vec.push(child);
							}
						}
					}
				}
				return vec;
			},
			&&SpecStep::ChildRE(ref cond) => {
				let mut vec = vec![p];
				let mut done = false;
				while !done {
					done = true;
					for v in vec.clone() {
						for child in v.get_children() {
							if child.matches(&cond) && !vec.contains(&child) {
								done = false;
								vec.push(child);
								match vec.iter().position(|ref a| **a == v) {
									Some(x) => { vec.remove(x); }
									None => {}
								}
							}
						}
					}
				}
				return vec;
			},
		}
	}

	pub fn gen() -> SpecStep {
		chance::<SpecStep>(vec![
			(10, &|| SpecStep::Which(Condition::gen())),
			(3, &|| SpecStep::Parent),
			(1, &|| SpecStep::ParentR),
			(3, &|| SpecStep::ParentRE),
			(3, &|| SpecStep::Child(Condition::gen())),
			(1, &|| SpecStep::ChildR(Condition::gen())),
			(3, &|| SpecStep::ChildRE(Condition::gen()))
		])
	}
}
