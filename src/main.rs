extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::io::{self, Write};

#[derive(Debug)]
struct Post {
    title: String,
		text_content: String,
		id: i32
}

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
							content = format!("{}\n{}", content.to_string(), input.to_string())
						}
					}

					println!("Is this correct?");
					println!("Title: {}", title);
					println!("Content:");
					println!("{}", content);
					console("(Y)es/No: ");

					if (get_input().chars().next().unwrap() == 'Y') || (get_input().chars().next().unwrap() == 'y') {
						break;
					}
				}
				conn.execute(
					"INSERT INTO blog_posts (title, text_content) values (?1, ?2)",
					&[&title, &content]
				)?;
				
			} else if (command == "get") || (command == "g") {
				console("Enter the ID of the entry you would like to get: ");
				let id = get_input();
				let mut stmt = conn.prepare(
					"SELECT title, text_content, id from blog_posts
					 WHERE id = ?"
				).unwrap();
				// TODO: Prevent from panicing from non-int.
				let posts = stmt.query_map(&[id.parse::<i32>().unwrap()], |row|
					Ok(
						Post {
							title: row.get(0).unwrap(),
							text_content: row.get(1).unwrap(),
							id: row.get(2).unwrap()
						}
					)
				)?;

				let postvec: Vec<_> = posts.collect();

				let lineco = postvec.len();
				if lineco == 0 {
					println!("No entries found.");
					continue;
				} else {
					println!("{} entries were returned.\n", lineco)
				}
				
				
				for post_res in postvec {
					let post = post_res.unwrap();
					println!("**POST**");
					println!("Title: {}, id: {}", post.title, post.id);
					println!("*CONTENTS*");
					println!("{}", post.text_content);
					break;
				}
			} else if (command == "m") || (command == "comment") {
				let mut name;
				let mut id;
				let mut text;
				
				loop {
					console("Enter Name: ");
					name = get_input();
					
					console("Enter POST ID: ");
					let orig_post = get_input();

					// TODO: Prevent from panicing from non-int.
					id = orig_post.parse::<i32>().unwrap();
					if !does_post_exist(&conn, id) {
						println!("Post not found! Is the POST ID correct?");
						continue;
					}
					println!("Enter the content of the comment below. When you are finished, type <<FIN>> on a new line:");
					text = String::new();
					loop {
						let input = get_input();
						if input == "<<FIN>>" {
							break;
						} else {
							text = format!("{}\n{}", text.to_string(), input.to_string())
						}
					}

					println!("Is this correct?");
					console("(Y)es/No: ");

					if (get_input().chars().next().unwrap() == 'Y') || (get_input().chars().next().unwrap() == 'y') {
						break;
					}
				}

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

fn get_input() -> String {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
	input = input.trim().to_string();
	return input;
}
fn console(output: &str) {
	print!("{}", output);

  io::stdout().flush().unwrap();
}
fn does_post_exist(conn: &Connection, id: i32) -> bool {
	let mut stmt = conn.prepare(
		"SELECT title, text_content, id from blog_posts
		 WHERE id = ?"
	).unwrap();
	let posts = stmt.query_map(&[id], |row|
		Ok(
			Post {
				title: row.get(0).unwrap(),
				text_content: row.get(1).unwrap(),
				id: row.get(2).unwrap()
			}
		)
	);
				 
	let count = posts.unwrap().count();
	if count == 0 {
		println!("No entries found.");
		return false;
	} else {
		return true;
	}
}

fn show_help() {
	println!("\nRust CLI Blog: v0.0.4 - (c) Max Rumsey 2019");
  println!("Commands:");
	println!("get/g = Open an entry.");
	println!("comment/m = Make a comment on an entry.");
	println!("create/c = Create an entry.");
	println!("help/h = Shows this screen.");
}