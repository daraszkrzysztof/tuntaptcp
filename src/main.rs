use std::io;

use tun_tap::{Iface, Mode};

fn main() {

    let iface = Iface::new("mytun", Mode::Tun)
        .expect("Failed to create a TUN device");

    let name = iface.name();

    println!("name {}", name);

    loop {
        let mut buf = vec![0; 1504]; // MTU + 4 for the header
        let nbytes = iface.recv(&mut buf).unwrap();

        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]){
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                //let plen = p.payload_len();
                let proto = p.protocol();
                if proto!= 0x06 {
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice( &buf[4+p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!("{} - {} {}b of tcp to port {}", src, dst, p.slice().len(), p.destination_port());
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet {:?}", e)
                    }
                }

                //eprintln!("{} - {} {}b of protocol {}", src, dst, plen, proto)
                // x? ⇒ Debug with lower-case hexadecimal integers
                // X? ⇒ Debug with upper-case hexadecimal integers
                // eprintln!("read {} bytes: (flags: {:X}, proto: {:X}) {:?}",
                //           nbytes-4, flags, proto, p);
            }
            Err(e) => {
                eprintln!("ignoring weird packet")
            }
        }
    }
}
