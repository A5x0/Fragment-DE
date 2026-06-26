match self.state {
    State::Disconnected => self.try_connect(),
    State::Connecting   => self.wait_for_socket(),
    State::Handshake    => self.wait_for_hello_reply(),
    State::SteadyState  => self.process_messages(),
    State::Recovering   => self.reconnect(),
    State::ShuttingDown => self.cleanup(),
}
