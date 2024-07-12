#[derive(Clone, Copy)]
struct Alice {
    x: i32,
    y: u8,
}

const ARR_SIZE: usize = 100000000;

fn work_class(arr: &mut [Alice]) {
    for (i, e) in arr.iter_mut().enumerate() {
        e.x = i as i32;
        e.y += (i % 10) as u8;
    }
}

#[inline(never)]
fn bench_class() -> Duration {
    let a = Alice { x: 0, y: b'a' };
    let mut arr = vec![a; ARR_SIZE];
    work_class(arr.as_mut());
    println!("Result is {}", arr[42].x); // if this is removed, call to work_class is optimized away (as it does nothing)
    let t1 = Instant::now();
    t1.elapsed()
}

use std::time::{Duration, Instant};

fn work_arrays(x: &mut [i32], y: &mut [u8]) {
    for i in 0..ARR_SIZE {
        x[i] = i as i32;
        y[i] += (i % 10) as u8;
    }
}

#[inline(never)]
fn bench_arrays() -> Duration {
    let mut x = vec![0i32; ARR_SIZE];
    let mut y = vec![0u8; ARR_SIZE];
    let t1 = Instant::now();
    work_arrays(x.as_mut(), y.as_mut());
    t1.elapsed()
}

fn run_bench(name: &str, work: fn() -> Duration) {
    let elapsed = work();
    println!("{name} took {elapsed:?}");
}

fn main() {
    //warmup
    bench_class();
    run_bench("Class/struct", bench_class);
    //warmup again
    bench_arrays();
    run_bench("arrays", bench_arrays);
}
