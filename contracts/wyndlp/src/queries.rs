use cosmwasm_std::{Addr, QuerierWrapper, StdError, StdResult, Uint128};
use outpost_utils::queries::query_wynd_pool_swap;
use wynd_stake;
use wyndex::{
    asset::{Asset, AssetInfo},
    pair::{PairInfo, SimulationResponse},
};
use wyndex_multi_hop::msg::SwapOperation;
use wyndex_stake::msg::RewardsPowerResponse;

use crate::{
    execute::{
        JUNO_WYND_PAIR_ADDR, NETA_CW20_ADDR, WYNDDEX_FACTORY_ADDR, WYND_CW20_ADDR,
        WYND_MULTI_HOP_ADDR,
    },
    msg::VersionResponse,
    ContractError,
};

pub fn query_version() -> VersionResponse {
    VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

pub fn query_current_user_pools(
    querier: &QuerierWrapper,
    delegator_addr: &Addr,
) -> StdResult<Vec<(PairInfo, RewardsPowerResponse)>> {
    let pools: wyndex::factory::PairsResponse = querier.query_wasm_smart(
        WYNDDEX_FACTORY_ADDR.to_string(),
        &wyndex::factory::QueryMsg::Pairs {
            start_after: None,
            limit: None,
        },
    )?;

    unimplemented!()

    // let current_user_pools = pools
    //     .pairs
    //     .iter()
    //     .filter_map(|pair| {
    //         let outstanding_rewards: Result<RewardsPowerResponse, StdError> = querier
    //             .query_wasm_smart(
    //                 pair.staking_addr,
    //                 &wyndex_stake::msg::QueryMsg::RewardsPower {
    //                     address: delegator_addr.to_string(),
    //                 },
    //             );

    //         match outstanding_rewards {
    //             Ok(RewardsPowerResponse { rewards })
    //                 if rewards.iter().any(|reward| !reward.1.is_zero()) =>
    //             {
    //                 Some((
    //                     pair.clone(),
    //                     RewardsPowerResponse {
    //                         rewards: rewards
    //                             .into_iter()
    //                             .filter(|(_, amount)| !amount.is_zero())
    //                             .collect(),
    //                     },
    //                 ))
    //             }
    //             _ => None,
    //         }
    //     })
    //     .collect();

    // Ok(current_user_pools)
}

pub fn query_pending_wynd_pool_rewards(
    querier: &QuerierWrapper,
    delegator: &Addr,
) -> Result<(), ContractError> {
    wyndex_stake::msg::QueryMsg::WithdrawableRewards {
        owner: delegator.to_string(),
    };

    todo!("get all the pending rewards per active pool");
    // .iter()
    // .map(|addr| {
    //     let rewards = querier.query_rewards(addr, delegator)?;
    //     Ok(rewards)
    // })
    // .collect::<Result<Vec<_>, _>>()?;

    // Ok(())
}
