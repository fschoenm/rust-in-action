#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id: id,
        }
    }

    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

impl Message {
    fn new(to: u64, content: &str) -> Message {
        Message {
            to: to,
            content: content.into(),
        }
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0, 1, 2]
}

fn main() {
    let mut mbox = Mailbox { messages: vec![] };
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();

    let mut sat_a = CubeSat::new(0);
    let mut sat_b = CubeSat::new(1);
    let mut sat_c = CubeSat::new(2);

    // status check
    let status_a = check_status(&sat_a);
    let status_b = check_status(&sat_b);
    let status_c = check_status(&sat_c);
    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);

    // send messages
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = Message::new(sat.id, "hello");
        base.send(&mut mbox, msg);
    }

    // receive message
    let msg = sat_a.recv(&mut mbox);
    println!("msg={:?}", msg);

    // status check
    let status_a = check_status(&sat_a);
    let status_b = check_status(&sat_b);
    let status_c = check_status(&sat_c);
    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);
}
