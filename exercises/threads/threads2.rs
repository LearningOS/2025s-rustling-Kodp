// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

//@ 好题目，值得重做！
//@ 好题目，值得重做！
//@ 好题目，值得重做！
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut target = status_shared.lock().unwrap();

            // / status_shared.lock() 方法会尝试获取锁。如果锁当前被其他线程持有，当前线程会被阻塞，直到锁被释放。。
            // / unwrap() 方法用于处理 lock() 方法返回的 Result 类型。如果获取锁的过程中发生错误（例如，持有锁的线程发生了 panic），unwrap() 会导致程序 panic。正常情况则解耦，返回类型是 MutexGuard。 
            // / 返回到的变量（target）在离开作用域后锁会自动释放。
            (*target).jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();  // 阻塞等待线程结束
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        /// 每个子线程完成执行后，获取 status 中 Mutex 的锁，并打印 jobs_completed 的当前值。
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
        // 主线程每次 join() 只等待特定线程完成，其他线程可能已经提前完成
    }
}
