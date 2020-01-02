extern crate rusqlite;
pub mod helper;
pub mod commands;

use helper::*;

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
			console("\nEnter your command: ");

			let command = get_input();
			
			if (command == "create") || (command == "c") {
				
				if commands::create::create(&conn) {
					continue;
				}
				
			} else if (command == "get") || (command == "g") {

				if commands::get::get(&conn) {
					continue;
				}

			} else if (command == "m") || (command == "comment") {

				if commands::comment::comment(&conn) {
					continue;
				}

			} else if (command == "h") || (command == "help") {

				commands::help::help();

			} else if (command == "delete") || (command == "d") {
				
				if commands::post_delete::delete(&conn) {
					continue;
				}

			} else if (command == "remove") || (command == "r") {
				if commands::comment_remove::remove(&conn) {
					continue;
				}
			} else if (command == "l") || (command == "latest") {
				commands::latest::latest(&conn);
			} else {

				println!("Command not found.")

			}
		}
}
