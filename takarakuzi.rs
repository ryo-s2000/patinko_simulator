extern crate rand;
use rand::Rng;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use thousands::Separable;

struct Channeles {
    sender_list: Vec<Sender<String>>
}

impl Channeles {
    pub fn new() -> Self {
        Channeles {
            sender_list: vec![]
        }
    }

    pub fn send_end_signal(&self) {
        for s in &self.sender_list {
            // rxがdropされるのでエラーが発生する
            s.send("end".to_string()).unwrap();
        }
    }
}

fn main() {
    let mut thread_handlers = vec![];
    let mut channeles = Channeles::new();
    let (end_tx, end_rx) = channel();

    let core: u8 = 9;
    for id in 1..core+1 {
        let (tx, rx) = channel();
        channeles.sender_list.push(tx);
        let end_tx = end_tx.clone();

        thread_handlers.push(
            thread::spawn(move || exec(id, core, rx, end_tx))
        );
    }

    let _stop_until = end_rx.recv();
    channeles.send_end_signal();

    for handle in thread_handlers {
        handle.join().unwrap();
    }
}

fn exec(id: u8, core: u8, rx: Receiver<String>, end_tx: Sender<String>) {
    let n: u8 = 37;
    let mut count: u128 = 0;

    loop {
        match rx.try_recv() {
            Ok(_) => break,
            _ => (),
        }

        count += 1;

        if sikou(n) && sikou(n-1) && sikou(n-2) && sikou(n-3) && sikou(n-4) && sikou(n-5) {
            let expected_count = count * core as u128;
            println!("-------------------------- ID:{} count = {} 買うのに必要な金額 {}円 --------------------------",
                id,
                expected_count.separate_with_commas(),
                (expected_count*30).separate_with_commas(),
            );
            end_tx.send("end".to_string()).unwrap();
            break;
        }

        if count % 100000000 == 0 {
            println!("ID:{} {}", id, count);
        }
    }

    println!("-------------------------- ID:{} thread end --------------------------", id);
}

fn sikou(n: u8) -> bool {
    let g: u8 = rand::thread_rng().gen_range(1, n);
    if g == rand::thread_rng().gen_range(1, n) {
        return true;
    } else {
        return false;
    }
}
