use std::{cell::RefCell, marker::PhantomData};

use libafl::{
    prelude::{Input, MutatorsTuple},
    state::HasRand,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct MutationVector<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand,
{
    mutations: Vec<RefCell<MT>>,
    phantom: PhantomData<(I, S)>,
}

pub trait SavedMutation<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand,
{
}

impl<I, MT, S> SavedMutation<I, MT, S> for MutationVector<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand,
{
}

pub trait HasSavedMutation<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand,
{
    type SavedMutation: SavedMutation<I, MT, S>;

    fn saved_mut(&self) -> &Self::SavedMutation;
}
