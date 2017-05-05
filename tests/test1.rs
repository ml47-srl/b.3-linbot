extern crate linbot;
extern crate botfather;

use linbot::Linbot;
use botfather::{Botfather, StopReason};
use botfather::libsrl;
use libsrl::db::Database;
use libsrl::cell::Cell;

#[test]
fn test() {
	let mut bot = Linbot::gen();
	let mut db = Database::by_string("= x y. x.").expect("Database creation failed");
	let target = Cell::by_string("y").expect("Cell creation failed");
	bot.call(&mut db, &target);
	bot.assess(StopReason::Timeout, 23);
}
