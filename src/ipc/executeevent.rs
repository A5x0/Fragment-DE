fn handle_event(&mut self, env: IpcEnvelope) -> Result<()> {
    let evt: IpcEvent = serde_json::from_value(env.payload)?;
    match evt {
        IpcEvent::WindowCreated { window_id, title, class, workspace_id, floating } =>
            self.state.add_window(window_id, title, class, workspace_id, floating),

        IpcEvent::WindowFocused { window_id } =>
            self.state.set_focus(window_id),

        IpcEvent::WorkspaceChanged { workspace_id } =>
            self.state.set_active_workspace(workspace_id),

        // etc...
    }
}
