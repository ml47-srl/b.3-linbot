use libsrl::db::Database;
use libsrl::cell::Cell;
use libsrl::navi::{CellID, CellPath};
use libsrl::error::SRLError;
use super::super::cond::Condition;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Location {
	Target,
	Rule { rule_index : usize }
}

#[derive(PartialEq)]
#[derive(Clone)]
pub struct LocatedCellPath {
	path : CellPath,
	location : Location
}

impl LocatedCellPath {
	pub fn create_target(target : Cell, indices : Vec<usize>) -> LocatedCellPath {
		let path = match CellPath::create(target, indices) {
			Ok(x) => x,
			Err(srl_error) => panic!("LocatedCellPath::create_target(): {}", srl_error)
		};
		LocatedCellPath { path : path, location : Location::Target }
	}

	pub fn create_rule(db : &Database, cell_id : CellID) -> LocatedCellPath {
		let path = match cell_id.get_path(&db.get_rules()) {
			Ok(x) => x,
			Err(srl_error) => panic!("LocatedCellPath::create_rule(): {}", srl_error)
		};
		LocatedCellPath { path : path, location : Location::Rule { rule_index : cell_id.get_rule_id() } }
	}

	pub fn get_cell_path(&self) -> CellPath {
		self.path.clone()
	}

	pub fn get_location(&self) -> Location {
		self.location.clone()
	}

	pub fn matches(&self, cond : &Condition) -> bool {
		cond.matched_by(&self.get_cell_path())
	}

	pub fn get_parent(&self) -> Result<LocatedCellPath, SRLError> {
		let parent_cell_path = self.path.get_parent()?;
		Ok(LocatedCellPath { path : parent_cell_path, location : self.get_location() })
	}

	pub fn get_cell(&self) -> Cell {
		self.get_cell_path().get_cell()
	}

	pub fn get_child(&self, index : usize) -> Result<LocatedCellPath, SRLError> {
		let new_cell_path = match self.path.get_child(index) {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		Ok(LocatedCellPath { path : new_cell_path, location : self.get_location() })
	}

	pub fn get_right_sibling(&self) -> Result<LocatedCellPath, SRLError> {
		let new_cell_path = match self.path.get_right_sibling() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		Ok(LocatedCellPath { path : new_cell_path, location : self.get_location() })
	}

	pub fn get_left_sibling(&self) -> Result<LocatedCellPath, SRLError> {
		let new_cell_path = match self.path.get_left_sibling() {
			Ok(x) => x,
			Err(srl_error) => return Err(srl_error)
		};
		Ok(LocatedCellPath { path : new_cell_path, location : self.get_location() })
	}

	pub fn get_children(&self) -> Vec<LocatedCellPath> {
		let mut current_child = match self.get_child(0) {
			Ok(x) => x,
			Err(_) => return vec![]
		};
		let mut vec = vec![current_child.clone()];
		while let Ok(x) = current_child.get_right_sibling() {
			current_child = x;
			vec.push(current_child.clone());
		}
		vec
	}

	pub fn get_indices(&self) -> Vec<usize> {
		self.path.get_indices()
	}
}
