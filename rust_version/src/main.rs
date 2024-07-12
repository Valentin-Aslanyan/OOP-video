use std::time::{Duration, Instant};

/// A function that is opaque to the optimizer, used to prevent the compiler from
/// optimizing away computations in a benchmark.
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
#[derive(Clone, Copy)]
struct Alice {
    x: i32,
    y: u8,
}
// the most obnoxious OOP you've seen!
#[allow(dead_code)]
impl Alice {
    fn get_x(&self) -> i32 {
        self.x
    }
    fn set_x(&mut self, v: i32) {
        self.x = v
    }
    fn get_y(&self) -> u8 {
        self.y
    }
    fn set_y(&mut self, v: u8) {
        self.y = v
    }
}

const ARR_SIZE: usize = 100000000;

#[inline(never)]
fn work_class_with_setters(arr: &mut [Alice]) {
    for (i, e) in arr.iter_mut().enumerate() {
        e.set_x(i as i32);
        e.set_y(e.get_y() + (i % 10) as u8);
    }
}

fn bench_class_with_setters() -> Duration {
    let a = Alice { x: 0, y: b'a' };
    let mut arr = vec![a; ARR_SIZE];
    let t1 = Instant::now();
    work_class_with_setters(arr.as_mut());
    black_box(arr);
    t1.elapsed()
}

#[inline(never)]
fn work_class(arr: &mut [Alice]) {
    for (i, e) in arr.iter_mut().enumerate() {
        e.x = i as i32;
        e.y += (i % 10) as u8;
    }
}

fn bench_class() -> Duration {
    let a = Alice { x: 0, y: b'a' };
    let mut arr = vec![a; ARR_SIZE];
    let t1 = Instant::now();
    work_class(arr.as_mut());
    black_box(arr);
    t1.elapsed()
}

#[inline(never)]
fn work_arrays(x: &mut [i32], y: &mut [u8]) {
    let x = &mut x[0..ARR_SIZE];
    let y = &mut y[0..ARR_SIZE];

    for i in 0..ARR_SIZE {
        x[i] = i as i32;
        y[i] += (i % 10) as u8;
    }
}

fn bench_arrays() -> Duration {
    let mut x = vec![0i32; ARR_SIZE];
    let mut y = vec![0u8; ARR_SIZE];
    let t1 = Instant::now();
    work_arrays(x.as_mut(), y.as_mut());
    black_box(x);
    black_box(y);
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
    bench_class_with_setters();
    run_bench("Class with setters", bench_class_with_setters);
    //warmup again
    bench_arrays();
    run_bench("arrays", bench_arrays);
}
