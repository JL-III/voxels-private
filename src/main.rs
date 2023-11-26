use std::io;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

fn run_server() -> io::Result<Child> {
    let server = Command::new("target/debug/server").spawn()?;

    Ok(server)
}

fn run_client() -> io::Result<Child> {
    let client = Command::new("target/debug/client").spawn()?;

    Ok(client)
}

fn main() -> io::Result<()> {
    let mut server = run_server()?;

    let mut client = run_client()?;

    thread::sleep(Duration::from_secs(1));

    match client.wait() {
        Ok(status) => println!("Client exited with status: {}", status),
        Err(e) => eprintln!("Error waiting for client process: {}", e),
    }

    match server.kill() {
        Ok(_) => println!("Server process was killed"),
        Err(e) => eprintln!("Error killing server process: {}", e),
    }

    match server.wait() {
        Ok(status) => println!("Server exited with status: {}", status),
        Err(e) => eprintln!("Error waiting for server process: {}", e),
    }

    Ok(())
}
