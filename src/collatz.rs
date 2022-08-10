fn collatz(n: usize) -> usize {
    let mut runs = 0;
    let mut value = n;

    while value >= 2 {
        if value%2 == 0 {
            value /= 2;
        } else {
            value *= 3;
            value += 1;
        }
        runs += 1;
    }

    return runs;
}

fn collatz_set(from: usize, to: usize) -> (usize, usize) {
    let mut current_number: usize = from;
    let mut highest_run: (usize, usize) = (0, 0);

    while current_number <= to {
        let runs = collatz(current_number);

        if runs >= highest_run.0 {
            highest_run = (runs, current_number);
        }
        current_number += 1;
    }

    highest_run
}

pub fn multithreaded(from: usize, to: usize, workers: usize) -> (usize, usize) {
    let pool = threadpool::ThreadPool::new(workers);
    let number_count = to - from;
    let chunks = number_count / workers;

    let (tx, rx) = std::sync::mpsc::channel();

    // Process excess numbers that don't fit neatly into the worker chunks
    if number_count % workers != 0 {
        let excess = (workers * chunks)..number_count;
        let tx = tx.clone();
        pool.execute(move || {
            let result = collatz_set(excess.start, excess.end);
            tx.send(result).expect("Channel is waiting for pool");
        });
    }

    for i in 0..workers {
        let tx = tx.clone();
        pool.execute(move || {
            let result = collatz_set( i * chunks, (i+1) * chunks);
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