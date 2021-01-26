//! # `incrementalmerkletree`
//!
//! Incremental Merkle Trees are fixed-depth Merkle trees with two primary
//! capabilities: appending (assigning a value to the next unused leaf and
//! advancing the tree) and obtaining the root of the tree. Importantly the tree
//! structure attempts to store the least amount of information necessary to
//! continue to function; other information should be pruned eagerly to avoid
//! waste when the tree state is encoded.
//!
//! ## Witnessing
//!
//! Merkle trees are typically used to show that a value exists in the tree via
//! an authentication path. We need an API that allows us to identify the
//! current leaf as a value we wish to compute authentication paths for even as
//! the tree continues to be appended to in the future; this is called
//! maintaining a witness. When we're later uninterested in such a leaf, we can
//! prune a witness and remove all unnecessary information from the structure as
//! a consequence.
//!
//! ## Checkpoints and Rollbacks
//!
//! The structure is not append-only in the strict sense. It is possible to
//! identify the current state of the tree as a "checkpoint" and to remove older
//! checkpoints that we're no longer interested in. It should be possible to
//! roll back to any previous checkpoint.

pub trait TreeHasher {
    type Digest: Clone + PartialEq + std::fmt::Debug;

    fn empty_leaf() -> Self::Digest;
    fn combine(a: &Self::Digest, b: &Self::Digest) -> Self::Digest;
}

#[derive(Clone)]
pub struct Subtree<H: TreeHasher> {
    start_position: usize,
    left: Option<H::Digest>,
    right: Option<H::Digest>,
    parents: Vec<Option<H::Digest>>,
}

#[derive(Clone)]
pub struct EfficientTree<H: TreeHasher> {
    depth: usize,
    subtrees: Vec<Subtree<H>>,
}

impl<H: TreeHasher> Subtree<H> {
    fn size(&self) -> usize {
        let mut b = 0;
        for parent in &self.parents {
            if parent.is_some() {
                b += 1;
            }
            b <<= 1;
        }
        if self.left.is_some() {
            b += 1;
        }
        if self.right.is_some() {
            b += 1;
        }
        b
    }

    fn current_position(&self) -> usize {
        self.start_position + self.size()
    }

    fn append(&mut self, depth: usize, value: &H::Digest) -> bool {
        if self.start_position % 2 == 1 {
            if self.right.is_none() {
                self.right = Some(value.clone());
            } else {
                return false;
            }
        } else {
            if self.left.is_none() {
                self.left = Some(value.clone());
            } else if self.right.is_none() {
                self.right = Some(value.clone())
            } else {
                if self.size() == self.max_size(depth) {
                    return false;
                }
                let left = self.left.take().unwrap();
                let right = self.right.take().unwrap();
                self.collapse(left, right);
                self.left = Some(value.clone());
            }
        }

        true
    }

    fn max_size(&self, depth: usize) -> usize {
        let mut k = 0;
        let mut start_position = self.start_position;
        for _ in 0..64 {
            if (start_position & 1) == 0 {
                k += 1;
                start_position >>= 1;
            } else {
                break;
            }
        }
        std::cmp::min(1 << k, 1 << depth)
    }

    fn collapse(&mut self, left: H::Digest, right: H::Digest) {
        let mut cur = H::combine(&left, &right);
        for parent in &mut self.parents {
            if parent.is_none() {
                *parent = Some(cur);
                return;
            } else {
                cur = H::combine(&parent.take().unwrap(), &cur);
            }
        }
        self.parents.push(Some(cur));
    }
}

impl<H: TreeHasher> EfficientTree<H> {
    pub fn new(depth: usize) -> Self {
        EfficientTree {
            depth,
            subtrees: vec![Subtree {
                start_position: 0,
                left: None,
                right: None,
                parents: vec![],
            }],
        }
    }

    pub fn append(&mut self, value: &H::Digest) -> bool {
        if !self.subtrees.last_mut().unwrap().append(self.depth, value) {
            let start_position = self.subtrees.last().unwrap().current_position();
            let mut new_subtree = Subtree {
                start_position,
                left: None,
                right: None,
                parents: vec![],
            };
            let ret = new_subtree.append(self.depth, value);
            if ret {
                self.subtrees.push(new_subtree);

                true
            } else {
                false
            }
        } else {
            true
        }
    }

