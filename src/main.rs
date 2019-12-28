extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::io::{self, Write};

fn main() -> Result<()> {

    println!("Rust CLI Blog: v0.0.0 - (c) Max Rumsey 2019");
    println!("Commands:");
    println!("search/s = Search for an entry.");
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