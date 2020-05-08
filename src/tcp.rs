use std::io;

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
    ip: etherparse::Ipv4Header,
}

// Send Sequence Space
//
// 1         2          3          4
// ----------|----------|----------|----------
//   SND.UNA    SND.NXT    SND.UNA
//                        +SND.WND
//
// 1 - old sequence numbers which have been acknowledged
// 2 - sequence numbers of unacknowledged data
// 3 - sequence numbers allowed for new data transmission
// 4 - future sequence numbers which are not yet allowed

#[derive(Debug)]
struct SendSequenceSpace {
    // send unacknowledged
    una: u32,
    // send next
    nxt: u32,
    // send window
    wnd: u16,
    // send urgent pointer
    up: bool,
    // segment sequence number used for last window update
    wl1: usize,
    // segment sequence number used for last window update
    wl2: usize,
    // initial send sequence number
    iss: u32,
}

// Receive Sequence Space
// 1          2          3
// ----------|----------|----------
//    RCV.NXT    RCV.NXT
//              +RCV.WND
// 1 - old sequence numbers which have been acknowledged
// 2 - sequence numbers allowed for new reception
// 3 - future sequence numbers which are not yet allowed

//   Receive Sequence Space
#[derive(Debug)]
struct RecvSequenceSpace {
    // receive next
    nxt: u32,
    // receive window
    wnd: u16,
    // receive urgent pointer
    up: bool,
    // initial receive sequence number
    irs: u32,
}

pub enum State {
    // Listen,
    SynRecv,
    Estab,
}

impl Connection {
    pub fn accept<'a>(
        // &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];
        if !tcph.syn() {
            // only expected SYN packet
            return Ok(None);
        }
        let iss = 0;
        let mut c = Connection {
            state: State::SynRecv,
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: iss + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: RecvSequenceSpace {
                irs: tcph.sequence_number(),
                nxt: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                up: false,
            },
            ip: etherparse::Ipv4Header::new(
                0,
                64,
                etherparse::IpTrafficClass::Tcp,
                [
                    iph.destination()[0],
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                ],
                [
                    iph.source()[0],
                    iph.source()[1],
                    iph.source()[2],
                    iph.source()[3],
                ],
            ),
        };
        // need to start establishing a connection
        let mut syn_ack = etherparse::TcpHeader::new(
            tcph.destination_port(),
            tcph.source_port(),
            c.send.iss,
            c.send.wnd,
        );
        syn_ack.acknowledgment_number = c.recv.nxt;
        syn_ack.syn = true;
        syn_ack.ack = true;
        c.ip.set_payload_len(syn_ack.header_len() as usize + 0);
        // the kernal is nice and does this for us
        // syn_ack
        //     .calc_checksum_ipv4(&ip, &[])
        //     .expect("failed to compute checksum");
        // eprintln!("got ip header:\n{:02x?}", iph);
        // eprintln!("got tcp header:\n{:02x?}", tcph);
        let unwritten = {
            let mut unwritten = &mut buf[..];
            c.ip.write(&mut unwritten);
            syn_ack.write(&mut unwritten);
            unwritten.len()
        };
        eprintln!("responding with {:02x?}", &buf[..buf.len() - unwritten]);
        nic.send(&buf[..unwritten])?;
        Ok(Some(c))
    }

    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<()> {
        // acceptable ack check
        // SND.UNA < SEG.ACK =< SND.NXT
        let ackn = tcph.acknowledgment_number();
        if !is_between_wrapped(self.send.una, self.send.nxt, self.send.nxt.wrapping_add(1)) {
            return Ok(());
        }
        let seqn = tcph.sequence_number();

        if data.len() == 0 && !tcph.syn() && !tcph.fin() {}

        let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
        if !is_between_wrapped(
            self.recv.nxt.wrapping_sub(1),
            seqn,
            self.recv.nxt.wrapping_add(self.recv.wnd as u32),
        ) && !is_between_wrapped(
            self.recv.nxt.wrapping_sub(1),
            seqn + data.len() as u32 - 1,
            wend,
        ) {
            return Ok(());
        }
        match self.state {
            State::SynRecv => {
                // expect to get an ACK for our SYN
            }
            State::Estab => {
                unimplemented!();
            }
        }
        Ok(())
    }
}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
    use std::cmp::Ordering;
    match start.cmp(&x) {
        Ordering::Equal => return false,
        Ordering::Less => {
            // we have:
            //
            //   0 |-------------S------X---------------------| (wraparound)
            //
            // X is between S and E (S < X < E) in these cases:
            //
            //   0 |-------------S------X---E-----------------| (wraparound)
            //
            //   0 |----------E--S------X---------------------| (wraparound)
            //
            // but *not* in these cases
            //
            //   0 |-------------S--E---X---------------------| (wraparound)
            //
            //   0 |-------------|------X---------------------| (wraparound)
            //                   ^-S+E
            //
            //   0 |-------------S------|---------------------| (wraparound)
            //                      X+E-^
            //
            // or, in other words, iff !(S <= E <= X)
            if end >= start && end <= x {
                return false;
            }
        }
        Ordering::Greater => {
            // we have the opposite of above:
            //
            //   0 |-------------X------S---------------------| (wraparound)
            //
            // X is between S and E (S < X < E) *only* in this case:
            //
            //   0 |-------------X--E---S---------------------| (wraparound)
            //
            // but *not* in these cases
            //
            //   0 |-------------X------S---E-----------------| (wraparound)
            //
            //   0 |----------E--X------S---------------------| (wraparound)
            //
            //   0 |-------------|------S---------------------| (wraparound)
            //                   ^-X+E
            //
            //   0 |-------------X------|---------------------| (wraparound)
            //                      S+E-^
            //
            // or, in other words, iff S < E < X
            if end < start && end > x {
            } else {
                return false;
            }
        }
    }
    true
}
