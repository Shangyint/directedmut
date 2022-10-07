use libafl::{
    inputs::Input,
    mutators::Mutator,
    prelude::{ComposedByMutations, Corpus, MutationResult, MutatorsTuple, Rand, ScheduledMutator},
    state::{HasCorpus, HasMetadata, HasRand, HasSolutions},
    Error,
};

use core::marker::PhantomData;
use std::borrow::BorrowMut;

pub struct StdFeedbackMutators<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand,
{
    mutations: MT,
    max_stack_pow: u64,
    phantom: PhantomData<(I, S)>,
}

pub trait FeedbackMutator<I, S>: Mutator<I, S>
where
    I: Input,
{
}

impl<I, MT, S> StdFeedbackMutators<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand + HasCorpus<I> + HasSolutions<I>,
{
    pub fn new(mutations: MT) -> Self {
        Self {
            mutations,
            max_stack_pow: 7,
            phantom: PhantomData,
        }
    }

    /// Create a new [`StdScheduledMutator`] instance specifying mutations and the maximun number of iterations
    pub fn with_max_stack_pow(mutations: MT, max_stack_pow: u64) -> Self {
        Self {
            mutations,
            max_stack_pow,
            phantom: PhantomData,
        }
    }
}

impl<I, MT, S> ComposedByMutations<I, MT, S> for StdFeedbackMutators<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand + HasCorpus<I> + HasSolutions<I>,
{
    /// Get the mutations
    #[inline]
    fn mutations(&self) -> &MT {
        &self.mutations
    }

    // Get the mutations (mutable)
    #[inline]
    fn mutations_mut(&mut self) -> &mut MT {
        &mut self.mutations
    }
}

impl<I, MT, S> ScheduledMutator<I, MT, S> for StdFeedbackMutators<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand + HasCorpus<I> + HasSolutions<I>,
{
    fn iterations(&self, state: &mut S, _: &I) -> u64 {
        1 << (1 + state.rand_mut().below(self.max_stack_pow))
    }

    // TODO: Change
    fn schedule(&self, state: &mut S, _: &I) -> usize {
        debug_assert!(!self.mutations().is_empty());
        state.rand_mut().below(self.mutations().len() as u64) as usize
    }

    // TODO: Change
    fn scheduled_mutate(
        &mut self,
        state: &mut S,
        input: &mut I,
        stage_idx: i32,
    ) -> Result<MutationResult, Error> {
        Ok(MutationResult::Skipped)
    }
}

impl<I, MT, S> Mutator<I, S> for StdFeedbackMutators<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand + HasCorpus<I> + HasSolutions<I>,
{
    fn mutate(
        &mut self,
        state: &mut S,
        input: &mut I,
        stage_idx: i32,
    ) -> Result<MutationResult, Error> {
        self.scheduled_mutate(state, input, stage_idx)
    }

    // TODO: change
    fn post_exec(
        &mut self,
        state: &mut S,
        _stage_idx: i32,
        _corpus_idx: Option<usize>,
    ) -> Result<(), Error> {
        if let Some(idx) = _corpus_idx {
            state.corpus().get(idx)?.borrow_mut().metadata_mut();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use libafl::{
        prelude::{BytesInput, Corpus, InMemoryCorpus, Testcase},
        state::StdState,
    };

    use crate::utils::XkcdRand;

    #[test]
    fn test_feedback_mutator() {
        let mut rand = XkcdRand::with_seed(5);
        let mut corpus: InMemoryCorpus<BytesInput> = InMemoryCorpus::new();
        corpus.add(Testcase::new(vec![b'a', b'b', b'c'])).unwrap();
        corpus.add(Testcase::new(vec![b'd', b'e', b'f'])).unwrap();

        let testcase = corpus.get(0).expect("Corpus contains no entries");
        let mut input = testcase.borrow_mut().load_input().unwrap().clone();

        let mut state = StdState::new(rand, corpus, InMemoryCorpus::new(), &mut (), &mut ());
    }
}
