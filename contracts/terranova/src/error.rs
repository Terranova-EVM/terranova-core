use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Terranova EVM transfer failed due to insufficient funds in the caller account")]
    InsufficientFunds,

    #[error("Contract creation failed")]
    ContractCreationFailed,

    #[error("The unsigned transaction is invalid")]
    InvalidTransactionData,

    #[error("An account balance overflowed")]
    BalanceOverflow,

    #[error("An account nonce overflowed")]
    NonceOverflow,

    #[error("The collected chunks of the transaction do not match the provided hash")]
    InvalidTxChunks,

    #[error("One or more of the chunks for this transaction are still missing")]
    TxChunksMissing,

    #[error("The provided query is invalid because it tried to incur a state change")]
    QueryChangedState,
}

macro_rules! Err {
    ( $n:expr; $($args:expr),* ) => ({
        #[cfg(target_arch = "wasm32")]
        {}
        // solana_program::msg!("{}:{} : {}", file!(), line!(), &format!($($args),*));

        #[cfg(not(target_arch = "wasm32"))]
        log::error!("{}", &format!($($args),*));

        Err($n)
    });
}

macro_rules! E {
    ( $n:expr; $($args:expr),* ) => ({
        #[cfg(target_arch = "wasm32")]
        {}
        // solana_program::msg!("{}:{} : {}", file!(), line!(), &format!($($args),*));

        #[cfg(not(target_arch = "wasm32"))]
        log::error!("{}", &format!($($args),*));

        $n
    });
}
