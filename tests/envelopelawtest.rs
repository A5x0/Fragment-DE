#[test]
fn rejects_unknown_command() {
    let msg = r#"{
        "version": 1,
        "type": "command",
        "payload": { "command": "explode_monitor" }
    }"#;

    let env: IpcEnvelope = serde_json::from_str(msg).unwrap();
    assert!(IpcCommand::deserialize(env.payload).is_err());
}
