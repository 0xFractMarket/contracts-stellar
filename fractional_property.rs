#![no_std]

use soroban_sdk::{contractimpl, symbol_short, Address, Env, Symbol, Map};

pub struct FractionalProperty;

#[contractimpl]
impl FractionalProperty {
    pub fn initialize(env: Env, admin: Address, total_shares: u32, price_per_share: i128) {
        admin.require_auth();
        env.storage().set(&symbol_short!("admin"), &admin);
        env.storage().set(&symbol_short!("total"), &total_shares);
        env.storage().set(&symbol_short!("price"), &price_per_share);
    }

    pub fn buy(env: Env, buyer: Address, shares: u32, payment: i128) {
        buyer.require_auth();

        let price: i128 = env.storage().get(&symbol_short!("price")).unwrap();
        let expected_payment = price * (shares as i128);
        if payment < expected_payment {
            panic!("Pago insuficiente");
        }

        let mut balances: Map<Address, u32> = env.storage().get(&symbol_short!("balances")).unwrap_or(Map::new(&env));
        let current = balances.get(buyer.clone()).unwrap_or(0);
        balances.set(buyer.clone(), current + shares);
        env.storage().set(&symbol_short!("balances"), &balances);
    }

    pub fn get_balance(env: Env, user: Address) -> u32 {
        let balances: Map<Address, u32> = env.storage().get(&symbol_short!("balances")).unwrap_or(Map::new(&env));
        balances.get(user).unwrap_or(0)
    }
}
