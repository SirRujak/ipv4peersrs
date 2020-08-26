pub mod ipv4peers {
    const DEFAULT_ENTRY_SIZE: usize = 6;

    pub enum PeerError {
        HostParseError,
        PortRangeError,
    }

    pub type EncodeResult = Result<Vec<u8>, PeerError>;
    pub type DecodeResult = Result<Peers, PeerError>;

    #[derive(Clone)]
    pub struct Peer {
        host: String,
        port: u16,
        id: Vec<u8>,
    }

    #[derive(Clone)]
    pub struct Peers {
        peers: Vec<Peer>,
    }

    pub struct IPV4Peers {
        id_length: Option<usize>,
        entry_size: usize,
    }

    impl IPV4Peers {
        pub fn new(id_length: Option<usize>) -> IPV4Peers {
            let entry_size: usize;
            match id_length {
                Some(i) => entry_size = i + DEFAULT_ENTRY_SIZE,
                None => entry_size = DEFAULT_ENTRY_SIZE,
            }
            IPV4Peers {
                id_length,
                entry_size,
            }
        }

        fn encoding_length(&self, peers: &Peers) -> usize {
            peers.peers.len() * self.entry_size
        }

        pub fn encode(&self, peers: &Peers, buf: Option<Vec<u8>>, offset: Option<usize>) -> EncodeResult {
            let mut local_buf: Vec<u8>;
            match buf {
                Some(n) => local_buf = n,
                None => {
                    //local_buf = Vec::with_capacity(self.encoding_length(peers));
                    local_buf = vec![0u8; self.encoding_length(&peers)]
                }
            }

            let mut local_offset: usize;
            match offset {
                Some(o) => local_offset = o,
                None => local_offset = 0,
            }

            let mut peer_host: Vec<&str>;
            for p in 0..peers.peers.len() {
                match self.id_length {
                    Some(i) => {
                            local_buf[local_offset..local_offset +i].clone_from_slice(&peers.peers[p].id);
                            local_offset += i;
                        },
                    None => {},
                };

                peer_host = peers.peers[p].host.split('.').collect();
                for j in 0..4 {
                    match peer_host[j].parse::<u8>() {
                        Ok(i) => {
                            local_buf[local_offset] = i;
                            local_offset += 1;
                        }

                        Err(_e) => return Err(PeerError::HostParseError)
                    }
                }

                let local_peer: [u8; 2] = peers.peers[p].port.to_be_bytes();
                for j in 0..2 {
                    local_buf[local_offset] = local_peer[j];
                    local_offset += 1;
                }
            }

            Ok(local_buf)
        }

        pub fn decode(&self, buf: Vec<u8>, offset: Option<usize>, end: Option<usize>) -> DecodeResult {
            let mut local_offset: usize;
            match offset {
                Some(o) => local_offset = o,
                None => local_offset = 0,
            }

            let local_end: usize;
            match end {
                Some(e) => local_end = e,
                None => local_end = buf.len(),
            }

            let mut peers = Peers{
                peers: vec![Peer{
                    host: "".to_string(),
                    port: 0,
                    id: Vec::<u8>::new(),
                }; (local_end - local_offset)/ self.entry_size],
            };

            let mut id: Vec<u8>;
            let mut host: String;
            let mut port: u16;
            let mut port_array: [u8; 2];
            for p in 0..peers.peers.len() {
                match self.id_length {
                    Some(i) => {
                        id = buf[local_offset..local_offset+i].to_vec();
                        local_offset += i;
                    },
                    None => id = Vec::new(),
                }

                host = [buf[local_offset].to_string(),
                        '.'.to_string(),
                        buf[local_offset + 1].to_string(),
                        '.'.to_string(),
                        buf[local_offset + 2].to_string(),
                        '.'.to_string(),
                        buf[local_offset + 3].to_string()
                    ].concat();
                local_offset += 4;

                port_array = [0; 2];
                port_array[0] = buf[local_offset];
                port_array[1] = buf[local_offset + 1];
                port = u16::from_be_bytes(port_array);
                if port == 0 {
                    return Err(PeerError::PortRangeError);
                }

                peers.peers[p] = Peer{
                    host,
                    port,
                    id,
                }
            }

            Ok(peers)
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
