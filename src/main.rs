use std::io;

use tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1500];
    let n_bytes = nic.recv(&mut buf[..]);
    eprintln!("read {:?} bytes", n_bytes);

    Ok(())
}
