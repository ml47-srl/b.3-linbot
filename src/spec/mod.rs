mod step;
mod lpath;

use self::step::SpecStep;
use self::lpath::{LocatedCellPath, Location};
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::cell::Cell;
use super::chance::{chance, gen_range};

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
	steps : Vec<SpecStep>,
	starter : Starter
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
enum Starter { ALL, TARGET }

impl Spec {
	pub fn gen() -> Spec {
		let starter = Starter::gen();
		let mut steps = vec![];
		for _ in 0..gen_range(0, 2) {
			steps.push(SpecStep::gen());
		}
		Spec { starter : starter, steps : steps }
	}

	fn get_lpaths(&self, db : &Database, target : &Cell) -> Vec<LocatedCellPath> {
		let mut vec;
		match self.starter {
			Starter::ALL => {
				vec = vec![];
				for i in 0..db.count_rules() {
					vec.push(LocatedCellPath::create_rule(db, CellID::create(i, vec![])));
				}
			},
			Starter::TARGET => {
				vec = vec![LocatedCellPath::create_target(target.clone(), vec![])];
			}
		}

		for step in &self.steps {
			vec = step.translate(vec);
		}
		vec
	}

	pub fn get_cell_ids(&self, db : &Database, target : &Cell) -> Vec<CellID> {
		let mut vec = vec![];
		for c in self.get_lpaths(db, target) {
			let rule_index = match c.get_location() {
				Location::Rule { rule_index : x } => x,
				Location::Target => continue
			};
			vec.push(CellID::create(rule_index, c.get_indices()));
		}
		vec
	}

	pub fn get_cells(&self, db : &Database, target : &Cell) -> Vec<Cell> {
		let mut vec = vec![];
		for c in self.get_lpaths(db, target) {
			vec.push(c.get_cell());
		}
		vec
	}

	pub fn get_indices(&self, db : &Database, target : &Cell) -> Vec<Vec<usize>> { // XXX probabbly not very useful
		let mut vec = vec![];
		for c in self.get_lpaths(db, target) {
			vec.push(c.get_indices());
		}
		vec
	}
}

impl Starter {
	fn gen() -> Starter {
		chance::<Starter>(vec![
			(1, &|| Starter::ALL),
			(1, &|| Starter::TARGET)
		])
	}
}
