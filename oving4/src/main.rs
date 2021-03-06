use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::{thread, time};

fn main() {
    println!("Hello work!");

    let mut worker_threads: Workers = Workers::new(4);
    let mut event_loop: Workers = Workers::new(1);

    let one_second = time::Duration::from_millis(1000);
    let two_seconds = time::Duration::from_millis(2000);
    let three_seconds = time::Duration::from_millis(3000);
    let four_seconds = time::Duration::from_millis(4000);
    let five_seconds = time::Duration::from_millis(5000);
    let six_seconds = time::Duration::from_millis(6000);

    worker_threads.post(Box::new(move || {
        thread::sleep(five_seconds);
        println!("Task A");
    }));

    worker_threads.post_timeout(
        Box::new(move || {
            println!("Task B");
        }),
        five_seconds,
    );

    worker_threads.post(Box::new(move || {
        thread::sleep(one_second);
        println!("Task C");
    }));

    event_loop.post(Box::new(move || {
        thread::sleep(one_second);
        println!("Event loop task D");
    }));

    event_loop.post(Box::new(move || {
        thread::sleep(one_second);
        println!("Event loop task E");
    }));

    // I did not get the joinhandle to work properly
    // worker_threads.stop();
    // event_loop.stop();
    loop {}
}

//Based off of the rust book's implementation of a thread pool

pub struct Workers {
    handlers: Option<Vec<thread::JoinHandle<()>>>,
    notifier: Arc<(std::sync::Mutex<State>, std::sync::Condvar)>,
}

impl Workers {
    pub fn new(size: usize) -> Workers {
        assert!(size > 0);

        let state = State {
            queue: VecDeque::with_capacity(1024),
            stopped: false,
        };

        let notifer = Arc::new((Mutex::new(state), Condvar::new()));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let notifer = notifer.clone();
            let handle = thread::spawn(move || {
                while let Some(job) = Workers::next_task(&notifer) {
                    job.call_box();
                }
            });

            workers.push(handle);
        }

        Workers {
            handlers: Some(workers),
            notifier: notifer.clone(),
        }
    }

    // I did not get this stop function to work as it is supposed to...

    // pub fn stop(&mut self) {
    //     let &(ref lock, ref cvar) = &*self.notifier.clone();
    //     let mut state = lock.lock().unwrap();
    //     loop {
    //         if state.queue.len() == 0 {
    //             state.stopped = true;
    //             if let Some(handlers) = self.handlers.take() {
    //                 for handle in handlers {
    //                     handle.join().unwrap();
    //                 }
    //             }
    //         }
    //     }
    // }

    // Using the condition varable to check for a new task
    fn next_task(notifer: &Arc<(Mutex<State>, Condvar)>) -> Option<Job> {
        let &(ref lock, ref cvar) = &**notifer;
        let mut state = lock.lock().unwrap();
        loop {
            if state.stopped {
                return None;
            }
            match state.queue.pop_front() {
                Some(t) => {
                    return Some(t);
                }
                None => {
                    state = cvar.wait(state).unwrap();
                }
            }
        }
    }

    // Posting a task to the  queue
    pub fn post(&self, task: Job) {
        let &(ref lock, ref cvar) = &*self.notifier;
        {
            let mut state = lock.lock().unwrap();
            state.queue.push_back(task);
            // this notifies one of the worker that a new task is ready to complete
            cvar.notify_one();
        }
    }

    // The instructions for this function seems to be a little vague
    // Is the main thread suppposed to sleep before putting the Job in the queue?
    // Is the thread receiving this Job supposed to sleep before executing?
    pub fn post_timeout(&self, task: Job, timeout: time::Duration) {
        // I am just bundling a delay into the Boxed function
        let delayed_task = Box::new(move || {
            thread::sleep(timeout);
            task.call_box();
        });

        let &(ref lock, ref cvar) = &*self.notifier;
        {
            let mut state = lock.lock().unwrap();
            state.queue.push_back(delayed_task);
            cvar.notify_one();
        }
    }
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Job = Box<FnBox + Send>;

// The state holds the work queue and a boolean to check wether the queue has stopped or not
struct State {
    queue: VecDeque<Job>,
    stopped: bool,
}
