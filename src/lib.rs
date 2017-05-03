extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate rand;
extern crate time;
extern crate botfather;

use botfather::{Botfather, StopReason};
use botfather::libsrl;

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
#[derive(Clone)]
pub struct Bot {
	ideas : Vec<WeightedIdea>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

	fn get_least_familiar_idea_index(&self) -> usize {
		let mut smallest = 0;
		for i in 1..self.ideas.len() {
			if self.ideas[smallest].familiarness > self.ideas[i].familiarness {
				smallest = i;
			}
		}
		smallest
	}
}

impl Botfather for Bot {
	fn call(&self, db : &mut Database, target : &Cell) {
		let id = self.get_least_familiar_idea_index();
		self.ideas[id].proof(target, db);
	}

	fn assess(&mut self, stop_reason : StopReason, milliseconds : u32) {
		let id = self.get_least_familiar_idea_index();
		let val = ((milliseconds as f64) / (200 as f64)) as i32 + match stop_reason {
			StopReason::Win => 20,
			StopReason::Fail => -1,
			StopReason::Timeout => -1,
		};
		self.execute_idea_evaluation(id, val);
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

	fn proof(&self, rule : &Cell, db : &mut Database) -> bool {
		self.idea.proof(rule, db)
	}
}
