fn handle_command(&mut self, env: IpcEnvelope) -> Result<()> {
    let cmd: IpcCommand = serde_json::from_value(env.payload)?;
    match cmd {
        IpcCommand::FocusWindow { window_id } => self.wm.focus(window_id),
        IpcCommand::MoveWindow { window_id, x, y } => self.wm.move_window(window_id, x, y),
        IpcCommand::ResizeWindow { window_id, width, height } =>
            self.wm.resize_window(window_id, width, height),
        // etc...
    }
}
