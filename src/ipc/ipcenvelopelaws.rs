impl IpcEnvelope {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err("Unsupported protocol version".into());
        }

        match self.r#type.as_str() {
            "command" | "event" | "response" => {}
            _ => return Err("Unknown message type".into()),
        }

        if !self.payload.is_object() {
            return Err("Payload must be an object".into());
        }

        Ok(())
    }
}
