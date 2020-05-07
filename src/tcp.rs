pub struct State {}

impl Default for State {
    // add code here
    fn default() -> Self {
        State {}
    }
}

impl State {
    pub fn on_packet<'a>(
        &self,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} - {}:{} {}b of tcp",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len(),
        )
    }
}
