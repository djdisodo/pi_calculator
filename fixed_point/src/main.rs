use std::f64::consts::PI;
use integer_sqrt::IntegerSquareRoot;
use std::ops::Range;
use std::time::{Instant, Duration};

fn main() {
    let start_time = Instant::now();
    

    let mut args = std::env::args();

    args.next(); //path

    let threads_count: u64 = args.next().unwrap().parse().unwrap();

    let chunks: u64 = args.next().unwrap().parse().unwrap();

    let chunks_pow = chunks.pow(2);

    let chunks_per_thread = chunks / threads_count;

    let mut threads = Vec::with_capacity(threads_count as usize);

    let mut left_chunks = chunks;

    for i in 1..threads_count {
        left_chunks -= chunks_per_thread;
        let range = ((i - 1) * chunks_per_thread)..(i * chunks_per_thread);
        threads.push(std::thread::spawn(move || {
            calc(range, chunks_pow)
        }));
    }

    let range = ((threads_count - 1) * chunks_per_thread)..((threads_count - 1) * chunks_per_thread + left_chunks);
    threads.push(std::thread::spawn(move || {
        calc(range, chunks_pow)
    }));

    let mut sum: u128 = 0;

    for x in threads {
        sum += x.join().unwrap();
    }

    let size = (sum << 2) + ((chunks as u128) << 1);
    println!("{}", size as f64 / chunks_pow as f64);
    println!("run time: {}s", Instant::now().duration_since(start_time).as_secs_f32());
}

fn calc(range: Range<u64>, chunks_pow: u64) -> u128 {

    let mut sum: u128 = 0;

    for i in range {
        sum += (chunks_pow - i.pow(2)).integer_sqrt() as u128;
    }
    sum
}
