use rusqlite::Connection;
use crate::helper::*;

pub fn remove(conn: &Connection) -> bool {
	console("Enter the ID of the comment you would like to remove: ");
	let id = get_input();

	// Getting post ID number and handling non-ints
	let cid = match id.parse::<i32>() {
		Ok(x) => x,
		Err(_e) => {
			println!("Failed to parse COMMENT ID. Are you sure it is a valid integer?");
			return true;
		}
	};

	if !does_comment_exist(conn, cid) {
		println!("We failed to find any comments under that COMMENT ID. Maybe it does not exist or you have failed to enter it correctly?");
		return true;
	}

	println!("This will remove the comment specified. Are you sure you want to do this?");
	console("(Y)es/No: ");

	let question = get_input();
	if !((question.chars().next().unwrap() == 'Y') || (question.chars().next().unwrap() == 'y')) {
		return true;
	}
	
	conn.execute(
		"UPDATE blog_comments SET text_content = \"--REMOVED--\", author = \"--REMOVED--\" WHERE id = ?1",
		&[cid]
	).unwrap();

	println!("Successfully removed the comment.");

	return false;
}