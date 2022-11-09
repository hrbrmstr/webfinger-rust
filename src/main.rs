use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	// resource URI
	resource: String,
	
  /// Host to issue the resource query on vs deriving hostname from the resource query itself.
	#[arg(short, long)]
	config: Option<String>,

	/// Pretty-print JSON response on successful resource queries?
	#[arg(short, long)]
  pretty: bool,

}

fn resolve_host(resource: &str) -> String {
	resource.split('@').last().expect("invalid resource URI").to_string()
}

fn main() {
	let cli = Cli::parse();

	let host = cli.config.unwrap_or_else(||resolve_host(&cli.resource));

	let url = format!("https://{}/.well-known/webfinger?resource={}", host, cli.resource);

	let res = reqwest::blocking::get(url).expect("Error retrieving resource.");

	let body = res.text().expect("Error extracting response body.");

	if cli.pretty {
		let pretty = jsonxf::pretty_print(&body).expect("Error formatting JSON");
		println!("{}", pretty);
	} else {
		println!("{}", body);
	}

}
