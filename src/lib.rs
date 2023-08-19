use std::net::{UdpSocket, ToSocketAddrs};
use std::ops::Deref;

type PassError = Box<dyn std::error::Error>;


pub fn buf_to_string(buf: &[u8]) -> String {
    let mut s = String::new();
    buf.iter().for_each(|&x| s.push(x as char));
    s
}

pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub fn new(port: u16) -> Self {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();
        Client {
            socket
        }
    }

    pub fn send_string(&self, msg: &str) -> Result<(), PassError> {
        self.send(msg.as_bytes())?;
        Ok(())
    }

    pub fn receive_string(&self) -> Result<String, PassError> {
        let mut buf = [0; 1024];
        let amt = self.recv(&mut buf)?;
        let addr = buf_to_string(&buf[..amt]);
        Ok(addr)
    }

    pub fn set_timeout(&self, sec: u64) {
        if sec == 0 {
            self.set_read_timeout(None).expect("cannot set timeout");
        } else {
            self.set_read_timeout(Some(std::time::Duration::from_secs(sec))).expect("cannot set timeout");
        }
    }

    pub fn connect_to_relay<A>(&self, addr: A) -> Result<&Self, PassError> 
        where A: ToSocketAddrs
    {
        self.connect(addr)?;
        Ok(self)
    }

    pub fn connect_to_peer<A>(&self, addr: A) -> Result<&Self, PassError> 
        where A: ToSocketAddrs
    {
        self.set_timeout(2);
        self.connect(addr)?;

        loop {
            self.send_string("connect")?;
            println!("try recv");
            if let Ok(msg) = self.receive_string() {
                println!("recv: {}", msg);
                break;
            }
        }
        self.send_string("connect")?;
        println!("connect to peer success");

        self.set_timeout(0);

        Ok(self)
    }

    pub fn pair_with(&self, code: String) -> Result<&Self, PassError> 
    {
        // send to relay server
        println!("sending code to relay server");
        self.send_string(&code)?;

        // recv from relay server
        println!("receiving command from relay server");
        let mut addr = self.receive_string()?;
        
        if addr == "wait" {
            println!("wait for server");
            // recv from relay server
            addr = self.receive_string()?;
        }

        println!("peer address: {}", addr);

        self.connect_to_peer(addr)?;

        Ok(self)
    }



}

impl Deref for Client {
    type Target = UdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

