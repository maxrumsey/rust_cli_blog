use rusqlite::Connection;
use crate::helper::*;

pub fn delete(conn: &Connection) -> bool {
	console("Enter the ID of the entry you would like to delete: ");
	let id = get_input();

	// Getting post ID number and handling non-ints
	let pid = match id.parse::<i32>() {
		Ok(x) => x,
		Err(_e) => {
			println!("Failed to parse POST ID. Are you sure it is a valid integer?");
			return true;
		}
	};

	if !does_post_exist(conn, pid) {
		println!("We failed to find any posts under that POST ID. Maybe it has already been deleted or you failed to enter it correctly?");
		return true;
	}

	println!("This will delete the entry specified and all comments associated with it. Are you sure you want to do this?");
	console("(Y)es/No: ");

	let question = get_input();
	if !((question.chars().next().unwrap() == 'Y') || (question.chars().next().unwrap() == 'y')) {
		return true;
	}
	
	conn.execute(
		"DELETE FROM blog_posts WHERE id = ?1",
		&[pid]
	).unwrap();
	conn.execute(
		"DELETE FROM blog_comments WHERE parent_post = ?1",
		&[pid]
	).unwrap();

	println!("Successfully deleted the post and comments.");

	return false;
}