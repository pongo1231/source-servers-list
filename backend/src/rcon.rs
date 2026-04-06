use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub fn rcon(addr: &str, pass: &str, cmd: &str) -> Option<String> {
	source(addr, pass, cmd).or_else(|| goldsrc(addr, pass, cmd))
}

fn source(addr: &str, pass: &str, cmd: &str) -> Option<String> {
	let mut s = TcpStream::connect_timeout(
		&addr.to_socket_addrs().ok()?.next()?,
		Duration::from_millis(500),
	)
	.ok()?;

	_ = s.set_read_timeout(Some(Duration::from_millis(500)));
	_ = s.set_write_timeout(Some(Duration::from_millis(500)));

	s.write_all(&pkt(1, 3, pass)).ok()?;

	loop {
		let (id, ty, _) = read(&mut s)?;
		if ty == 2 {
			// SERVERDATA_AUTH_RESPONSE
			if id == -1 {
				return None;
			}
			break;
		}
	}

	s.write_all(&pkt(2, 2, cmd)).ok()?;
	s.write_all(&pkt(3, 2, "")).ok()?;

	let mut out = String::new();

	loop {
		let (id, _ty, body) = read(&mut s)?;
		if id == 3 {
			break;
		}
		if id == 2 {
			out.push_str(&body);
		}
	}

	Some(out)
}

fn goldsrc(addr: &str, password: &str, cmd: &str) -> Option<String> {
	let s = UdpSocket::bind("0.0.0.0:0").ok()?;
	_ = s.set_read_timeout(Some(Duration::from_millis(500)));
	_ = s.set_write_timeout(Some(Duration::from_millis(500)));

	s.send_to(b"\xFF\xFF\xFF\xFFchallenge rcon", addr).ok()?;

	let mut buf = [0u8; 1400];
	let (n, _) = s.recv_from(&mut buf).ok()?;
	let r = std::str::from_utf8(&buf[4..n]).ok()?;
	let ch = r
		.strip_prefix("challenge rcon ")?
		.trim()
		.strip_suffix("\n\0")
		.unwrap();

	let mut pkt = Vec::new();
	pkt.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
	pkt.extend_from_slice(b"rcon ");
	pkt.extend_from_slice(ch.as_bytes());
	pkt.extend_from_slice(b" ");
	pkt.extend_from_slice(password.as_bytes());
	pkt.extend_from_slice(b" ");
	pkt.extend_from_slice(cmd.as_bytes());
	pkt.push(b'\0');

	s.send_to(&pkt, addr).ok()?;

	let (n, _) = s.recv_from(&mut buf).ok()?;
	Some(String::from_utf8_lossy(&buf[4..n]).to_string())
}

fn pkt(id: i32, ty: i32, body: &str) -> Vec<u8> {
	let mut p = Vec::new();
	p.write_i32::<LittleEndian>(id).unwrap();
	p.write_i32::<LittleEndian>(ty).unwrap();
	p.extend_from_slice(body.as_bytes());
	p.push(0);
	p.push(0);
	let mut out = Vec::new();
	out.write_i32::<LittleEndian>(p.len() as i32).unwrap();
	out.extend(p);
	out
}

fn read(stream: &mut TcpStream) -> Option<(i32, i32, String)> {
	let size = stream.read_i32::<LittleEndian>().ok()?;
	let mut buf = vec![0; size as usize];
	stream.read_exact(&mut buf).ok()?;

	let mut c = &buf[..];
	let id = c.read_i32::<LittleEndian>().ok()?;
	let ty = c.read_i32::<LittleEndian>().ok()?;

	let mut body = Vec::new();
	c.read_to_end(&mut body).ok()?;

	let body_str = String::from_utf8_lossy(&body)
		.trim_matches(char::from(0))
		.to_string();

	Some((id, ty, body_str))
}
