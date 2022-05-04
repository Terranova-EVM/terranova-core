module.exports = ({ wallets, refs, config, client }) => ({
  getCount: () => client.query("counter", { get_count: {} }),
  sendTx: (caller_evm_address, unsigned_tx, signer = wallets.validator) =>
    client.execute(
      signer,
      "terranova",
      {
        // FIXME: Use the correct types for LCDClient
        call_from_raw_ethereum_t_x: {
          caller_evm_address: caller_evm_address,
          unsigned_tx: unsigned_tx
        }
      }
    ),
});
