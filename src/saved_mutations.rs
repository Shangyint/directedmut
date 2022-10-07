use std::{marker::PhantomData, cell::RefCell};

use libafl::{prelude::{MutatorsTuple, Input}, state::HasRand};
use serde::{Serialize, de::DeserializeOwned};

pub struct MutationVector<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand
{
    mutations: Vec<RefCell<MT>>,
    phantom: PhantomData<(I, S)>,
}


pub trait SavedMutation<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand
{

}

impl<I, MT, S> SavedMutation<I, MT, S> for MutationVector<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand
{

}


pub trait HasSavedMutation<I, MT, S>
where
    I: Input,
    MT: MutatorsTuple<I, S>,
    S: HasRand
{
    type SavedMutation: SavedMutation<I, MT, S>;

    fn saved_mut(&self) -> &Self::SavedMutation;
}