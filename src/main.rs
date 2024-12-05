use std::collections::HashMap;
use std::io::{self};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tiny_server::handlers::*;
use tiny_server::http::{get, post};
use tiny_server::router::Router;
use tiny_server::server::*;

fn main() -> io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    let mut dir = "".to_string();

    // Check if the correct number of arguments are provided
    if args.len() == 3 {
        // Parse the arguments
        if args[1] == "--directory" {
            dir += &args[2];
            println!("Directory: {}", dir);
        } else {
            eprintln!("Unknown argument: {}", args[1]);
            eprintln!("Usage: {} --directory <path>", args[0]);
        }
    } else {
        eprintln!("Usage: {} --directory <path>", args[0]);
    }

    let mut router: Router = Router::new();

    let mut ctx = HashMap::new();
    dbg!(&dir);
    ctx.insert("dir".to_string(), dir);

    router
        .route(get("/"), handle_success)
        .route(get("/echo/:var/"), handle_echo)
        .route(get("/user-agent/"), handle_user_agent)
        .route(get("/files/:file/"), handle_files)
        .route(post("/files/:file/"), handle_post_files);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4221);
    let app = Server::new(socket);
    app.serve(&router, Some(&ctx))
}
