use rusqlite::Connection;
use crate::helper::*;

pub fn comment(conn: &Connection) -> bool {
	let mut name;
	let mut id;
	let mut text;

	// Looping through dialogue until satisfies constraints of comment
	loop {
		console("Enter Name: ");
		name = get_input();
		
		console("Enter POST ID: ");
		let orig_post = get_input();

		// Parsing post ID and handling errors
		id = match orig_post.parse::<i32>() {
			Ok(x) => x,
			Err(_e) => {
				println!("Failed to parse POST ID. Are you sure it is a valid integer?");
				return true;
			}
		};

		
		if !does_post_exist(&conn, id) {
			println!("Post not found! Is the POST ID correct?");
			return true;
		}

		// Comment contents loop
		println!("Enter the content of the comment below. When you are finished, type <<FIN>> on a new line:");
		text = String::new();

		loop {
			let input = get_input();
			
			// TODO: Add check for empty comment.
			// Could replace this with a function
			if input == "<<FIN>>" {
				break;
			} else {
				// Preventing whitespace at start of string from no preexisting input
				if text.len() != 0 {
					text = format!("{}\n{}", text.to_string(), input.to_string());
				} else {
					text = format!("{}", input.to_string());
				}
			}
		}
		// Breaking dialogue loop if constraints are satisfied
		println!("Is this correct?");
		console("(Y)es/No: ");

		let question = get_input();
		if (question.chars().next().unwrap() == 'Y') || (question.chars().next().unwrap() == 'y') {
			break;
		}
	}

	// Inserting into DB
	conn.execute(
		"INSERT INTO blog_comments (author, text_content, parent_post) values (?1, ?2, ?3)",
		&[&name, &text, &(id.to_string())]
	).unwrap();
	return false;
}