    /// Obtains the current root of this Merkle tree.
    pub fn root(&self) -> H::Digest {
        unimplemented!()
    }

    /// Marks the current tree state leaf as a value that we're interested in
    /// witnessing. Returns true if successful and false if the tree is empty.
    pub fn witness(&mut self) -> bool {
        unimplemented!()
    }

    /// Obtains an authentication path to the value specified in the tree.
    /// Returns `None` if there is no available authentication path to the
    /// specified value.
    pub fn authentication_path(&self, value: &H::Digest) -> Option<(usize, Vec<H::Digest>)> {
        unimplemented!()
    }

    /// Marks the specified tree state value as a value we're no longer
    /// interested in maintaining a witness for. Returns true if successful and
    /// false if the value is not a known witness.
    pub fn remove_witness(&mut self, value: &H::Digest) -> bool {
        unimplemented!()
    }

    /// Marks the current tree state as a checkpoint if it is not already a
    /// checkpoint.
    pub fn checkpoint(&mut self) {
        unimplemented!()
    }

    /// Rewinds the tree state to the previous checkpoint. This function will
    /// fail and return false if there is no previous checkpoint or in the event
    /// witness data would be destroyed in the process.
    pub fn rewind(&mut self) -> bool {
        unimplemented!()
    }

    /// Removes the oldest checkpoint. Returns true if successful and false if
    /// there are no checkpoints.
    pub fn pop_checkpoint(&mut self) -> bool {
        unimplemented!()
    }

    /// Start a recording of append operations performed on a tree.
    pub fn recording(&self) -> EfficientRecording<H> {
        unimplemented!()
    }

