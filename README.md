# Rust CLI Blog

A quick and simple CLI blog platform. Make blog posts, and leave comments.

This is only a proof of concept, please don't use this in production, as it's possible for anyone to change the database's contents through SQL injections or physically accessing the `blog.db` file.

## Installation
To install this program, first clone the repository like so: `git clone https://github.com/maxrumsey/rust_cli_blog`.

Once that's completed, `cd rust_cli_blog` to change your directory to the project's index.

Then run `cargo run` to build the program automatically and run it!

## Usage
* get/g = Open an entry.
* comment/m = Make a comment on an entry.
* create/c = Create an entry.
* delete/d = Delete a post and all comments associated with it.
* remove/r = Remove a comment.
* help/h = Shows the help menu.

## Learning from this Project
If you're new to programming or rust, this project could be helpful in showing you how to interact with SQLite databases from within Rust and how to make CLI style applications. 
Feel free to copy and take whatever from the code, or fork it and modify it to your heart's content. 
If you're looking to build a more robust CLI blog with rust, try with a main process that 'serves' the blog pages and manages authentication and privileges. (To prevent someone from deleting pages without restraint).
Use the main process in conjunction with a client process that communicates with the main process and handles the display of blog posts, as well as personalised settings.