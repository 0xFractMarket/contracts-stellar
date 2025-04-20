#![no_std]

use soroban_sdk::{contractimpl, symbol_short, Address, Env, Symbol};

pub struct DividendVault;

#[contractimpl]
impl DividendVault {
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let contract = env.current_contract_address();
        let token = soroban_sdk::native_contracts::token::Client::new(
            &env,
            &soroban_sdk::native_token::native_token_id(),
        );
        token.transfer(&from, &contract, &amount);

        let prev_total: i128 = env.storage().get(&symbol_short!("total_dividends")).unwrap_or(0);
        env.storage().set(&symbol_short!("total_dividends"), &(prev_total + amount));
    }

    pub fn claim(env: Env, user: Address, property_contract: Address) {
        user.require_auth();

        let claimed_key = Symbol::short(format!("claimed_{}", user.to_string()).as_str());
        let already_claimed: bool = env.storage().get(&claimed_key).unwrap_or(false);
        if already_claimed {
            panic!("Ya reclamado");
        }

        let prop = FractionalPropertyClient::new(&env, &property_contract);
        let balance = prop.get_balance(user.clone());

        let total_supply: u32 = env.storage().get(&symbol_short!("total_shares")).expect("Debe setearse");
        let total_dividends: i128 = env.storage().get(&symbol_short!("total_dividends")).unwrap_or(0);

        let payout = (total_dividends * (balance as i128)) / (total_supply as i128);

        let token = soroban_sdk::native_contracts::token::Client::new(
            &env,
            &soroban_sdk::native_token::native_token_id(),
        );
        token.transfer(&env.current_contract_address(), &user, &payout);

        env.storage().set(&claimed_key, &true);
    }

    pub fn set_total_shares(env: Env, total: u32) {
        env.storage().set(&symbol_short!("total_shares"), &total);
    }
}

mod external {
    use soroban_sdk::{contractclient, Address, Env};

    #[contractclient(name = "FractionalPropertyClient")]
    pub trait FractionalPropertyTrait {
        fn get_balance(env: Env, user: Address) -> u32;
    }
}
