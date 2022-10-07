use libafl::{inputs::Input, mutators::Mutator, prelude::MutatorsTuple, state::HasRand};

use core::marker::PhantomData;

pub struct StdFeedbackMutators<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand,
{
    mutations: MT,
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
    S: HasRand,
{
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
