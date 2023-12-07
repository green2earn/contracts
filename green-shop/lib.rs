#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod green_shop {
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    };

    /// A token ID.
    pub type TokenId = u32;

    /// Event emitted when a token mint occurs.
    #[ink(event)]
    pub struct GreenShopMinted {
        to: AccountId,
        value: Balance,
        token_id: TokenId,
    }

    #[ink(storage)]
    pub struct GreenShopContractCaller {
        green_token: AccountId,
        green_nft: AccountId,
        token_id: TokenId,
    }

    impl GreenShopContractCaller {
        #[ink(constructor)]
        pub fn new(green_token: AccountId, green_nft: AccountId) -> Self {
            Self {
                green_token,
                green_nft,
                token_id: 0,
            }
        }

        #[ink(message)]
        pub fn mint_token(&mut self, to: AccountId, value: Balance) -> Result<(), ink::LangError> {
            //Mint token
            let selector: Selector = Selector::new(ink::selector_bytes!("mint"));
            let _ = build_call::<DefaultEnvironment>()
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

            // Mint NFT
            self.token_id += 1;
            let result = build_call::<DefaultEnvironment>()
                .call(self.green_nft)
                // .gas_limit(5000)
                // .transferred_value(0)
                // .call_flags(CallFlags::default().set_tail_call(true))
                .exec_input(
                    ExecutionInput::new(selector)
                        .push_arg(self.token_id)
                        .push_arg(to),
                )
                .returns::<()>()
                .try_invoke();

            self.env().emit_event(GreenShopMinted {
                to,
                value,
                token_id: self.token_id,
            });
            result.unwrap()
        }
    }
}
