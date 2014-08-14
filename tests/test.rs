
#![feature(globs)]

extern crate euler;
extern crate test;

use euler::*;
use test::Bencher;
use test::black_box;

#[test]
fn test_p1() {
    assert_eq!(p1(), 233_168);
}
#[bench]
fn bench_p1(b: &mut Bencher) {
    b.iter(|| black_box(p1()));
}
#[test]
fn test_p2() {
    assert_eq!(p2(), 4_613_732);
}
#[bench]
fn bench_p2(b: &mut Bencher) {
    b.iter(|| black_box(p2()));
}
#[test]
fn test_p3() {
    assert_eq!(p3(), 6_857);
}
#[bench]
fn bench_p3(b: &mut Bencher) {
    b.iter(|| black_box(p3()));
}
