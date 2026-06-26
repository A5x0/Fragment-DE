loop {
    let line = read_line_from_socket()?;
    let envelope: IpcEnvelope = serde_json::from_str(&line)?;
    protocol.handle(envelope)?;
}
