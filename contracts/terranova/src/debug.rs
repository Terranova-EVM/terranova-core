#[cfg(all(target_arch = "wasm32", not(feature = "no-logs")))]
#[macro_export]
macro_rules! debug_print {
//    ($( $args:expr ),*) => { solana_program::msg!( $( $args ),* ) }
    ($( $args:expr ),*) => {}
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "no-logs")))]
#[macro_export]
macro_rules! debug_print {
    ($( $args:expr ),*) => { log::debug!( $( $args ),* ) }
}

#[cfg(feature = "no-logs")]
#[macro_export]
macro_rules! debug_print {
    ($( $args:expr ),*) => {}
}
