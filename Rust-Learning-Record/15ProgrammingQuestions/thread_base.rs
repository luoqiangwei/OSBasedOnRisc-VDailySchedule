use std::thread;
use std::time::Duration;

fn main() {
    let handle0 = thread::spawn(move || {
        for i in 1..10 {
            println!("A {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle1 = thread::spawn(move || {
        for i in 1..10 {
            println!("B {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 1..10 {
            println!("C {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle0.join().unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();
}