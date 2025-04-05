mod hostname;
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

	match hostname::get_hostname(&mut my_host) {
		Ok(()) => println!("Hostname: {}", my_host),
		_ => eprintln!("Error"),
	}

	println!("hostname position: {}", cfg.position.hostname);
	println!("main color: {}", cfg.color.main_color);
	println!("info color: {}", cfg.color.info_color);

	Ok(())
}
