use rustyscript::{
    worker::{DefaultWorker, DefaultWorkerQuery, WorkerPool},
    Error,
};

fn main() -> Result<(), Error> {
    let mut pool = WorkerPool::<DefaultWorker>::new(Default::default(), 4)?;

    // Get the next available worker
    let worker_a = pool.next_worker();
    worker_a.borrow().send(DefaultWorkerQuery::Eval(
        "console.log('Hello from worker A!')".to_string(),
    ))?;

    // Start a long-running task in worker B
    let worker_b = pool.next_worker();
    worker_b.borrow().send(DefaultWorkerQuery::Eval(
        "for (let i = 0; i < 10000000000; i++) {} ".to_string(),
    ))?;

    // Wait for worker B to finish
    worker_b.borrow().receive()?;

    Ok(())
}
