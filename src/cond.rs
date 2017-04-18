use super::pattern::Pattern;
use libsrl::navi::CellPath;
use super::chance::chance;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Condition {
	Pattern(Pattern),
	Bool,
	CompleteBool,
	Wrapper { positive : Trool, nallq : Trool, nexq : Trool },
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Trool { True, Ignore, False }

impl Condition {
	pub fn gen() -> Condition {
		chance::<Condition>(vec![
			(4, &|| Condition::Pattern(Pattern::gen())),
			(1, &|| Condition::Bool),
			(1, &|| Condition::CompleteBool),
			(2, &|| Condition::Wrapper { positive : Trool::gen(), nallq : Trool::gen(), nexq : Trool::gen() }),
		])
	}

	pub fn matched_by(&self, c : &CellPath) -> bool {
		match &self {
			&&Condition::Pattern(ref pat) => pat.matched_by(&c.get_cell()),
			&&Condition::Bool => c.is_bool(),
			&&Condition::CompleteBool => c.is_complete_bool(),
			&&Condition::Wrapper { positive : ref positive_out, nallq : ref nallq_out, nexq : ref nexq_out } => {
				if let Some(w) = c.get_wrapper() {
					if !(positive_out.accepts(w.is_positive())) {
						return false;
					} else if !(nallq_out.accepts(w.is_nallq())) {
						return false;
					} else if !(nexq_out.accepts(w.is_nexq())) {
						return false;
					} else {
						return true;
					}
				} else {
					return false;
				}
			}
		}
	}
}

impl Trool {
	fn accepts(&self, b : bool) -> bool {
		match &self {
			&&Trool::True => b,
			&&Trool::Ignore => true,
			&&Trool::False => !b
		}
	}

	fn gen() -> Trool {
		chance::<Trool>(vec![
			(1, &|| Trool::True),
			(1, &|| Trool::Ignore),
			(1, &|| Trool::False),
		])
	}
}
