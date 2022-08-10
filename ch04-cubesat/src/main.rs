#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id: id,
            mailbox: Mailbox { messages: vec![] },
        }
    }

    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg)
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

type Message = String;

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

fn main() {
    let base = GroundStation {};

    let mut sat_a = CubeSat::new(0);
    let mut sat_b = CubeSat::new(1);
    let mut sat_c = CubeSat::new(2);

    // status check
    let status_a = check_status(&sat_a);
    let status_b = check_status(&sat_b);
    let status_c = check_status(&sat_c);
    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);

    base.send(&mut sat_a, Message::from("hello there!"));

    let msg = sat_a.recv();
    println!("msg={:?}", msg);

    // status check
    let status_a = check_status(&sat_a);
    let status_b = check_status(&sat_b);
    let status_c = check_status(&sat_c);
    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);
}
