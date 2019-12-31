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
				continue;
			}
		};

		
		if !does_post_exist(&conn, id) {
			println!("Post not found! Is the POST ID correct?");
			continue;
		}

		// Comment contents loop
		println!("Enter the content of the comment below. When you are finished, type <<FIN>> on a new line:");
		text = String::new();
		let mut warn_flag = false;

		loop {
			let input = get_input();
			
			// Checking if text-editing is / should be over.
			if input == "<<FIN>>" {
				if allowed_to_break(input, &text) {
					break;
				} else {
					warn_flag = true;
					continue;
				}
			// Adding text to string and starting over for new line of text.
			} else {
				// Preventing whitespace from appearing at start of string from no preexisting input.
				if text.len() != 0 {
					text = format!("{}\n{}", text.to_string(), input.to_string());
				} else {
					text = format!("{}", input.to_string());
				}
			}
		}

		// Breaking dialogue loop if constraints are satisfied
		// (User says yes, post exists, etc.)
		println!("Is this correct?");
		if warn_flag {
			println!("Note: Any lines containing nothing but <<FIN>>, will be automatically removed.")
		}
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
fn allowed_to_break(input: String, text: &String) -> bool {
	if (input == "<<FIN>>") && !(text == "") {
		return true;
	}
	return false;
}