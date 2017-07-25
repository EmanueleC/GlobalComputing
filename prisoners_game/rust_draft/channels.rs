use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc::sync_channel;
use std::thread;

struct Bulb {
    on : bool,
    on_sender : SyncSender<i32>, // optional
    on_receiver : Receiver<i32>,
    off_sender : SyncSender<i32>, // optional
    off_receiver : Receiver<i32>
}

impl Bulb {
    fn run(&mut self) -> () {
        loop {
            if self.on {
                self.off_receiver.recv().unwrap();
                self.on = false;
            } else {
                self.on_receiver.recv().unwrap();
                self.on = true;
            }
        }
    }
}

fn prisoner(id : i32, off_sender : SyncSender<i32>) {
    off_sender.send(1).unwrap();
    println!("Prisoner {} switched the light off once", id);
    off_sender.send(1).unwrap();
    println!("Prisoner {} switched the light off twice", id);
}

fn counter_prisoner(n : i32, on_sender : SyncSender<i32>) {
    for i in 0..2*(n-1) {
        on_sender.send(1).unwrap();
        println!("CounterPrisoner switch the light on");
    }
    println!("Freedom!");
}

fn main() {
    let (on_sd, on_rcv) : (SyncSender<i32>, Receiver<i32>) = sync_channel(0);
    let (off_sd, off_rcv) : (SyncSender<i32>, Receiver<i32>) = sync_channel(0);
    let on_sdx = on_sd.clone();
    let off_sdx = off_sd.clone();
    thread::spawn(move || {
        let mut bulb = Bulb {
            on : true,
            on_sender : on_sd,
            on_receiver : on_rcv,
            off_sender : off_sd,
            off_receiver : off_rcv
        };
        bulb.run(); });
    for i in 0..49 {
        let off_sdx = off_sdx.clone();
        thread::spawn(move || { prisoner(i, off_sdx)});
    }
    thread::spawn(move || { counter_prisoner(50, on_sdx); }).join();
}
