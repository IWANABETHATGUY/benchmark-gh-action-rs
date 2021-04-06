#![feature(test)]

extern crate test;
use bench_action::fibonacci;
use test::Bencher;

#[bench]
fn fibonacci_30(b: &mut Bencher) {
    b.iter(|| fibonacci(30))
}

#[bench]
fn fibonacci_20(b: &mut Bencher) {
    b.iter(|| fibonacci(20))
}