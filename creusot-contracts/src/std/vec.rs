use crate as creusot_contracts;
use crate::logic::*;
use crate::{std::clone::Clone, Int, Model, Seq};
use creusot_contracts_proc::*;

impl<T, A : std::alloc::Allocator> Model for Vec<T, A> {
    type ModelTy = Seq<T>;
    #[logic]
    #[trusted]
    fn model(self) -> Self::ModelTy {
        std::process::abort()
    }
}

extern_spec! {
    #[ensures((@result).len() === 0)]
    fn std::vec::Vec::new<T>() -> std::vec::Vec<T>
}

// extern_spec! {
//     fn std::vec::Vec::len<T, A : std::alloc::Allocator>(v: &std::vec::Vec<T, A>) -> usize
// }

extern_spec! {
    #[ensures(result.into() === (@v).len())]
    fn std::vec::Vec::len<T,A : std::alloc::Allocator>(v: &Vec<T, A>) -> usize
}


extern_spec! {
    #[ensures(@^self_ === (@self_).push(v))]
    fn std::vec::Vec::push<T>(self_: &mut Vec<T>, v: T)
}

// extern_spec! {
//     #[requires(@i < (@self_).len())]
//     #[requires(@j < (@self_).len())]
//     #[ensures((@^self_).exchange(@*self_, @i, @j))]
//     fn std::vec::Vec::swap<T>(self_: &mut Vec<T>, i: usize, j: usize)
// }

// impl<T> Vec<T> {
//     #[trusted]
//     #[ensures((@result).len() === 0)]
//     pub fn new() -> Self {
//         Vec(std::vec::Vec::new())
//     }

//     #[trusted]
//     #[ensures((@result).len() === 0)]
//     pub fn with_capacity(capacity: usize) -> Vec<T> {
//         Vec(std::vec::Vec::with_capacity(capacity))
//     }

//     #[trusted]
//     #[ensures(result.into() === (@self).len())]
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     #[trusted]
//     #[ensures(match result {
//         Some(t) => *t === (@*self)[ix.into()],
//         None => (@*self).len() <= ix.into(),
//     })]
//     pub fn get(&self, ix: usize) -> Option<&T> {
//         self.0.get(ix)
//     }

//     #[trusted]
//     #[ensures(@^self === (@self).push(v))]
//     pub fn push(&mut self, v: T) {
//         self.0.push(v)
//     }

//     #[trusted]
//     #[requires(@i < (@self).len())]
//     #[requires(@j < (@self).len())]
//     #[ensures((@^self).exchange(@*self, @i, @j))]
//     pub fn swap(&mut self, i: usize, j: usize) {
//         self.0.swap(i, j)
//     }

//     #[trusted]
//     #[ensures(match result {
//         Some(t) => (@self) === (@^self).push(t),
//         None => (@self).len() === (@^self).len() && (@self).len() === 0
//     })]
//     pub fn pop(&mut self) -> Option<T> {
//         self.0.pop()
//     }
// }

// impl<T> std::ops::IndexMut<usize> for Vec<T> {
//     #[trusted]
//     #[requires(@ix < (@*self).len())]
//     #[ensures(*result === (@self)[@ix])]
//     #[ensures(^result === (@^self)[@ix])]
//     #[ensures(forall<j : Int> 0 <= j && j < (@^self).len() ==>
//         !(j === @ix) ==>
//         (@^self)[j] === (@*self)[j])]
//     #[ensures((@*self).len() === (@^self).len())]
//     fn index_mut(&mut self, ix: usize) -> &mut T {
//         self.0.index_mut(ix)
//     }
// }

use std::slice::SliceIndex;
use std::ops::{Index, IndexMut};
// impl<T, I> std::ops::Index<I> for Vec<T>
//     where I : SliceIndex<[T]> {
//     type Output = <I as SliceIndex<[T]>>::Output;

//     #[trusted]
//     // #[requires(@ix < (@self).len())]
//     // #[ensures(*result === (@self)[@ix])]
//     fn index(&self, ix: I) -> &T {
//         self.0.index(ix)
//     }
// }
//
trait IndexSpec<I> : Index<I> {

    // Check whether an index is 'in bounds' for a structure
    #[predicate]
    fn in_bounds(self, i: I) -> bool;

    // Condition under which we get `out` from index `i` in `self`
    #[predicate]
    fn has_elem_at(self, i: I, out: Self::Output) -> bool;
}

extern_spec! {
    #[requires(self_.in_bounds(i))]
    #[ensures(self_.has_elem_at(i, *result))]
    fn std::ops::Index::index<T, I>(self_: &T, i: I) -> &T::Output
        where T : IndexSpec<I>
}

trait IndexMutSpec<I> : IndexMut<I> {
    // Check whether an index is 'in bounds' for a structure
    #[predicate]
    fn in_bounds(self, i: I) -> bool;

    // Condition underwhich we get `out` from index `i` in `self`
    #[predicate]
    fn has_elem_at(self, i: I, out: Self::Output) -> bool;

    // Explains what happens to the elements we didn't index
    #[predicate]
    fn resolve_except(&mut self, i: I) -> bool;
}


