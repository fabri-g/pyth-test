use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::Value;
use near_sdk::{
    env, 
    near_bindgen, 
    AccountId,
    PanicOnDefault,
    NearToken,
    Promise,
    PromiseError,
    PromiseResult,
    ext_contract,
    Gas
};

#[ext_contract(pyth_oracle)]
pub trait PythOracle {
    fn update_price_feeds(&mut self, data: String);
    fn get_price(&self, price_identifier: String) -> Option<u128>;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PythTest {
    is_initialized: bool,
}

#[near_bindgen]
impl PythTest {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            is_initialized: true,
        }
    }

    pub fn update_price_feeds(&self, data: String) -> Promise {
        let account_id = "pyth-oracle.testnet".parse::<AccountId>().unwrap();
        let promise = pyth_oracle::ext(account_id)
            .with_attached_deposit(NearToken::from_near(1))
            .with_static_gas(Gas::from_tgas(10))
            .update_price_feeds(data.clone());
        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(10))
                .update_price_feeds_callback()
        )
    }

    pub fn get_price(&self, price_identifier: String) -> Promise {
        let account_id = "pyth-oracle.testnet".parse::<AccountId>().unwrap();
        let promise = pyth_oracle::ext(account_id)
            .with_attached_deposit(NearToken::from_near(0))
            .with_static_gas(Gas::from_tgas(20))
            .get_price(price_identifier);

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(10))
                .get_price_callback()
        )
    }

    #[private]
    pub fn update_price_feeds_callback(&self, #[callback_result] call_result: Result<(), PromiseError>) {
        match call_result {
            Ok(_) => env::log_str("Price feeds updated successfully."),
            Err(e) => env::panic_str(&format!("Failed to update price feeds: {:?}", e)),
        }
    }

    #[private]
    pub fn get_price_callback(&self) -> Option<u128> {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Expected exactly one promise result."
        );
        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                let json_result: Value = near_sdk::serde_json::from_slice(&result).expect("Failed to deserialize JSON result");
                if let Some(price_str) = json_result.get("price").and_then(|v| v.as_str()) {
                    price_str.parse::<u128>().ok()
                } else {
                    None
                }
            },
            _ => None, 
        }
    }
}
