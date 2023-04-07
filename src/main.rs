use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

use tun_tap;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();

    let mut nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let n_bytes = nic.recv(&mut buf[..])?;

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[..n_bytes]) {
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                let proto = iph.protocol();

                if proto != 0x06 {
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[iph.slice().len()..n_bytes]) {
                    Ok(tcph) => {
                        let datai = iph.slice().len() + tcph.slice().len();
                        // will cause SYN flood attack when we remember the state of all incoming connections
                        // https://www.cloudflare.com/learning/ddos/syn-flood-ddos-attack/
                        // but we don't care about it here
                        match connections.entry(Quad {
                            src: (src, tcph.source_port()),
                            dst: (dst, tcph.destination_port()),
                        }) {
                            Entry::Occupied(mut c) => {
                                c.get_mut()
                                    .on_packet(&mut nic, iph, tcph, &buf[datai..n_bytes])?;
                            }
                            Entry::Vacant(mut e) => {
                                if let Some(c) = tcp::Connection::accept(
                                    &mut nic,
                                    iph,
                                    tcph,
                                    &buf[datai..n_bytes],
                                )? {
                                    e.insert(c);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("ignoring packet {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring packet {:?}", e);
            }
        }
    }
}
