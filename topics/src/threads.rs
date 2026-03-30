use std::thread;
use std::time::Duration;

pub fn spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("i am number {} in the spawned thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("i am number {} in the main thread", i);
        thread::sleep(Duration::from_millis(100));
    }
}
