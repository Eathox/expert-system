use super::SequentialPermutationIter;

use crossbeam::channel::Sender;

pub struct ParallelPermutationIter {}

impl ParallelPermutationIter {
    pub fn new(sequential_iter: SequentialPermutationIter, sender: Sender<String>) {
        std::thread::spawn(move || {
            for p in sequential_iter {
                sender.send(p).unwrap();
            }
        });
    }
}
