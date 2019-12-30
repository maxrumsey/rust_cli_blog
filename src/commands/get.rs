use rusqlite::Connection;
use crate::helper::*;
use crate::structs::*;

pub fn get(conn: &Connection) -> bool {
	console("Enter the ID of the entry you would like to get: ");
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

	// Checking for number of returned posts
	let lineco = postvec.len();
	if lineco == 0 {
		println!("No entries found.");
		return true;
	} else {
		println!("{} entries were returned.\n", lineco)
	}
	
	// Printing loop
	for post_res in postvec {
		let post = post_res.unwrap();
		println!("**POST**");
		println!("Title: {}, id: {}", post.title, post.id);
		println!("*CONTENTS*");
		println!("{}", post.text_content);
		break;
	}

	// Fetching comments
	console("\nRetrieve Comments? (Y)es/No: ");
	let question = get_input();
	if (question.chars().next().unwrap() == 'Y') || (question.chars().next().unwrap() == 'y') {
		
		// Fetching comments from DB
		let mut stmt = conn.prepare(
			"SELECT author, text_content, id from blog_comments
				WHERE parent_post = ?"
		).unwrap();

		// Parsing results
		let posts_pre_unwrap = stmt.query_map(&[pid], |row|
			Ok(
				Comment {
					author: row.get(0).unwrap(),
					text_content: row.get(1).unwrap(),
					id: row.get(2).unwrap()
				}
			)
		);

		// Print loop
		let posts = posts_pre_unwrap.unwrap();
		for post in posts {
			let comment = post.unwrap();
			println!("Comment by {}, ID: {}", comment.author, comment.id);
			println!("{}\n", comment.text_content);
		}
	}

	return false;
}