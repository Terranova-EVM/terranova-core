use std::convert::TryInto;

use evm::{U256, H160};

use crate::{storage::StorageInterface, executor_state::ExecutorState, transaction::UnsignedTransaction};

/// Not required for bytecode execution itself, but necessary if you want to measure opcode usage 
/// TODO: Flesh out the skeleton here
pub struct Gasometer {
    gas: u64,
}

impl Gasometer {
    pub fn new() -> Self {
        Self { gas: 0_u64 }
    }

    #[must_use]
    pub fn used_gas(&self) -> U256 {
        U256::from(self.gas)
    }

    pub fn record_iterative_overhead(&mut self) {

    }

    pub fn record_transaction_size(&mut self, trx: &UnsignedTransaction) {

    }

    pub fn record_evm_steps(&mut self, steps: u64) {

    }

    pub fn pad_evm_steps(&mut self, steps: u64) {

    }

    pub fn record_storage_write<B>(&mut self, state: &ExecutorState<B>, address: H160, key: U256)
    where
        B: StorageInterface
    {

    }

    pub fn record_deploy<B>(&mut self, state: &ExecutorState<B>, address: H160)
    where
        B: StorageInterface
    {

    }

    pub fn record_transfer<B>(&mut self, state: &ExecutorState<B>, target: H160, value: U256)
    where
        B: StorageInterface
    {

    }
}
