use rusqlite::Connection;
use std::io::{self, Write};
pub mod structs;

use structs::*;

pub fn show_help() {
	println!("\nRust CLI Blog: v0.1.0 - (c) Max Rumsey 2019\n");
  println!("Commands:");
	println!("get/g = Open an entry.");
	println!("comment/m = Make a comment on an entry.");
	println!("create/c = Create an entry.");
	println!("delete/d = Delete a post and all comments associated with it.");
	println!("remove/r = Remove a comment.");
	println!("latest/l = Gets the latest entry from the database.");
	println!("help/h = Shows this screen.");
}

pub fn does_post_exist(conn: &Connection, id: i32) -> bool {
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
		println!("No entries found under this POST ID.");
		return false;
	} else {
		return true;
	}
}
pub fn does_comment_exist(conn: &Connection, id: i32) -> bool {
	let mut stmt = conn.prepare(
		"SELECT author, text_content, id from blog_comments
		 WHERE id = ?"
	).unwrap();
	let comments = stmt.query_map(&[id], |row|
		Ok(
			Comment {
				author: row.get(0).unwrap(),
				text_content: row.get(1).unwrap(),
				id: row.get(2).unwrap()
			}
		)
	);
				 
	let count = comments.unwrap().count();
	if count == 0 {
		println!("No comments found under this COMMENT ID.");
		return false;
	} else {
		return true;
	}
}

pub fn get_input() -> String {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
	input = input.trim().to_string();
	return input;
}

pub fn console(output: &str) {
	print!("{}", output);

  io::stdout().flush().unwrap();
}
