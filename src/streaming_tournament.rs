use std::{cmp::Ordering, collections::BinaryHeap};

use streaming_iterator::StreamingIterator;

use crate::comparator::{Comparator, MaxComparator, MinComparator};

/// A tournament that implements [`StreamingIterator`] and merges [`StreamingIterator`]s
#[derive(Clone, Debug)]
pub struct StreamingTournament<T, C>
where
    T: StreamingIterator,
{
    // Indicates if first call to advance was made
    started: bool,
    // The tree that stores the "contestants"
    tree: BinaryHeap<StreamingTournamentEntry<T, C>>,
}

impl<T> StreamingTournament<T, MinComparator<T::Item>>
where
    T: StreamingIterator,
    T::Item: Ord,
{
    /// A tournament that rates entries from smallest to largest.
    /// The provided iterators must yeild data from smallest to largest
    /// or the results are undefined.
    pub fn from_iters_min<I: IntoIterator<Item = T>>(
        iters: I,
    ) -> StreamingTournament<T, MinComparator<T::Item>> {
        StreamingTournament::from_iters(iters, MinComparator::default())
    }
}

impl<T> StreamingTournament<T, MaxComparator<T::Item>>
where
    T: StreamingIterator,
    T::Item: Ord,
{
    /// A tournament that rates entries from largest to smallest.
    /// The provided iterators must yeild data from largest to smallest
    /// or the results are undefined.
    pub fn from_iters_max<I: IntoIterator<Item = T>>(
        iters: I,
    ) -> StreamingTournament<T, MaxComparator<T::Item>> {
        StreamingTournament::from_iters(iters, MaxComparator::default())
    }
}

impl<T, C> StreamingTournament<T, C>
where
    T: StreamingIterator,
    C: Comparator<T::Item> + Clone,
{
    /// Create a tournament with a custom comparator
    pub fn from_iters<I: IntoIterator<Item = T>>(
        iters: I,
        comparator: C,
    ) -> StreamingTournament<T, C> {
        let mut tree = BinaryHeap::new();
        for mut iter in iters {
            iter.advance();
            if iter.get().is_some() {
                tree.push(StreamingTournamentEntry {
                    iter,
                    comparator: comparator.clone(),
                });
            }
        }

        StreamingTournament {
            tree,
            started: false,
        }
    }
}

impl<T, F> StreamingIterator for StreamingTournament<T, F>
where
    T: StreamingIterator,
    F: Comparator<T::Item>,
{
    type Item = T::Item;

    fn advance(&mut self) {
        if !self.started {
            self.started = true;
            return;
        }

        match self.tree.pop() {
            None => {}
            Some(StreamingTournamentEntry {
                mut iter,
                comparator,
            }) => {
                iter.advance();
                if iter.get().is_some() {
                    self.tree
                        .push(StreamingTournamentEntry { iter, comparator });
                }
            }
        }
    }

    fn get(&self) -> Option<&<Self as StreamingIterator>::Item> {
        self.tree.peek().and_then(|i| i.iter.get())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.tree.iter().fold(
            (self.tree.len(), Some(self.tree.len())),
            |(lower, upper), i| {
                let (l, u) = i.iter.size_hint();
                (lower + l, upper.zip(u).map(|(u1, u2)| u1 + u2))
            },
        )
    }
}

/// An entry into the inner binary tree that implements ['Ord`]
/// over the inner `[StreamingIterator]` by comparing the current
/// element of each iterator. This is implemented that way because
/// the data is acutally owned by the iterator, and it is impossible
/// to have any external references to it, while still allowing mutable
/// access.
#[derive(Clone, Debug)]
struct StreamingTournamentEntry<I, C>
where
    I: StreamingIterator,
{
    iter: I,
    comparator: C,
}

impl<I, C> Ord for StreamingTournamentEntry<I, C>
where
    I: StreamingIterator,
    C: Comparator<I::Item>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.comparator
            .cmp(
                self.iter.get().as_ref().unwrap(),
                other.iter.get().as_ref().unwrap(),
            )
            .reverse()
    }
}

impl<I, C> PartialOrd for StreamingTournamentEntry<I, C>
where
    I: StreamingIterator,
    C: Comparator<I::Item>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C> PartialEq for StreamingTournamentEntry<I, C>
where
    I: StreamingIterator,
    C: Comparator<I::Item>,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<I, C> Eq for StreamingTournamentEntry<I, C>
where
    I: StreamingIterator,
    C: Comparator<I::Item>,
{
}

#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, DistString};
    use streaming_iterator::StreamingIterator;

    use crate::StreamingTournament;

    #[test]
    fn test_min() {
        let mut rng = rand::thread_rng();

        let vecs = (1..100)
            .map(|_| {
                (1..200)
                    .map(|_| Alphanumeric.sample_string(&mut rng, 32))
                    .collect::<Vec<_>>()
            })
            .map(|mut v| {
                v.sort();
                v
            })
            .collect::<Vec<_>>();

        let tournament_result =
            StreamingTournament::from_iters_min(vecs.iter().map(streaming_iterator::convert_ref))
                .cloned()
                .collect::<Vec<_>>();

        let mut sort_result = vecs.iter().flatten().cloned().collect::<Vec<_>>();
        sort_result.sort();

        assert_eq!(tournament_result, sort_result);
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();

        let vecs = (1..100)
            .map(|_| {
                (1..200)
                    .map(|_| Alphanumeric.sample_string(&mut rng, 32))
                    .collect::<Vec<_>>()
            })
            .map(|mut v| {
                v.sort_by(|a, b| b.cmp(a));
                v
            })
            .collect::<Vec<_>>();

        let tournament_result =
            StreamingTournament::from_iters_max(vecs.iter().map(streaming_iterator::convert_ref))
                .cloned()
                .collect::<Vec<_>>();

        let mut sort_result = vecs.iter().flatten().cloned().collect::<Vec<_>>();
        sort_result.sort_by(|a, b| b.cmp(a));

        assert_eq!(tournament_result, sort_result);
    }
}
