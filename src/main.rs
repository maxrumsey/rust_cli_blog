extern crate rusqlite;
mod helper;

use helper::*;
use helper::structs::*;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

fn main() -> Result<()> {

    show_help();

		// Creating and setting up database and tables.
		let conn = Connection::open("blog.db")?;
    conn.execute(
        "create table if not exists blog_posts (
             id integer primary key,
						 title text not null,
						 text_content text not null
         )",
        NO_PARAMS,
    )?;
    conn.execute(
			"create table if not exists blog_comments (
					 id integer primary key,
					 author text not null,
					 text_content text not null,
					 parent_post integer
			 )",
			NO_PARAMS,
		)?;
		
		// Command loop.
		loop {
			console("Enter your command: ");

			let command = get_input();
			
			if (command == "create") || (command == "c") {
				
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

					println!("Is this correct?");
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
				)?;
				
			} else if (command == "get") || (command == "g") {

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
						continue;
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
				)?;

				// Changing into Vector to allow for multiple uses of iterator (len() and iter)
				let postvec: Vec<_> = posts.collect();

				// Checking for number of returned posts
				let lineco = postvec.len();
				if lineco == 0 {
					println!("No entries found.");
					continue;
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
					let posts = stmt.query_map(&[pid], |row|
						Ok(
							Comment {
								author: row.get(0).unwrap(),
								text_content: row.get(1).unwrap(),
								id: row.get(2).unwrap()
							}
						)
					)?;

					// Print loop
					for post in posts {
						let comment = post.unwrap();
						println!("Comment by {}, ID: {}", comment.author, comment.id);
						println!("{}\n", comment.text_content);
					}
				}

			} else if (command == "m") || (command == "comment") {

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

					loop {
						let input = get_input();
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
				)?;

			} else if (command == "h") || (command == "help") {

				show_help();

			} else {

				println!("Command not found.")

			}
		}
}
