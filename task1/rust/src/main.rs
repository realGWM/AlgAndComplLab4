use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use core::iter::StepBy;
use std::ops::RangeInclusive;
use std::io::{self, Write};
use std::fs::File;

const TIMES:        usize = 2_000;         //how many times do we test each size in order to minimize statistical error
const MIN:          usize = 20;            //min array size
const MAX:          usize = 4_000;         //max array size
const STEP:         usize = 20;            //step for incrementing array size
const UPPER_BOUND1: u32   = 1e3 as u32;
const UPPER_BOUND2: u32   = 1e4 as u32;
const UPPER_BOUND3: u32   = 1e5 as u32;
const UPPER_BOUND4: u32   = 1e6 as u32;

fn main() {
    let mut rng = rand::thread_rng();

    let result1 = worker_gen_sizes(&mut rng, UPPER_BOUND1);
    let result2 = worker_gen_sizes(&mut rng, UPPER_BOUND2);
    let result3 = worker_gen_sizes(&mut rng, UPPER_BOUND3);
    let result4 = worker_gen_sizes(&mut rng, UPPER_BOUND4);

    //"Sizes are the same" + "but I'm lazy" yet again, but now with some workaround :D
    save_results(&result1, "../r/sizes.txt", "../r/simple_totals1.txt", "../r/sieve_totals1.txt");
    save_results(&result2, "/dev/null", "../r/simple_totals2.txt", "../r/sieve_totals2.txt");
    save_results(&result3, "/dev/null", "../r/simple_totals3.txt", "../r/sieve_totals3.txt");
    save_results(&result4, "/dev/null", "../r/simple_totals4.txt", "../r/sieve_totals4.txt");
}

fn gen_sizes() -> StepBy<RangeInclusive<usize>> {
    let min = if MIN != 0 { MIN } else { MIN + STEP };

    (min..=MAX).step_by(STEP)
}

fn worker_gen_sizes(rng: &mut ThreadRng, n: u32) -> Result {
    worker(gen_sizes(), rng, n)
}

fn worker(sizes_iter: StepBy<RangeInclusive<usize>>, rng: &mut ThreadRng, n: u32) -> Result {
    let dist = Uniform::new_inclusive(0, n);

    //Yeah, the same reason as before...
    let mut simple_useless_variable: usize = 0;
    let mut sieve_useless_variable: usize = 0;

    let mut sizes: Vec<usize> = Vec::new();
    let mut simple_totals: Vec<u128> = Vec::new();
    let mut sieve_totals: Vec<u128> = Vec::new();

    for size in sizes_iter {
        let mut simple_total: u128 = 0;
        let mut sieve_total: u128 = 0;
        let mut haystack: Vec<u32> = vec![0; size];

        for _ in 0..TIMES {

            for i in haystack.iter_mut() {
                *i = dist.sample(rng);
            }


            let simpe_start = Instant::now();

            for value in haystack.iter() {
                if is_prime(*value) {
                    simple_useless_variable += 1;
                }
            }

            let simple_end = Instant::now();
            let simple_time_taken = simple_end.duration_since(simpe_start).as_nanos();
            simple_total += simple_time_taken;


            let sieve_start = Instant::now();

            let sieve = gen_sieve_of_eratosthenes(n);

            for value in haystack.iter() {
                if sieve[*value as usize] {
                    sieve_useless_variable += 1;
                }
            }

            let sieve_end = Instant::now();
            let sieve_time_taken = sieve_end.duration_since(sieve_start).as_nanos();
            sieve_total += sieve_time_taken;
        }

        simple_total /= TIMES as u128;
        sieve_total /= TIMES as u128;
        println!("size = {}, simple total = {}, sieve total = {}", size, simple_total, sieve_total);
        sizes.push(size);
        simple_totals.push(simple_total);
        sieve_totals.push(sieve_total);
    }

    println!("Also same as before, yeah... Simple: {}, Sieve: {}", simple_useless_variable, sieve_useless_variable);
    //But also, to be extra safe, we can check whether the number of primes counted by different methods is the same
    assert!(simple_useless_variable == sieve_useless_variable);

    Result { sizes, simple_totals, sieve_totals }
}

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    let sqrt = (num as f64).sqrt() as u32;
    for i in 2..=sqrt {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn gen_sieve_of_eratosthenes(n: u32) -> Vec<bool> {
    let sqrt = (n as f64).sqrt() as u32;
    let mut sieve = vec![true; n as usize + 1];
    sieve[0] = false; //0 is not prime
    sieve[1] = false; //1 is not prime
    for i in 2..=sqrt {
        if sieve[i as usize] {
            let mut j = i*i;
            while j <= n {
                sieve[j as usize] = false;
                j += i;
            }
        }
    }
    sieve
}

struct Result {

    sizes: Vec<usize>,
    simple_totals: Vec<u128>,
    sieve_totals: Vec<u128>
}

//Save results to a file and print them to stdout
fn save_results(result: &Result, sizes_file_name: &str, simple_totals_file_name: &str, sieve_totals_file_name: &str) {
    let mut sizes_string = result.sizes.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    let mut simple_totals_string = result.simple_totals.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    let mut sieve_totals_string = result.sieve_totals.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    sizes_string.push('\n');
    simple_totals_string.push('\n');
    sieve_totals_string.push('\n');
    io::stdout().write_all(sizes_string.as_bytes()).unwrap();
    io::stdout().write_all(simple_totals_string.as_bytes()).unwrap();
    io::stdout().write_all(sieve_totals_string.as_bytes()).unwrap();
    let mut sizes_file = File::create(sizes_file_name).unwrap();
    let mut simple_totals_file = File::create(simple_totals_file_name).unwrap();
    let mut sieve_totals_file = File::create(sieve_totals_file_name).unwrap();
    sizes_file.write_all(sizes_string.as_bytes()).unwrap();
    simple_totals_file.write_all(simple_totals_string.as_bytes()).unwrap();
    sieve_totals_file.write_all(sieve_totals_string.as_bytes()).unwrap();
}
