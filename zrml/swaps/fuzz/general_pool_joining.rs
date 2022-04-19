#![no_main]

use libfuzzer_sys::fuzz_target;

use zrml_swaps::mock::{ExtBuilder, Origin, Swaps};
mod data_structs;
use data_structs::{GeneralPoolData, POOL_LIQUIDITY};
use zeitgeist_primitives::{traits::Swaps as SwapsTrait, types::ScoringRule};
mod helper_functions;
use helper_functions::asset;

use zrml_swaps::mock::Shares;

use orml_traits::MultiCurrency;

use sp_runtime::DispatchError;

fuzz_target!(|data: GeneralPoolData| {
    let mut ext = ExtBuilder::default().build();
    let _ = ext.execute_with(|| {
        // ensure that the account origin has a sufficient balance
        // use orml_traits::MultiCurrency; required for this
        for a in &data.pool_creation.assets {
            // deposit balance of asset (could lead to multiple deposits of one asset)
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
                for (a, upper_bound_balance) in
                    data.pool_creation.assets.iter().zip(data.assets.clone())
                {
                    // ensure that pool_join account has an initial balance
                    let _ = Shares::deposit(asset(*a), &data.origin, upper_bound_balance);
                }
                // join a pool with a valid pool id
                let result = Swaps::pool_join(
                    Origin::signed(data.origin),
                    pool_id,
                    data.pool_amount,
                    data.assets,
                );

                match result {
                    Ok(r) => println!("Success {:?}", r),
                    Err(e) => {
                        if let DispatchError::Module { index: _, error: _, message } = e.error {
                            println!("Pallet Error: {:?}", message);
                        } else {
                            println!("Other Error: {:?}", e);
                        }
                    }
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
