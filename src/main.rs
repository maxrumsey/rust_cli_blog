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

    println!("Rust CLI Blog: v0.0.2 - (c) Max Rumsey 2019");
    println!("Commands:");
    println!("get/g = Open an entry.");
		println!("create/c = Create an entry.");

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
					&[&title.to_string(), &content.to_string()]
				)?;
				
			} else if (command == "get") || (command == "g") {
				console("Enter the ID of the entry you would like to get: ");
				let id = get_input();
				let mut stmt = conn.prepare(
					"SELECT title, text_content, id from blog_posts
					 WHERE id = ?"
				).unwrap();
				let posts = stmt.query_map(&[id.parse::<i32>().unwrap()], |row|
					Ok(
						Post {
							title: row.get(0).unwrap(),
							text_content: row.get(1).unwrap(),
							id: row.get(2).unwrap()
						}
					)
				)?;

				// TODO: Fix
				/*				 
				let count = posts.cloned().count();
				if count == 0 {
					println!("No entries found.");
					break Ok(());
				}
				*/
				
				for post_res in posts {
					let post = post_res.unwrap();
					println!("**POST**");
					println!("Title: {}, id: {}", post.title, post.id);
					println!("*CONTENTS*");
					println!("{}", post.text_content);
					break;
				}
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