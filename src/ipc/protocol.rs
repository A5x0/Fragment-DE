impl Protocol {
    pub fn handle(&mut self, env: IpcEnvelope) -> Result<()> {
        env.validate()?; // version, type, payload

        match env.r#type.as_str() {
            "command" => self.handle_command(env),
            "event"   => self.handle_event(env),
            "response"=> self.handle_response(env),
            _ => Err("Unknown message type".into())
        }
    }
}
