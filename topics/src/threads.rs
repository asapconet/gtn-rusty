use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// stage one -- regular spawn
pub fn spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("i am number {} in the spawned thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("i am number {} in the main thread", i);
        thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap();
}

// stage two -- channel thread
pub fn channel_thread() {
    let (tx, rx) = mpsc::channel();
    let txl = tx.clone();
    let txlx = tx.clone();

    thread::spawn(move || {
        let value = String::from("hyia su original");
        txl.send(value).unwrap();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hyia su"),
            String::from("from"),
            String::from("spawened"),
            String::from("thread"),
        ];

        for val in vals {
            txlx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let more_vals = vec![
            String::from("more"),
            String::from("values"),
            String::from("another"),
            String::from("another"),
            String::from("producer"),
        ];

        for val in more_vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("got: {}", received)
    }
}
