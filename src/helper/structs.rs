#[derive(Debug)]
pub struct Post {
  pub title: String,
	pub text_content: String,
	pub id: i32
}

#[derive(Debug)]
pub struct Comment {
	pub author: String,
	pub text_content: String,
	pub id: i32
}