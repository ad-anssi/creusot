#![feature(min_specialization)]
extern crate creusot_contracts;

use creusot_contracts::*;

struct Pair<T, U>(T, U);

unsafe impl<T1, T2> Resolve for Pair<T1, T2> {
    #[predicate]
    fn resolve(self) -> bool {
        Resolve::resolve(self.0) && Resolve::resolve(self.1)
    }
}
