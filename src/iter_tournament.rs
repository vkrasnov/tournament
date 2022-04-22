use std::{cmp::Ordering, collections::BinaryHeap};

use crate::comparator::{Comparator, MaxComparator, MinComparator};

/// A tournament that implements [`Iterator`] and merges [`Iterator`]s.
#[derive(Clone, Debug)]

pub struct Tournament<T, C>
where
    T: Iterator,
{
    tree: BinaryHeap<TournamentEntry<T::Item, C>>,
    results: Vec<T>,
}

impl<T> Tournament<T, MinComparator<T::Item>>
where
    T: Iterator,
    T::Item: Ord,
{
    /// A tournament that rates entries from smallest to largest.
    /// The provided iterators must yeild data from smallest to largest
    /// or the results are undefined.
    pub fn from_iters_min<I: IntoIterator<Item = T>>(
        iters: I,
    ) -> Tournament<T, MinComparator<T::Item>> {
        Tournament::from_iters(iters, MinComparator::default())
    }
}

impl<T> Tournament<T, MaxComparator<T::Item>>
where
    T: Iterator,
    T::Item: Ord,
{
    /// A tournament that rates entries from largest to smallest.
    /// The provided iterators must yeild data from largest to smallest
    /// or the results are undefined.
    pub fn from_iters_max<I: IntoIterator<Item = T>>(
        iters: I,
    ) -> Tournament<T, MaxComparator<T::Item>> {
        Tournament::from_iters(iters, MaxComparator::default())
    }
}

impl<T, C> Tournament<T, C>
where
    T: Iterator,
    C: Comparator<T::Item> + Clone,
{
    /// Create a new tournament from a set of iterators and a custom comparator.
    /// The iterators mush have the data sorted using the same semantics used
    /// be the provided comparator.
    ///
    /// # Examples
    ///
    /// ```
    /// use tournament::Tournament;
    ///
    /// #[derive(Clone, Copy)]
    /// struct CompareIgnoringCase {}
    ///
    /// impl tournament::Comparator<&str> for CompareIgnoringCase {
    ///     fn cmp(&self, a: &&str, b: &&str) -> core::cmp::Ordering {
    ///         a.to_lowercase().cmp(&b.to_lowercase())
    ///     }
    /// }
    ///
    /// let tournament = Tournament::from_iters(
    ///     vec![vec!["aa", "bb"].into_iter(), vec!["AA", "BB"].into_iter()],
    ///     CompareIgnoringCase {},
    /// );
    /// assert_eq!(tournament.collect::<Vec<_>>(), ["aa", "AA", "bb", "BB"]);
    /// ```
    ///
    pub fn from_iters<I: IntoIterator<Item = T>>(iters: I, comparator: C) -> Self {
        let mut tree = BinaryHeap::new();
        let mut results = Vec::new();

        for (index, mut iter) in iters.into_iter().enumerate() {
            if let Some(item) = iter.next() {
                tree.push(TournamentEntry {
                    item,
                    index,
                    comparator: comparator.clone(),
                });
            }
            results.push(iter);
        }

        Tournament { tree, results }
    }
}

impl<T, C> Iterator for Tournament<T, C>
where
    T: Iterator,
    C: Comparator<T::Item> + Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tree.pop() {
            None => None,
            Some(TournamentEntry {
                item,
                index,
                comparator,
            }) => {
                if let Some(item) = self.results[index].next() {
                    self.tree.push(TournamentEntry {
                        item,
                        index,
                        comparator,
                    });
                }
                Some(item)
            }
        }
    }
}

/// An entry into the inner binary tree that implements ['Ord`] over elements
/// of the tournament

#[derive(Clone, Debug)]

struct TournamentEntry<I, C> {
    item: I,
    index: usize,
    comparator: C,
}

impl<I, C> Ord for TournamentEntry<I, C>
where
    C: Comparator<I>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.comparator.cmp(&self.item, &other.item).reverse()
    }
}

impl<I, C> PartialOrd for TournamentEntry<I, C>
where
    C: Comparator<I>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C> PartialEq for TournamentEntry<I, C>
where
    C: Comparator<I>,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<I, C> Eq for TournamentEntry<I, C> where C: Comparator<I> {}

#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, DistString};

    use crate::Tournament;

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
            Tournament::from_iters_min(vecs.iter().map(|v| v.iter())).collect::<Vec<_>>();

        let mut sort_result = vecs.iter().flatten().collect::<Vec<_>>();
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
            Tournament::from_iters_max(vecs.iter().map(|v| v.iter())).collect::<Vec<_>>();

        let mut sort_result = vecs.iter().flatten().collect::<Vec<_>>();
        sort_result.sort_by(|a, b| b.cmp(a));

        assert_eq!(tournament_result, sort_result);
    }
}
