#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, to: &mut CubeSat, msg: Message) {
        mailbox.post(to, msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id, mailbox: Mailbox { messages: vec![] } }
    }
}

impl CubeSat {
    fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
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

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox {
            messages: vec![],
        },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);

    let base1 = GroundStation { };

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base1.connect(sat_id);

        base.send(&mut sat, Message::from("hello"));
    }
}