extern_spec! {
    #[requires(self_.in_bounds(i))]
    #[ensures((*self_).has_elem_at(i, *result))]
    #[ensures((^self_).has_elem_at(i, ^result))]
    #[ensures(self_.resolve_except(i))]
    fn std::ops::IndexMut::index_mut<T, I>(self_: &mut T, i: I) -> &mut T::Output
        where
                T : IndexMutSpec<I>,
}

trait SliceIndexSpec<T : ?Sized> : SliceIndex<T>
    {
    // Check whether an index is 'in bounds' for a structure
    #[predicate]
    fn in_bounds(self, s: T) -> bool;

    // Condition underwhich we get `out` from index `i` in `self`
    #[predicate]
    fn has_elem_at(self, s: T, out: Self::Output) -> bool;

    // Explains what happens to the elements we didn't index
    #[predicate]
    fn resolve_except(self, s: &mut T) -> bool;
}

// We probably want to move these into a `SliceIndexSpec` trait as well...
impl<T, I : SliceIndexSpec<[T]>,A : std::alloc::Allocator> IndexMutSpec<I> for Vec<T, A> {
    #[predicate]
    fn in_bounds(self, i : I) -> bool {
        pearlite! { i.in_bounds(self) }
    }

    #[predicate]
    fn has_elem_at(self, i: I, out: Self::Output) -> bool {
        pearlite! { i.has_elem_at(self, out) }
    }

    #[predicate]
    fn resolve_except(&mut self, i : I) -> bool {
        pearlite! { i.resolve_except(self) }
    }
}

impl<T> SliceIndexSpec<[T]> for usize {
    #[predicate]
    fn in_bounds(self, s: [T]) -> bool {
        pearlite! { @self < (@s).len() }
    }

    #[predicate]
    fn has_elem_at(self, s: [T], out: Self::Output) -> bool {
        pearlite! { (@s)[@self] === out }
    }

    #[predicate]
    fn resolve_except(self, old: [T], new: [T]) -> bool {
        pearlite! {
            (@old).len() === (@new).len() &&
            forall<i : Int> 0 <= i && i != @self && i < (@old).len() ==> (@old)[i] === (@new)[i]
        }
    }
}


extern_spec! {
    #[requires(self_.in_bounds(ix))]
    #[ensures((*self_).has_elem_at(ix, *result))]
    #[ensures((^self_).has_elem_at(ix, ^result))]
    #[ensures(self_.resolve_except(ix))]
    fn std::vec::Vec::index_mut<T, I : SliceIndexSpec<[T]>, A : std::alloc::Allocator>(self_: &mut Vec<T, A>, ix: I) -> &mut <I as SliceIndex<[T]>>::Output
        // where [T] : IndexMutSpec<I>
}

// TODO: Ensure extern functions & extern_specs inherit trait contracts


// extern_spec! {
//   fn std::vec::Vec::index<T, I : SliceIndex<[T]>>(self_: &Vec<T>, ix: I) -> &<I as SliceIndex<[T]>>::Output
//     where crate::Seq<T> : Index<I, Output = <I as SliceIndex<[T]>>::Output>
// }

// trait SliceIndexSpec<T> : SliceIndex<[T]> {
//     // type Output;

//     #[predicate]
//     fn in_bounds(self, seq: Seq<T>) -> bool;

//     // TODO: better name
//     #[predicate]
//     fn partition(self, seq: Seq<T>, res: Self::Output) -> bool;
// }

// impl<T> SliceIndexSpec<T> for usize {
//     #[predicate]
//     fn in_bounds(self, seq: Seq<T>) -> bool {
//         pearlite! { @self < seq.len() }
//     }

//     #[predicate]
//     fn partition(self, seq: Seq<T>, res: Self::Output) -> bool {
//         pearlite! { seq.set(@self, res) }
//     }
// }


// extern_spec! {
//   fn std::vec::Vec::index<T, I : SliceIndex<[T]>>(self_: &Vec<T>, ix: I) -> &<I as SliceIndex<[T]>>::Output
//     where crate::Seq<T> : Index<I, Output = <I as SliceIndex<[T]>>::Output>
// }

// extern_spec! {
//   #[requires(ix.in_bounds(@self_))]
//   #[ensures(ix.partition(@*self_, *result))]
//   #[ensures(ix.partition(@^self_, ^result))]
//   fn std::vec::Vec::index_mut<T, I : SliceIndexSpec<T>>(self_: &mut Vec<T>, ix: I) -> &mut <I as SliceIndex<[T]>>::Output
//     // where crate::Seq<T> : Index<I>,
//     //       <I as SliceIndex<[T]>>::Output : Model,
// }

// impl<T: Clone> Clone for Vec<T> {
//     #[trusted]
//     fn clone(&self) -> Self {
//         panic!()
//         // Vec(self.0.iter().map(|r : &T| r.clone()).collect())
//     }
// }

unsafe impl<T> Resolve for Vec<T> {
    #[predicate]
    fn resolve(self) -> bool {
        pearlite! { forall<i : Int> 0 <= i && i < (@self).len() ==> (@self)[i].resolve() }
    }
}

// #[trusted]
// #[ensures((@result).len() === @n)]
// #[ensures(forall<i : Int> 0 <= i && i < @n ==> (@result)[i] === elem)]
// pub fn from_elem<T: Clone>(elem: T, n: usize) -> Vec<T> {
//     panic!()
// }