    /// Plays a recording of append operations back. Returns true if successful
    /// and false if the recording is incompatible with the current tree state.
    pub fn play(&mut self, recording: &EfficientRecording<H>) -> bool {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct EfficientRecording<H: TreeHasher> {
    something: H::Digest,
}

impl<H: TreeHasher> EfficientRecording<H> {
    pub fn append(&mut self, value: &H::Digest) -> bool {
        unimplemented!()
    }

    pub fn play(&mut self, recording: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct CombinedTree<H: TreeHasher> {
    inefficient: Tree<H>,
    efficient: EfficientTree<H>,
}

impl<H: TreeHasher> CombinedTree<H> {
    pub fn new(depth: usize) -> Self {
        CombinedTree {
            inefficient: Tree::new(depth),
            efficient: EfficientTree::new(depth),
        }
    }

    pub fn append(&mut self, value: &H::Digest) -> bool {
        let a = self.inefficient.append(value);
        let b = self.efficient.append(value);
        assert_eq!(a, b);
        a
    }

    /// Obtains the current root of this Merkle tree.
    pub fn root(&self) -> H::Digest {
        let a = self.inefficient.root();
        let b = self.efficient.root();
        assert_eq!(a, b);
        a
    }

    /// Marks the current tree state leaf as a value that we're interested in
    /// witnessing. Returns true if successful and false if the tree is empty.
    pub fn witness(&mut self) -> bool {
        let a = self.inefficient.witness();
        let b = self.efficient.witness();
        assert_eq!(a, b);
        a
    }

    /// Obtains an authentication path to the value specified in the tree.
    /// Returns `None` if there is no available authentication path to the
    /// specified value.
    pub fn authentication_path(&self, value: &H::Digest) -> Option<(usize, Vec<H::Digest>)> {
        let a = self.inefficient.authentication_path(value);
        let b = self.efficient.authentication_path(value);
        assert_eq!(a, b);
        a
    }

    /// Marks the specified tree state value as a value we're no longer
    /// interested in maintaining a witness for. Returns true if successful and
    /// false if the value is not a known witness.
    pub fn remove_witness(&mut self, value: &H::Digest) -> bool {
        let a = self.inefficient.remove_witness(value);
        let b = self.efficient.remove_witness(value);
        assert_eq!(a, b);
        a
    }

    /// Marks the current tree state as a checkpoint if it is not already a
    /// checkpoint.
    pub fn checkpoint(&mut self) {
        self.inefficient.checkpoint();
        self.efficient.checkpoint();
    }

    /// Rewinds the tree state to the previous checkpoint. This function will
    /// fail and return false if there is no previous checkpoint or in the event
    /// witness data would be destroyed in the process.
    pub fn rewind(&mut self) -> bool {
        let a = self.inefficient.rewind();
        let b = self.efficient.rewind();
        assert_eq!(a, b);
        a
    }

    /// Removes the oldest checkpoint. Returns true if successful and false if
    /// there are no checkpoints.
    pub fn pop_checkpoint(&mut self) -> bool {
        let a = self.inefficient.pop_checkpoint();
        let b = self.efficient.pop_checkpoint();
        assert_eq!(a, b);
        a
    }

    /// Start a recording of append operations performed on a tree.
    pub fn recording(&self) -> CombinedRecording<H> {
        CombinedRecording {
            inefficient: self.inefficient.recording(),
            efficient: self.efficient.recording(),
        }
    }

    /// Plays a recording of append operations back. Returns true if successful
    /// and false if the recording is incompatible with the current tree state.
    pub fn play(&mut self, recording: &CombinedRecording<H>) -> bool {
        let a = self.inefficient.play(&recording.inefficient);
        let b = self.efficient.play(&recording.efficient);
        assert_eq!(a, b);
        a
    }
}

#[derive(Clone)]
pub struct CombinedRecording<H: TreeHasher> {
    inefficient: Recording<H>,
    efficient: EfficientRecording<H>,
}

impl<H: TreeHasher> CombinedRecording<H> {
    pub fn append(&mut self, value: &H::Digest) -> bool {
        let a = self.inefficient.append(value);
        let b = self.efficient.append(value);
        assert_eq!(a, b);
        a
    }

    pub fn play(&mut self, recording: &Self) -> bool {
        let a = self.inefficient.play(&recording.inefficient);
        let b = self.efficient.play(&recording.efficient);
        assert_eq!(a, b);
        a
    }
}

#[derive(Clone)]
pub struct Tree<H: TreeHasher> {
    leaves: Vec<H::Digest>,
    current_position: usize,
    witnesses: Vec<(usize, H::Digest)>,
    checkpoints: Vec<usize>,
    depth: usize,
}

impl<H: TreeHasher> Tree<H> {
    /// Creates a new, empty binary tree of specified depth.
    ///
    /// # Panics
    ///
    /// Panics if the specified depth is zero.
    pub fn new(depth: usize) -> Self {
        if depth == 0 {
            panic!("invalid depth for incremental merkle tree");
        }

        Tree {
            leaves: vec![H::empty_leaf(); 1 << depth],
            current_position: 0,
            witnesses: vec![],
            checkpoints: vec![],
            depth,
        }
    }

    /// Appends a new value to the tree at the next available slot. Returns true
    /// if successful and false if the tree is full.
    pub fn append(&mut self, value: &H::Digest) -> bool {
        if self.current_position == (1 << self.depth) {
            false
        } else {
            self.leaves[self.current_position] = value.clone();
            self.current_position += 1;
            true
        }
    }

    /// Obtains the current root of this Merkle tree.
    pub fn root(&self) -> H::Digest {
        lazy_root::<H>(self.leaves.clone())
    }

    /// Marks the current tree state leaf as a value that we're interested in
    /// witnessing. Returns true if successful and false if the tree is empty.
    pub fn witness(&mut self) -> bool {
        if self.current_position == 0 {
            return false;
        } else {
            let value = self.leaves[self.current_position - 1].clone();
            self.witnesses.push((self.current_position - 1, value));
            true
        }
    }

    /// Obtains an authentication path to the value specified in the tree.
    /// Returns `None` if there is no available authentication path to the
    /// specified value.
    pub fn authentication_path(&self, value: &H::Digest) -> Option<(usize, Vec<H::Digest>)> {
        self.witnesses
            .iter()
            .find(|witness| witness.1 == *value)
            .map(|&(pos, _)| {
                let mut path = vec![];

                let mut index = pos;
                for bit in 0..self.depth {
                    index ^= 1 << bit;
                    path.push(lazy_root::<H>(self.leaves[index..][0..(1 << bit)].to_vec()));
                    index &= usize::MAX << (bit + 1);
                }

                (pos, path)
            })
    }

    /// Marks the specified tree state value as a value we're no longer
    /// interested in maintaining a witness for. Returns true if successful and
    /// false if the value is not a known witness.
    pub fn remove_witness(&mut self, value: &H::Digest) -> bool {
        if let Some((position, _)) = self
            .witnesses
            .iter()
            .enumerate()
            .find(|witness| (witness.1).1 == *value)
        {
            self.witnesses.remove(position);

            true
        } else {
            false
        }
    }

    /// Marks the current tree state as a checkpoint if it is not already a
    /// checkpoint.
    pub fn checkpoint(&mut self) {
        self.checkpoints.push(self.current_position);
    }

    /// Rewinds the tree state to the previous checkpoint. This function will
    /// fail and return false if there is no previous checkpoint or in the event
    /// witness data would be destroyed in the process.
    pub fn rewind(&mut self) -> bool {
        if let Some(checkpoint) = self.checkpoints.pop() {
            if self.witnesses.iter().any(|&(pos, _)| pos >= checkpoint) {
                self.checkpoints.push(checkpoint);
                return false;
            }

            self.current_position = checkpoint;
            if checkpoint != (1 << self.depth) {
                self.leaves[checkpoint..].fill(H::empty_leaf());
            }

            true
        } else {
            false
        }
    }

    /// Removes the oldest checkpoint. Returns true if successful and false if
    /// there are no checkpoints.
    pub fn pop_checkpoint(&mut self) -> bool {
        if self.checkpoints.is_empty() {
            false
        } else {
            self.checkpoints.remove(0);
            true
        }
    }

    /// Start a recording of append operations performed on a tree.
    pub fn recording(&self) -> Recording<H> {
        Recording {
            start_position: self.current_position,
            current_position: self.current_position,
            depth: self.depth,
            appends: vec![],
        }
    }

    /// Plays a recording of append operations back. Returns true if successful
    /// and false if the recording is incompatible with the current tree state.
    pub fn play(&mut self, recording: &Recording<H>) -> bool {
        if recording.start_position == self.current_position && self.depth == recording.depth {
            for val in recording.appends.iter() {
                self.append(val);
            }
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Recording<H: TreeHasher> {
    start_position: usize,
    current_position: usize,
    depth: usize,
    appends: Vec<H::Digest>,
}

impl<H: TreeHasher> Recording<H> {
    /// Appends a new value to the tree at the next available slot. Returns true
    /// if successful and false if the tree is full.
    pub fn append(&mut self, value: &H::Digest) -> bool {
        if self.current_position == (1 << self.depth) {
            false
        } else {
            self.appends.push(value.clone());
            self.current_position += 1;

            true
        }
    }

    /// Plays a recording of append operations back. Returns true if successful
    /// and false if the provided recording is incompatible with `Self`.
    pub fn play(&mut self, recording: &Self) -> bool {
        if self.current_position == recording.start_position && self.depth == recording.depth {
            self.appends.extend_from_slice(&recording.appends);
            self.current_position = recording.current_position;
            true
        } else {
            false
        }
    }
}

fn lazy_root<H: TreeHasher>(mut leaves: Vec<H::Digest>) -> H::Digest {
    while leaves.len() != 1 {
        leaves = leaves
            .iter()
            .enumerate()
            .filter(|(i, _)| (i % 2) == 0)
            .map(|(_, a)| a)
            .zip(
                leaves
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (i % 2) == 1)
                    .map(|(_, b)| b),
            )
            .map(|(a, b)| H::combine(a, b))
            .collect();
    }

    leaves[0].clone()
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::*;
    use std::hash::Hasher;
    use std::hash::SipHasher as Hash;

    impl TreeHasher for Hash {
        type Digest = u64;

        fn empty_leaf() -> Self::Digest {
            0
        }
        fn combine(a: &Self::Digest, b: &Self::Digest) -> Self::Digest {
            let mut hasher = Hash::new();
            hasher.write_u64(*a);
            hasher.write_u64(*b);
            hasher.finish()
        }
    }

    fn compute_root_from_auth_path<H: TreeHasher>(
        value: H::Digest,
        position: usize,
        path: &[H::Digest],
    ) -> H::Digest {
        let mut cur = value;
        for (i, v) in path
            .iter()
            .enumerate()
            .map(|(i, v)| (((position >> i) & 1) == 1, v))
        {
            if i {
                cur = H::combine(v, &cur);
            } else {
                cur = H::combine(&cur, v);
            }
        }
        cur
    }

    #[test]
    fn test_compute_root_from_auth_path() {
        let expected = Hash::combine(
            &Hash::combine(&Hash::combine(&0, &1), &Hash::combine(&2, &3)),
            &Hash::combine(&Hash::combine(&4, &5), &Hash::combine(&6, &7)),
        );

        assert_eq!(
            compute_root_from_auth_path::<Hash>(
                0,
                0,
                &[
                    1,
                    Hash::combine(&2, &3),
                    Hash::combine(&Hash::combine(&4, &5), &Hash::combine(&6, &7))
                ]
            ),
            expected
        );

        assert_eq!(
            compute_root_from_auth_path::<Hash>(
                4,
                4,
                &[
                    5,
                    Hash::combine(&6, &7),
                    Hash::combine(&Hash::combine(&0, &1), &Hash::combine(&2, &3))
                ]
            ),
            expected
        );
    }

    #[test]
    fn correct_empty_root() {
        const DEPTH: usize = 5;
        let mut expected = 0u64;
        for _ in 0..DEPTH {
            expected = Hash::combine(&expected, &expected);
        }

        let tree = Tree::<Hash>::new(DEPTH);
        assert_eq!(tree.root(), expected);
    }

    #[test]
    fn correct_root() {
        const DEPTH: usize = 3;
        let values: Vec<u64> = (0..(1 << DEPTH)).collect();

        let mut tree = Tree::<Hash>::new(DEPTH);
        for value in values.iter() {
            assert!(tree.append(value));
        }
        assert!(!tree.append(&0));

        let expected = Hash::combine(
            &Hash::combine(&Hash::combine(&0, &1), &Hash::combine(&2, &3)),
            &Hash::combine(&Hash::combine(&4, &5), &Hash::combine(&6, &7)),
        );

        assert_eq!(tree.root(), expected);
    }

    #[test]
    fn correct_auth_path() {
        const DEPTH: usize = 3;
        let values: Vec<u64> = (0..(1 << DEPTH)).collect();

        let mut tree = Tree::<Hash>::new(DEPTH);
        for value in values.iter() {
            assert!(tree.append(value));
            tree.witness();
        }
        assert!(!tree.append(&0));

        let expected = Hash::combine(
            &Hash::combine(&Hash::combine(&0, &1), &Hash::combine(&2, &3)),
            &Hash::combine(&Hash::combine(&4, &5), &Hash::combine(&6, &7)),
        );

        assert_eq!(tree.root(), expected);

        for i in 0..(1 << DEPTH) {
            println!("value: {}", i);
            let (position, path) = tree.authentication_path(&i).unwrap();
            assert_eq!(
                compute_root_from_auth_path::<Hash>(i, position, &path),
                expected
            );
        }
    }

    use proptest::prelude::*;

    #[derive(Clone, Debug)]
    enum Operation {
        Append(u64),
        Witness,
        Unwitness(u64),
        Checkpoint,
        Rewind,
        PopCheckpoint,
        Authpath(u64),
    }

    use Operation::*;

    prop_compose! {
        fn arb_operation()
                    (
                        opid in (0..7),
                        item in (0..32u64),
                    )
                    -> Operation
        {
            match opid {
                0 => Append(item),
                1 => Witness,
                2 => Unwitness(item),
                3 => Checkpoint,
                4 => Rewind,
                5 => PopCheckpoint,
                6 => Authpath(item),
                _ => unimplemented!()
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100000))]
        #[test]
        fn do_stuff(ops in proptest::collection::vec(arb_operation(), 1..100)) {
            const DEPTH: usize = 4;
            let mut tree = CombinedTree::<Hash>::new(DEPTH);

            //let mut prevtrees = vec![];

            let mut tree_size = 0;
            let mut tree_values = vec![];
            let mut tree_checkpoints = vec![];
            let mut tree_witnesses: Vec<(usize, u64)> = vec![];

            for op in ops {
                prop_assert_eq!(tree_size, tree_values.len());
                match op {
                    Append(value) => {
                        //prevtrees.push((tree.clone(), tree.recording()));
                        if tree.append(&value) {
                            prop_assert!(tree_size < (1 << DEPTH));
                            tree_size += 1;
                            tree_values.push(value);

                            /*
                            for &mut (_, ref mut recording) in &mut prevtrees {
                                prop_assert!(recording.append(&value));
                            }
                            */
                        } else {
                            prop_assert!(tree_size == (1 << DEPTH));
                        }
                    }
                    Witness => {
                        if tree.witness() {
                            prop_assert!(tree_size != 0);
                            tree_witnesses.push((tree_size - 1, *tree_values.last().unwrap()));
                        } else {
                            prop_assert!(tree_size == 0);
                        }
                    }
                    Unwitness(value) => {
                        if tree.remove_witness(&value) {
                            if let Some((i, _)) = tree_witnesses.iter().enumerate().find(|v| (v.1).1 == value) {
                                tree_witnesses.remove(i);
                            } else {
                                panic!("witness should not have been removed");
                            }
                        } else {
                            if tree_witnesses.iter().find(|v| v.1 == value).is_some() {
                                panic!("witness should have been removed");
                            }
                        }
                    }
                    Checkpoint => {
                        tree_checkpoints.push(tree_size);
                        tree.checkpoint();
                    }
                    Rewind => {
                        // prevtrees.truncate(0);

                        if tree.rewind() {
                            prop_assert!(tree_checkpoints.len() > 0);
                            let checkpoint_location = tree_checkpoints.pop().unwrap();
                            for &(index, _) in tree_witnesses.iter() {
                                // index is the index in tree_values
                                // checkpoint_location is the size of the tree
                                // at the time of the checkpoint
                                // index should always be strictly smaller or
                                // else a witness would be erased!
                                prop_assert!(index < checkpoint_location);
                            }
                            tree_values.truncate(checkpoint_location);
                            tree_size = checkpoint_location;
                        } else {
                            if tree_checkpoints.len() != 0 {
                                let checkpoint_location = *tree_checkpoints.last().unwrap();
                                prop_assert!(tree_witnesses.iter().any(|&(index, _)| index >= checkpoint_location));
                            }
                        }
                    }
                    PopCheckpoint => {
                        if tree.pop_checkpoint() {
                            prop_assert!(tree_checkpoints.len() > 0);
                            tree_checkpoints.remove(0);
                        } else {
                            prop_assert!(tree_checkpoints.len() == 0);
                        }
                    }
                    Authpath(value) => {
                        if let Some((position, path)) = tree.authentication_path(&value) {
                            // must be the case that value was a witness
                            prop_assert!(tree_witnesses.iter().any(|&(_, witness)| witness == value));

                            let mut extended_tree_values = tree_values.clone();
                            extended_tree_values.resize(1 << DEPTH, Hash::empty_leaf());
                            let expected_root = lazy_root::<Hash>(extended_tree_values);

                            let tree_root = tree.root();
                            prop_assert_eq!(tree_root, expected_root);

                            prop_assert_eq!(
                                compute_root_from_auth_path::<Hash>(value, position, &path),
                                expected_root
                            );
                        } else {
                            // must be the case that value wasn't a witness
                            for &(_, witness) in tree_witnesses.iter() {
                                prop_assert!(witness != value);
                            }
                        }
                    }
                }
            }

            // for (mut other_tree, other_recording) in prevtrees {
            //     prop_assert!(other_tree.play(&other_recording));
            //     prop_assert_eq!(tree.root(), other_tree.root());
            // }
        }
    }
}
