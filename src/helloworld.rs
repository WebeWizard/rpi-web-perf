extern crate dotenv;
extern crate tokio;

use std::env;
use std::net::Ipv4Addr;

use tokio::runtime::Runtime;

use webe_web::responders::static_message::StaticResponder;
use webe_web::server::{Route, RouteMap, Server};

fn main() {
    // load the environment config file
    print!("Loading config from environment...... ");
    dotenv::dotenv().expect("Failed to load .env environment config file");
    println!("Done");

    // create the web server
    print!("Setting up Web Server and Route(s)...... ");
    let web_bind_ip = env::var("WEB_BIND_IP").expect("Failed to load web server bind ip from .env");
    let web_bind_port = env::var("WEB_BIND_PORT").expect("Failed to load web server bind port from .env");
    let ip = web_bind_ip.parse::<Ipv4Addr>().expect("Failed to parse WEB_BIND_IP as Ipv4Addr");
    let port = web_bind_port.parse::<u16>().expect("Failed to parse WEB_BIND_PORT as u16");

    
    // Create the runtime
    let rt = Runtime::new().unwrap();
        
    let web_server = rt.block_on(Server::new(&ip, &port)).expect("Failed to create web server");

    // add routes
    let mut route_map = RouteMap::new();
    let hw_route = Route::new("GET", "");
    let hw_responder = StaticResponder::new(200, "Hello World!".to_owned());

    route_map.add_route(hw_route, hw_responder);
    println!("Done");

    // start the web server
    println!("Running web server at {}:{}", web_bind_ip, web_bind_port);
    rt.block_on(web_server.start(route_map));
    
}