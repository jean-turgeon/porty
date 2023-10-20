

use clap::{Arg, ArgMatches, Command};


mod scanner;


use scanner::{scan_port, scan_port_range};


pub const LONG_HELP: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";



#[tokio::main]
async fn main() {

    // fetch manifest information
    let authors: &str = env!("CARGO_PKG_AUTHORS");
    let description: &str = env!("CARGO_PKG_DESCRIPTION");
    let name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");

    // Parse command line arguments
    let cli_matches:ArgMatches = Command::new(name)
    .author(authors)
    .version(version)
    .about(description)
    .arg(
        Arg::new("port")
            .help("Port to scan")
            .long("port")
            .short('p'),
    )
    .arg(
        Arg::new("start port")
            .help("Range start port number")
            .long("start")
            .short('s')
            .requires("end port"),
    )
    .arg(
        Arg::new("end port")
            .help("Range end port number")
            .long("end")
            .short('e')
            .requires("start port"),
    )
    .after_help(LONG_HELP)
    .get_matches();


    // Matches arguments
    if cli_matches.contains_id("port") {
        let port_arg:&String = cli_matches.get_one::<String>("port").unwrap();
        let port:u16 = port_arg.to_string().parse::<u16>().unwrap();

        scan_port(port);
    }
    else if  cli_matches.contains_id("start port") && cli_matches.contains_id("end port") {

        let start_arg:&String = cli_matches.get_one::<String>("start port").unwrap();
        let start:u16 = start_arg.to_string().parse::<u16>().unwrap();

        let end_arg:&String = cli_matches.get_one::<String>("end port").unwrap();
        let end:u16 = end_arg.to_string().parse::<u16>().unwrap();

        scan_port_range(start, end);
    }
    else {
        if cli_matches.contains_id("start port") {
            println!("ERROR: end argument is missing!")
        }
        else if cli_matches.contains_id("end port") {
            println!("ERROR: end argument is missing!")
        }
        else  {
            println!("ERROR: no argument!")
        }
    }
}