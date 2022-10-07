use std::marker::PhantomData;

use crate::saved_mutations::{HasSavedMutation, MutationVector, SavedMutation};
use libafl::{
    prelude::{ClientPerfMonitor, Corpus, Feedback, Input, MutatorsTuple, Rand},
    state::{HasClientPerfMonitor, HasRand, StdState},
};

pub struct SavedMutationStdState<C, I, R, SC, MT, S, SM>
where
    C: Corpus<I>,
    I: Input,
    R: Rand,
    SC: Corpus<I>,
    S: HasRand,
    MT: MutatorsTuple<I, S>,
    SM: SavedMutation<I, MT, S>,
{
    state: StdState<C, I, R, SC>,
    saved_mutations: SM,
    phantom: PhantomData<(MT, S)>,
}

impl<C, I, R, SC, MT, S, SM> HasSavedMutation<I, MT, S>
    for SavedMutationStdState<C, I, R, SC, MT, S, SM>
where
    C: Corpus<I>,
    I: Input,
    R: Rand,
    SC: Corpus<I>,
    S: HasRand,
    MT: MutatorsTuple<I, S>,
    SM: SavedMutation<I, MT, S>,
{
    type SavedMutation = SM;

    fn saved_mut(&self) -> &Self::SavedMutation {
        &self.saved_mutations
    }
}

impl<C, I, R, SC, MT, S, SM> HasRand for SavedMutationStdState<C, I, R, SC, MT, S, SM>
where
    C: Corpus<I>,
    I: Input,
    R: Rand,
    SC: Corpus<I>,
    S: HasRand,
    MT: MutatorsTuple<I, S>,
    SM: SavedMutation<I, MT, S>,
{
    type Rand = R;

    #[inline]
    fn rand(&self) -> &Self::Rand {
        &self.state.rand()
    }

    #[inline]
    fn rand_mut(&mut self) -> &mut Self::Rand {
        self.state.rand_mut()
    }
}

impl<C, I, R, SC, MT, S, SM> SavedMutationStdState<C, I, R, SC, MT, S, SM>
where
    C: Corpus<I>,
    I: Input,
    R: Rand,
    SC: Corpus<I>,
    S: HasRand,
    MT: MutatorsTuple<I, S>,
    SM: SavedMutation<I, MT, S>,
{
    pub fn new(std_state: StdState<C, I, R, SC>, saved_mutations: SM) -> Self {
        Self {
            state: std_state,
            saved_mutations,
            phantom: PhantomData,
        }
    }

    pub fn new_stdstate<F, O>(
        rand: R,
        corpus: C,
        new_corpus: SC,
        feedback: &mut F,
        objective: &mut O,
        saved_mutations: SM,
    ) -> Self
    where
        F: Feedback<I, StdState<C, I, R, SC>>,
        O: Feedback<I, StdState<C, I, R, SC>>,
    {
        Self {
            state: StdState::new(rand, corpus, new_corpus, feedback, objective).unwrap(),
            saved_mutations,
            phantom: PhantomData,
        }
    }
}
