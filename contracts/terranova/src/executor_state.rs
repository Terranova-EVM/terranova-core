/// Represents the state of executor abstracted away from a backend.
#[derive(Serialize, Deserialize)]
pub struct ExecutorSubstate {
    metadata: ExecutorMetadata,
    parent: Option<Box<ExecutorSubstate>>,
    logs: Vec<Log>,
    transfers: Vec<Transfer>,
    block_hashes: RefCell<BTreeMap<U256, H256>>,
    accounts: BTreeMap<H160, ExecutorAccount>,
    balances: RefCell<BTreeMap<H160, U256>>,
    storages: BTreeMap<(H160, U256), U256>,
    // spl_balances: RefCell<BTreeMap<Pubkey, u64>>,
    // spl_decimals: RefCell<BTreeMap<Pubkey, u8>>,
    // spl_supply: RefCell<BTreeMap<Pubkey, u64>>,
    // spl_transfers: Vec<SplTransfer>,
    // spl_approves: Vec<SplApprove>,
    withdrawals: Vec<Withdraw>,
    // erc20_allowances: RefCell<BTreeMap<(H160, H160, H160, Pubkey), U256>>,
    deletes: BTreeSet<H160>,
    // query_account_cache: query::AccountCache,
}