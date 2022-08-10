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
}

struct GroundStation;

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
    let sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    let status_a = check_status(&sat_a);
    let status_b = check_status(&sat_b);
    let status_c = check_status(&sat_c);
    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);

    let status_a = check_status(&sat_a);
    let status_b = check_status(&sat_b);
    let status_c = check_status(&sat_c);
    println!("a={:?} b={:?} c={:?}", status_a, status_b, status_c);
}
