use cosmwasm_std::Addr;

pub fn token_mint_dummy() -> Addr {
    Addr::unchecked("NOVA_token_mint_dummy")
}

pub fn chain_id_dummy() -> u64 {
    789_789_789_u64
}