use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

const ITERATIONS:u64 = 1_000_000;

struct SharedData {
    last_updated_by: u64,
}

fn main() {
    for _ in 0..10 {
        let shared_data = Arc::new(Mutex::new(SharedData {
            last_updated_by: 1,
        }));

        let condvar = Arc::new(Condvar::new());

        let start = Instant::now();
        let thread1 = create_thread(shared_data.clone(), condvar.clone(), 1);
        let thread2 = create_thread(shared_data.clone(), condvar.clone(), 2);
        thread1.join().unwrap();
        thread2.join().unwrap();
        let end = Instant::now();

        let duration = end - start;
        println!("Total time: {:?}", duration);
        println!("      Rate: {:.0}/second", ITERATIONS as f64 / duration.as_secs_f64())
    }
}

fn create_thread(shared_data: Arc<Mutex<SharedData>>, condvar: Arc<Condvar>, id: u64) -> JoinHandle<()> {
    thread::Builder::new()
        .name(format!("thread-{}", id))
        .spawn(move || {
            let shared_data = shared_data.clone();
            let condvar = condvar.clone();
            let mut locked_shared_data = shared_data.lock().unwrap();
            for _ in 0..ITERATIONS {
                locked_shared_data = condvar.wait_while(locked_shared_data, |shared_data| {
                    shared_data.last_updated_by == id
                }).unwrap();

                locked_shared_data.last_updated_by = id;
                condvar.notify_all();
            }
        }).unwrap()
}
