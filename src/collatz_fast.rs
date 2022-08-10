fn collatz(n: u64) -> u64 {
    let mut x = n;
    let mut k: u64 = n.trailing_zeros().into();
    x >>= k;

    while x > 1 {
        x = 3*x+1;
        let ctz: u64 = x.trailing_zeros().into();
        x >>= ctz;
        k += ctz + 1;
    }

    return k;
}

fn collatz_set(from: u64, to: u64) -> (u64, u64) {
    let mut current_number: u64 = from;
    let mut highest_run: (u64, u64) = (0, 0);

    while current_number <= to {
        let ops =  collatz(current_number);

        if ops >= highest_run.0 {
            highest_run = (ops, current_number);
        }
        current_number += 1;
    }

    highest_run
}

pub fn multithreaded(from: usize, to: usize, workers: usize) -> (u64, u64) {
    let pool = threadpool::ThreadPool::new(workers);
    let number_count = to - from;
    let chunks = number_count / workers;

    let (tx, rx) = std::sync::mpsc::channel();

    // Process excess numbers that don't fit neatly into the worker chunks
    if number_count % workers != 0 {
        let excess = (workers * chunks)..number_count;
        let tx = tx.clone();
        pool.execute(move || {
            let result = collatz_set(excess.start as u64, excess.end as u64);
            tx.send(result).expect("Channel is waiting for pool");
        });
    }

    for i in 0..workers {
        let tx = tx.clone();
        pool.execute(move || {
            let result = collatz_set( (i * chunks) as u64, ((i+1) * chunks) as u64);
            tx.send(result).expect("Channel is waiting for pool");
        })
    }

    let mut best_result = (0, 0);
    for result in rx.iter().take(if number_count % workers != 0 { workers+1 } else { workers }) {
        if result.0 > best_result.0 {
            best_result = result;
        }
    }
    best_result
}