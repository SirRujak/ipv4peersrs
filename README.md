# ipv4peersrs

A direct rust translation of ipv4-peers for use in the hyper protocol based on the original nodejs library found here:

https://github.com/mafintosh/ipv4-peers

# Usage

```

let ipv4_peers = IPV4Peers::new(None);

let peers = Peers {
  peers: vec![
    Peer {
      host: "127.0.0.1",
      port: 8080,
      id: Vec::new(),
    },
    Peer {
      host: "127.0.0.1",
      port: 9090,
      id: Vec::new(),
    }
  ]
}

let result: EncodeResult = ipv4_peers.encode(peers);
match result {
  Ok(r) => println!("{:?}", r),
  Err(e) => println!("{:?}", e),
}

```
