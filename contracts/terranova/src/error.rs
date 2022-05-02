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
    NonceOverflow
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
