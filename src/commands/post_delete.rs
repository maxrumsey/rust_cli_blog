use rusqlite::Connection;
use crate::helper::*;
use crate::structs::*;

pub fn delete(conn: &Connection) -> bool {
	console("Enter the ID of the entry you would like to delete: ");
	let id = get_input();

	// Preparing DB check for whether post exists
	let mut stmt = conn.prepare(
		"SELECT title, text_content, id from blog_posts
			WHERE id = ?"
	).unwrap();

	// Getting post ID number and handling non-ints
	let pid = match id.parse::<i32>() {
		Ok(x) => x,
		Err(_e) => {
			println!("Failed to parse POST ID. Are you sure it is a valid integer?");
			return true;
		}
	};

	// TODO: Replace with does_post_exist()
	// Executing and parsing results
	let posts = stmt.query_map(&[pid], |row|
		Ok(
			Post {
				title: row.get(0).unwrap(),
				text_content: row.get(1).unwrap(),
				id: row.get(2).unwrap()
			}
		)
	);

	// Changing into Vector to allow for multiple uses of iterator (len() and iter)
	let postvec: Vec<_> = posts.unwrap().collect();
	
	if postvec.len() == 0 {
		println!("No posts found under this POST ID.");
		return false;
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