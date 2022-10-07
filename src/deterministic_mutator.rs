use libafl::{
    prelude::{HasBytesVec, Input, MutationResult, Mutator, Rand},
    state::HasRand,
    Error,
};

pub struct DeterministicBitFlipMutator {
    bit: u8,
    byte: usize,
}

impl<I, S> Mutator<I, S> for DeterministicBitFlipMutator
where
    I: Input + HasBytesVec,
{
    fn mutate(
        &mut self,
        _state: &mut S,
        input: &mut I,
        _stage_idx: i32,
    ) -> Result<MutationResult, Error> {
        if input.bytes().is_empty() {
            Ok(MutationResult::Skipped)
        } else {
            let bit = self.bit;
            let byte = &mut input.bytes_mut()[self.byte];
            *byte ^= bit;
            Ok(MutationResult::Mutated)
        }
    }
}

pub struct DeterministicByteFlipMutator {
    byte: usize,
}

impl<I, S> Mutator<I, S> for DeterministicByteFlipMutator
where
    I: Input + HasBytesVec,
{
    fn mutate(
        &mut self,
        _state: &mut S,
        input: &mut I,
        _stage_idx: i32,
    ) -> Result<MutationResult, Error> {
        // what if this one fails?
        if input.bytes().is_empty() {
            Ok(MutationResult::Skipped)
        } else {
            *(&mut input.bytes_mut()[self.byte]) ^= 0xff;
            Ok(MutationResult::Mutated)
        }
    }
}
