use std::collections::HashMap;
mod first;
mod second;


// To prevent any typos.
static EN: &str = "en";
static RU: &str = "ru";
static ES: &str = "es";


fn main() {
	let greeting = first::t(first::Msg::Greeting, EN, None);
	println!("{greeting}"); // Hello!

	let params = HashMap::from([("name", "Alice")]);
	let farewell = first::t(first::Msg::Farewell, EN, Some(&params));
	println!("{farewell}"); // Goodbye, Alice.

	let msg = second::t(second::Msg::ChangesSaved, EN, None);
	println!("{:?}", stringify!(first::Lang::wtf)); // Changes saved.
}
