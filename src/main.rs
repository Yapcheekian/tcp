use std::io;

use tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let n_bytes = nic.recv(&mut buf[..])?;
        let eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        if eth_proto != 2048 {
            // not ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..n_bytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();

                if proto != 0x06 {
                    // not tcp
                    continue;
                }
                eprintln!(
                    "{:?} -> {:?} {:?}b of protocol {:?}",
                    src,
                    dst,
                    p.payload_len(),
                    proto
                );

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{:?} -> {:?} {:?}b of protocol {:?} to port {:?}",
                            src,
                            dst,
                            p.slice().len(),
                            proto,
                            p.destination_port()
                        );
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

        eprintln!(
            "read {:?} bytes, flags: {:?}, proto: {:?}",
            n_bytes, eth_flags, eth_proto
        );
    }
    Ok(())
}
