use super::spec::Spec;
use libsrl::db::Database;
use libsrl::cell::Cell;
use libsrl::gen::simple;
use super::chance::chance;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
	EqualsLaw(Spec, Spec),
	EqualsLawImpl(Spec, Spec),
	InequalConstants(Spec),
	AddEqt(Spec),
	RmEqt(Spec),
	ScopeInsertion(Spec, Spec),
	ScopeCreation(Spec, Spec),
	ImplicationsDerivation(Spec, Spec),
	ScopeExchange(Spec),
	CaseCreation(Spec, Spec),
	Declaration(Spec)
}

impl Action {
	pub fn gen() -> Action {
		chance::<Action>(vec![
			(4, &|| Action::EqualsLaw(Spec::gen(), Spec::gen())),
			(1, &|| Action::EqualsLawImpl(Spec::gen(), Spec::gen())),
			(3, &|| Action::InequalConstants(Spec::gen())),
			(3, &|| Action::AddEqt(Spec::gen())),
			(3, &|| Action::RmEqt(Spec::gen())),
			(2, &|| Action::ScopeInsertion(Spec::gen(), Spec::gen())),
			(1, &|| Action::ScopeCreation(Spec::gen(), Spec::gen())),
			(1, &|| Action::ImplicationsDerivation(Spec::gen(), Spec::gen())),
			(1, &|| Action::ScopeExchange(Spec::gen())),
			(1, &|| Action::CaseCreation(Spec::gen(), Spec::gen())),
			(1, &|| Action::Declaration(Spec::gen()))
		])
	}

	pub fn call(&self, target : &Cell, db : &mut Database) -> u32 { // returns the amount of created rules
		let mut counter = 0;
		match &self {
			&&Action::EqualsLaw(ref spec1, ref spec2) => {
				let ids1 = spec1.get_cell_ids(db, target);
				let ids2 = spec2.get_cell_ids(db, target);
				for id1 in &ids1 {
					for id2 in &ids2 {
						if db.equals_law(id1.clone(), id2.clone()).is_ok() {
							counter += 1;
						}
					}
				}
			},
			&&Action::EqualsLawImpl(ref spec1, ref spec2) => {
				let ids1 = spec1.get_cell_ids(db, target);
				let ids2 = spec2.get_cell_ids(db, target);
				for id1 in &ids1 {
					for id2 in &ids2 {
						if db.equals_law_impl(id1.clone(), id2.clone()).is_ok() {
							counter += 1;
						}
					}
				}
			},
			&&Action::InequalConstants(ref spec) => {
				let ids = spec.get_cell_ids(db, target);
				for id in &ids {
					if db.inequal_constants(id.clone()).is_ok() {
						counter += 1;
					}
				}
			},
			&&Action::AddEqt(ref spec) => {
				let ids = spec.get_cell_ids(db, target);
				for id in &ids {
					if db.add_eqt(id.clone()).is_ok() {
						counter += 1;
					}
				}
			},
			&&Action::RmEqt(ref spec) => {
				let ids = spec.get_cell_ids(db, target);
				for id in &ids {
					if db.rm_eqt(id.clone()).is_ok() {
						counter += 1;
					}
				}
			},
			&&Action::ScopeInsertion(ref spec1, ref spec2) => {
				let ids = spec1.get_cell_ids(db, target);
				let cells = spec2.get_cells(db, target);
				for id in &ids {
					for cell in &cells {
						if db.scope_insertion(id.clone(), cell.clone()).is_ok() {
							counter += 1;
						}
					}
				}
			},
			&&Action::ScopeCreation(ref spec1, ref spec2) => {
				let ids = spec1.get_cell_ids(db, target);
				let indices = spec2.get_indices(db, target);
				for id in &ids {
					if db.scope_creation(id.clone(), indices.clone()).is_ok() { // XXX indices is not really gonna be useful here
						counter += 1;
					}
				}
			},
			&&Action::ImplicationsDerivation(ref spec1, ref spec2) => {
				let ids1 = spec1.get_cell_ids(db, target);
				let ids2 = spec2.get_cell_ids(db, target);
				for id1 in &ids1 {
					for id2 in &ids2 {
						if db.implications_derivation(id1.clone(), id2.clone()).is_ok() {
							counter += 1;
						}
					}
				}
			},
			&&Action::ScopeExchange(ref spec) => {
				let ids = spec.get_cell_ids(db, target);
				for id in &ids {
					if db.scope_exchange(id.clone()).is_ok() {
						counter += 1;
					}
				}
			},
			&&Action::CaseCreation(ref spec1, ref spec2) => {
				let ids = spec1.get_cell_ids(db, target);
				let cells = spec2.get_cells(db, target);
				for id in &ids {
					for cell in &cells {
						if db.case_creation(id.clone(), cell.clone()).is_ok() {
							counter += 1;
						}
					}
				}
			},
			&&Action::Declaration(ref spec) => {
				let ids = spec.get_cell_ids(db, target);
				for id in &ids {
					let simple_cell = simple(get_free_declaration_name(db));
					if db.case_creation(id.clone(), simple_cell).is_ok() {
						counter += 1;
					}
				}
			}
		}
		return counter;
	}

	pub fn mutate(&self, keep : i32) -> Action {
		Action::gen() // XXX
	}
}

fn get_free_declaration_name(db : &Database) -> String {
	let mut string : String = "_decl_a".to_string();
	while db.contains_cellname(&string) {
		match string.pop() {
			Some('z') => { string.push_str("aa"); },
			Some(x) => { string.push(((x as u8) + 1) as char); },
			None => panic!("get_free_declaration_name(): string is empty")
		}
	}
	string
}
