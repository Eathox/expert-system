extern crate expert_system;

use expert_system::*;
use std::fs::File;

// fn bench_medium(b: &mut Bencher) {
//     b.iter(|| {
//         TruthTable::try_from(
//             "A + B + C + D + E + F + G + H + I + J + K + L + M + N + O + P + Q => Z",
//         )
//         .unwrap()
//     });
// }

// fn bench_permutation_iter(b: &mut Bencher) {
//     b.iter(|| TruthTable::try_from("A + B + C + D + E + F + G + H + I + J + K + L + M + N + O + P + Q + R + S + T + U + V + W + X + Y => Z").unwrap());
// }

// // // fn two(b: &mut Bencher) {
// // //     b.iter(|| TruthTable::try_from("A => Z").unwrap());
// // // }

// // // fn three(b: &mut Bencher) {
// // //     b.iter(|| TruthTable::try_from("A + B => Z").unwrap());
// // // }

// // // fn four(b: &mut Bencher) {
// // //     b.iter(|| TruthTable::try_from("A + B + C => Z").unwrap());
// // // }

// // // fn five(b: &mut Bencher) {
// // //     b.iter(|| TruthTable::try_from("A + B + C + D => Z").unwrap());
// // // }

// // // fn six(b: &mut Bencher) {
// // //     b.iter(|| TruthTable::try_from("A + B + C + D + E => Z").unwrap());
// // // }

// benchmark_group!(benches, bench_medium);
// benchmark_group!(benches, bench_permutation_iter);
// // benchmark_group!(benches, two, three, four, five, six);
// benchmark_main!(benches);

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // let res = TruthTable::try_from("A + B + C + D + E + F + G + H + I + J + K + L + M + N + O + P + Q + R + S + T + U + V + W + X + Y => Z");
    let res = TruthTable::try_from(
        "A + B + C + D + E + F + G + H + I + J + K + L + M + N + O + P + Q + R => Z",
    );

    let elapsed = now.elapsed();
    println!("Ok? {:?}", res.is_ok());
    println!("Elapsed: {}", elapsed.as_secs_f64());
}
