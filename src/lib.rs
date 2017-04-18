pub extern crate serde;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde_json;
pub extern crate rand;
pub extern crate time;
pub extern crate libsrl;

mod chance;
mod idea;
mod action;
mod pattern;
mod spec;
mod cond;

use self::idea::Idea;
use libsrl::cell::Cell;
use libsrl::db::Database;

const MIN_IDEAS : usize = 7;

// linear bot
pub struct Bot {
	ideas : Vec<WeightedIdea>
}

#[derive(Serialize, Deserialize, Debug)]
struct WeightedIdea {
	idea : Idea,
	niceness : i32,
	familiarness : u32 // number of usages
}

impl Bot {
	pub fn gen() -> Box<Bot> {
		let mut ideas = vec![];
		for _ in 0..MIN_IDEAS {
			ideas.push(WeightedIdea::gen())
		}
		Box::new(Bot { ideas : ideas })
	}

	pub fn by_string(string : String) -> Box<Bot> {
		let mut ideas = vec![];
		for split in string.split('\n') {
			if split.is_empty() { continue; }
			ideas.push(serde_json::from_str(&split).expect("by_string failed"));
		}
		Box::new(Bot { ideas : ideas })
	}


	pub fn to_string(&self) -> String {
		let mut string_vec = vec![];
		for idea in &self.ideas {
			string_vec.push(serde_json::to_string(&idea).expect("serde_json::to_string failed on idea"));
		}
		string_vec.join("\n")
	}

	pub fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		for i in 0..self.ideas.len() {
			if self.ideas[i].proof(rule, db) {
				return true;
			}
		}
		false
	}

	pub fn practice(&mut self, rule : &Cell, db : &mut Database) -> bool {
		let mut worked = false;
		for i in 0..self.ideas.len() {
			let (time, b) = self.ideas[i].proof_timed(rule, db);
			let mut evaluation = 0;
			evaluation -= ((time as f64) / (200 as f64)) as i32;
			if b {
				evaluation += 20;
				worked = true;
			} else {
				evaluation -= 1;
			}
			self.execute_idea_evaluation(i, evaluation);
		}
		worked
	}

	fn execute_idea_evaluation(&mut self, i : usize, evaluation : i32) {
		self.ideas[i].eval(evaluation);
		let weighted_niceness = self.ideas[i].get_weighted_niceness();

		if weighted_niceness > 100 {
			let mutation = self.ideas[i].mutate();
			self.ideas.push(mutation); // XXX would cause too many mutations sometimes
		} else if weighted_niceness < -100 {
			self.remove_idea(i);
		}
	}

	fn remove_idea(&mut self, i : usize) {
		self.ideas.remove(i);
		if self.ideas.len() < MIN_IDEAS {
			self.find_new_idea();
		}
	}

	fn find_new_idea(&mut self) {
		self.ideas.push(WeightedIdea::gen()); // XXX maybe use mutation of best ideas here
	}
}


impl WeightedIdea {
	fn gen() -> WeightedIdea {
		WeightedIdea { idea : Idea::gen(), niceness : 0, familiarness : 0 }
	}

	fn get_weighted_niceness(&self) -> i32 {
		self.niceness * self.familiarness as i32 // XXX 10 fails & 1 win (-9 * 11) should be better than 9 fails (-9 * 9)
	}

	fn eval(&mut self, evaluation : i32) {
		self.familiarness += 1;
		self.niceness += evaluation;
	}

	fn mutate(&self) -> WeightedIdea {
		let keep = self.get_weighted_niceness();
		WeightedIdea { idea : self.idea.mutate(keep), niceness : 0, familiarness : 0 }
	}

	fn proof_timed(&self, rule : &Cell, db : &mut Database) -> (i64, bool) {
		self.idea.proof_timed(rule, db)
	}

	fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		self.idea.proof(rule, db)
	}
}
