use colored::Colorize;

mod hostname;
mod username;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut  my_host = String::new();

	let cfg = match config::load_or_create() {
		Ok(c) => c,
		Err(e) => {
			eprintln!("Error while loading config: {}", e);
			return Ok(());
		}
	};

	let user_host_color = cfg.get_color("host_color");

	let username = match username::get_username() {
		Ok(name) => name,
		Err(e) => {
			eprintln!("Error while getting username: {}", e);
			"unknown".to_string()
		}
	};

	match hostname::get_hostname(&mut my_host) {
		Ok(()) => println!("{}@{}", username.color(user_host_color).bold(), my_host.color(user_host_color).bold()),
		_ => eprintln!("Error"),
	}

	let user_host_len = username.len() + my_host.len() + 1;

	for _i in 1..user_host_len {
		print!("-");
	}
	println!("");

	println!("hostname position: {}", cfg.position.hostname);
	println!("main color: {}", cfg.color.main_color);
	println!("info color: {}", cfg.color.info_color);

	Ok(())
}
