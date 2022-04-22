use core::cmp::Ordering;
use std::marker::PhantomData;

/// A [`Comparator`] can compare two items in the tournament to decide which is the winner.
pub trait Comparator<I: ?Sized> {
    /// Compare two results in the tournament, the winner is the smaller of the two as decide
    /// by [`Ordering::Less`]. [`Ordering::Equal`] is concidered a draw, and either contestant
    /// may be chosen as the winner.
    fn cmp(&self, a: &I, b: &I) -> Ordering;
}

/// A [`Comparator`] that choses the smaller result of the two
#[derive(Copy)]
pub struct MinComparator<I: ?Sized + Ord> {
    _p: PhantomData<I>,
}

impl<I: ?Sized + Ord> Default for MinComparator<I> {
    #[inline(always)]
    fn default() -> Self {
        MinComparator { _p: PhantomData }
    }
}

impl<I: ?Sized + Ord> Clone for MinComparator<I> {
    #[inline(always)]
    fn clone(&self) -> Self {
        MinComparator { _p: PhantomData }
    }
}

impl<I: ?Sized + Ord> Comparator<I> for MinComparator<I> {
    #[inline(always)]
    fn cmp(&self, a: &I, b: &I) -> Ordering {
        a.cmp(b)
    }
}

/// A [`Comparator`] that choses the larger result of the two
#[derive(Copy)]
pub struct MaxComparator<I: ?Sized + Ord> {
    _p: PhantomData<I>,
}

impl<I: ?Sized + Ord> Default for MaxComparator<I> {
    #[inline(always)]
    fn default() -> Self {
        MaxComparator { _p: PhantomData }
    }
}

impl<I: ?Sized + Ord> Clone for MaxComparator<I> {
    #[inline(always)]
    fn clone(&self) -> Self {
        MaxComparator { _p: PhantomData }
    }
}

impl<I: ?Sized + Ord> Comparator<I> for MaxComparator<I> {
    #[inline(always)]
    fn cmp(&self, a: &I, b: &I) -> Ordering {
        b.cmp(a)
    }
}
