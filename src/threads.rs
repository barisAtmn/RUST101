use std::thread ;

fn spawn_test() {
    let handle = thread::spawn(move || {

    });

    // wait until thread has exited
    // it may return panic or result.
    let td = handle.join().unwrap();
}