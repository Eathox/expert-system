use super::SequentialPermutationIter;

use crossbeam::channel::{bounded, Receiver};

pub struct ParallelPermutationIter {
    receiver: Receiver<String>,
}

impl ParallelPermutationIter {
    pub fn new(
        sequential_iter: SequentialPermutationIter,
        buff_size: usize,
    ) -> ParallelPermutationIter {
        let (sender, receiver) = bounded(buff_size);
        std::thread::spawn(move || {
            for p in sequential_iter {
                sender.send(p).unwrap();
            }
        });
        ParallelPermutationIter { receiver }
    }
}

impl Iterator for ParallelPermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}
