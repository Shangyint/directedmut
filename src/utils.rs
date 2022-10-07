#[cfg(test)]
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct XkcdRand {
    val: u64,
}

#[cfg(test)]
impl libafl::prelude::Rand for XkcdRand {
    fn set_seed(&mut self, val: u64) {
        self.val = val;
    }

    fn next(&mut self) -> u64 {
        self.val
    }
}

/// A test rng that will return the same value (chose by fair dice roll) for testing.
#[cfg(test)]
impl XkcdRand {
    /// Creates a new [`XkCDRand`] with the rand of 4, [chosen by fair dice roll, guaranteed to be random](https://xkcd.com/221/).
    /// Will always return this seed.
    #[must_use]
    pub fn new() -> Self {
        Self::with_seed(4)
    }

    /// Creates a new [`XkcdRand`] with the given seed. Will always return this seed.
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self { val: seed }
    }
}
