#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod green_shop {
    /// We import the generated `ContractRef` of our other contract.
    ///
    /// Note that the other contract must have re-exported it (`pub use
    /// GreenTokenContract`) for us to have access to it.
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    };

    /// Event emitted when a token mint occurs.
    #[ink(event)]
    pub struct GreenShopMinted {
        to: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    pub struct GreenShopContractCaller {
        /// We specify that our contract will store a reference to the `OtherContract`.
        green_token: AccountId,
    }

    impl GreenShopContractCaller {
        /// In order to use the `OtherContract` we first need to **instantiate** it.
        ///
        /// To do this we will use the uploaded `code_hash` of `OtherContract`.
        #[ink(constructor)]
        pub fn new(green_token: AccountId) -> Self {
            // let green_token = GreenTokenRef::new(10000)
            //     .code_hash(green_token_contract_code_hash)
            //     .endowment(0)
            //     .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            //     .instantiate();

            Self { green_token }
        }

        #[ink(message)]
        pub fn mint_token(&mut self, to: AccountId, value: Balance) -> Result<(), ink::LangError> {
            //self.green_token.mint(to, value);
            let selector: Selector = Selector::new(ink::selector_bytes!("mint"));
            let result = build_call::<DefaultEnvironment>()
                .call(self.green_token)
                // .gas_limit(5000)
                // .transferred_value(0)
                // .call_flags(CallFlags::default().set_tail_call(true))
                .exec_input(
                    ExecutionInput::new(selector)
                        .push_arg(to)
                        .push_arg(Balance::from(value)),
                )
                .returns::<()>()
                .try_invoke();

            ink::env::debug_println!("got a call from {:?}", result);

            self.env().emit_event(GreenShopMinted { to, value });
            result.unwrap()
        }
    }
}
