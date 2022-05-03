#[macro_use]
mod debug;
#[macro_use]
mod error;

pub mod contract;
pub mod message;
pub mod account;
pub mod storage;
pub mod executor_state;
pub mod executor;
pub mod utils;
pub mod gasometer;
pub mod transaction;
pub mod config;
pub mod airdrop;

pub use crate::error::{ContractError};

#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "tracing")]
macro_rules! event {
    ($x:expr) => {
        use crate::tracing::Event::*;
        crate::tracing::with(|listener| listener.event($x));
    };
}

#[cfg(feature = "tracing")]
macro_rules! emit_exit {
    ($reason:expr) => {{
        let reason = $reason;
        event!(Exit {
            reason: &reason.into(),
            return_value: &Vec::new(),
        });
        reason
    }};
    ($return_value:expr, $reason:expr) => {{
        let reason = $reason;
        let return_value = $return_value;
        event!(Exit {
            reason: &reason,
            return_value: &return_value,
        });
        (return_value, reason)
    }};
}

#[cfg(not(feature = "tracing"))]
macro_rules! emit_exit {
    ($reason:expr) => {
        $reason
    };
    ($return_value:expr, $reason:expr) => {
        ($return_value, $reason)
    };
}

#[cfg(not(feature = "tracing"))]
macro_rules! event {
    ($x:expr) => {};
}

pub(crate) use emit_exit;
pub(crate) use event;