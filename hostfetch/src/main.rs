mod hostname;

fn main(){
	let mut  my_host = String::new();

	match hostname::capture_hostname(&mut my_host) {
		Ok(()) => println!("Hostname: {}", my_host),
		_ => eprintln!("Error"),
	}
}
