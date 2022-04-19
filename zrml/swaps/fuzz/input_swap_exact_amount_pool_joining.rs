#![no_main]

use libfuzzer_sys::fuzz_target;
use zrml_swaps::mock::{ExtBuilder, Origin, Swaps};

mod data_structs;
use data_structs::{SwapExactAmountData, POOL_LIQUIDITY};
mod helper_functions;
use helper_functions::asset;

use orml_traits::MultiCurrency;
use zeitgeist_primitives::{traits::Swaps as SwapsTrait, types::ScoringRule};

use rand::{seq::SliceRandom, Rng};
use sp_runtime::DispatchError;
use zrml_swaps::mock::Shares;

fuzz_target!(|data: SwapExactAmountData| {
    let mut ext = ExtBuilder::default().build();
    let _ = ext.execute_with(|| {
        // ensure that the account origin has a sufficient balance
        // use orml_traits::MultiCurrency; required for this
        for a in &data.pool_creation.assets {
            let _ = Shares::deposit(asset(*a), &data.pool_creation.origin, POOL_LIQUIDITY);
        }
        match Swaps::create_pool(
            data.pool_creation.origin,
            data.pool_creation.assets.clone().into_iter().map(asset).collect(),
            Some(data.pool_creation.base_asset).map(asset),
            data.pool_creation.market_id,
            ScoringRule::CPMM,
            Some(data.pool_creation.swap_fee),
            Some(data.pool_creation.weights),
        ) {
            Ok(pool_id) => {
                let mut rng = rand::thread_rng();
                // init with random data
                let mut asset_in = asset(data.asset_in);
                let mut asset_out = asset(data.asset_out);
                if let Some(a) = data.pool_creation.assets.choose(&mut rng) {
                    asset_in = asset(*a);
                }
                if let Some(a) = data.pool_creation.assets.choose(&mut rng) {
                    asset_out = asset(*a);
                }
                let asset_amount_in = rng.gen_range(0..100);
                let asset_amount_out = rng.gen_range(0..100);
                // provide a sufficient balance
                let _ = Shares::deposit(asset_in, &data.origin, asset_amount_in);
                let _ = Shares::deposit(asset_out, &data.origin, asset_amount_out);

                let pool_account_id = Swaps::pool_account_id(pool_id);
                let pool_balance = rng.gen_range(100000..100000000);
                let _ = Shares::deposit(asset_in, &pool_account_id, pool_balance);
                let _ = Shares::deposit(asset_out, &pool_account_id, pool_balance);
                let result = Swaps::swap_exact_amount_in(
                    Origin::signed(data.origin),
                    pool_id,
                    asset_in,
                    asset_amount_in,
                    asset_out,
                    asset_amount_out,
                    data.max_price,
                );
                if let Err(e) = result {
                    if let DispatchError::Module { index: _, error: _, message } = e.error {
                        println!("Pallet Error: {:?}", message);
                    } else {
                        println!("Other Error: {:?}", e);
                    }
                } else {
                    println!("Success {:?}", result);
                }
            }
            Err(e) => panic!(
                "There needs to be a valid pool creation! This Swaps::create_pool call returns an \
                 error, but should be ok. Error: {:?}",
                e
            ),
        }
    });
    let _ = ext.commit_all();
});
