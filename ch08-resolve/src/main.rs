use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{Arg, Command};
use rand;

//use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::rr::{Name, RecordType};
use trust_dns_client::op::{Message, MessageType, OpCode, Query};
use trust_dns_client::serialize::binary::{BinEncoder, BinEncodable, BinDecoder, BinDecodable};

fn main() {
    let app = Command::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(Arg::new("dns-server").short('s').default_value("1.1.1.1"))
        .arg(Arg::new("domain-name").short('n').required(true))
        .get_matches();

    let domain_name_raw = app.get_one::<String>("domain-name").unwrap();
    let domain_name = Name::from_ascii(domain_name_raw).unwrap();

    let dns_server_raw = app.get_one::<String>("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse()
        .expect("Invalid DNS server address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg
        .set_id(rand::random())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0")
        .expect("Failed to bind to local address");
    let timeout = Duration::from_secs(5);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("Failed to send request, socket misconfigured");

    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("Failed to receive response before timeout");

    let dns_msg = Message::from_vec(&response_as_bytes)
        .expect("Failed to parse response from DNS server");

    for answer in dns_msg.answers() {
        println!("{}", answer);
    }
}
