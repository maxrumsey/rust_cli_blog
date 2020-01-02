use rusqlite::Connection;
use crate::helper::*;
use crate::structs::*;
use rusqlite::NO_PARAMS;

pub fn latest(conn: &Connection) -> bool {
	let mut pid = 1;

	// Preparing DB query to fetch post
	let mut stmt = conn.prepare(
		"SELECT title, text_content, id FROM blog_posts ORDER BY id DESC LIMIT 1"
	).unwrap();

	// Executing and parsing results
	let posts = stmt.query_map(NO_PARAMS, |row|
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
		pid = post.id;
		break;
	}
	// Fetching comments
	console("\nRetrieve Comments? (Y)es/No: ");
	let question = get_input();
	if (question.chars().next().unwrap() == 'Y') || (question.chars().next().unwrap() == 'y') {
		println!("");
		
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