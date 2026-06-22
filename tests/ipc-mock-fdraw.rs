use std::os::unix::net::UnixListener;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[test]
fn test_fragment_ipc_transport() {
    let socket_path = "/tmp/fdraw.sock";
    let _ = std::fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).expect("Failed to bind FDraw socket");

    // Spawn Fragment in the background
    let mut child = std::process::Command::new("./target/debug/fragment")
        .spawn()
        .expect("Failed to start Fragment");

    // Accept Fragment's connection
    let (stream, _) = listener.accept().expect("Failed to connect to FDraw");

    let mut reader = BufReader::new(stream);

    // Read one JSON message from Fragment
    let mut line = String::new();
    reader.read_line(&mut line).expect("How does this even happen?");

    assert!(line.contains("\"version\":"));
    assert!(line.contains("\"type\":\"command\""));

    // Kill Fragment
    let _ = child.kill();
}
