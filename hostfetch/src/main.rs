mod hostname;

fn main(){
	let mut  my_host = String::new();
	hostname::capture_hostname(&mut my_host).expect("Error while getting hostname");

	println!("{}", my_host);
}
