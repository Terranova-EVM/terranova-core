use cw_storage_plus::Map;
use evm::H160;

use crate::account::EthereumAccount;

pub const ACCOUNTS: Map<H160, EthereumAccount> = Map::new("accounts");