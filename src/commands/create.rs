use rusqlite::Connection;
use crate::helper::*;

pub fn create(conn: &Connection) -> bool {
	let mut title;
	let mut content;

	loop {
		console("Enter Title Of Post: ");
		title = get_input();
		println!("Enter the content of the post below. When you are finished, type <<FIN>> on a new line:");
		content = String::new();
		loop {
			let input = get_input();
			if input == "<<FIN>>" {
				break;
			} else {
				if content.len() != 0 {
					content = format!("{}\n{}", content.to_string(), input.to_string());
				} else {
					content = format!("{}", input.to_string());
				}
			}
		}

		println!("\nIs this correct?");
		println!("Title: {}", title);
		println!("Content:");
		println!("{}", content);
		console("(Y)es/No: ");

		let question = get_input();
		if (question.chars().next().unwrap() == 'Y') || (question.chars().next().unwrap() == 'y') {
			break;
		}

	}

	// Entering post into DB.
	conn.execute(
		"INSERT INTO blog_posts (title, text_content) values (?1, ?2)",
		&[&title, &content]
	).unwrap();
	return false;
}