// Copyright 2021 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

pub use crate::nymd::cosmwasm_client::signing_client::SigningCosmWasmClient;
use crate::nymd::cosmwasm_client::types::ExecuteResult;
use crate::nymd::error::NymdError;
use crate::nymd::fee::helpers::Operation;
use crate::nymd::{cosmwasm_coin_to_cosmos_coin, NymdClient};
use async_trait::async_trait;
use cosmwasm_std::Coin;
use mixnet_contract::{Gateway, IdentityKey, MixNode};
use vesting_contract::messages::ExecuteMsg as VestingExecuteMsg;

#[async_trait]
pub trait VestingSigningClient {
    async fn vesting_bond_gateway(
        &self,
        gateway: Gateway,
        pledge: Coin,
        owner_signature: &str,
    ) -> Result<ExecuteResult, NymdError>;

    async fn vesting_unbond_gateway(&self) -> Result<ExecuteResult, NymdError>;

    async fn vesting_track_unbond_gateway(
        &self,
        owner: &str,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError>;

    async fn vesting_bond_mixnode(
        &self,
        mix_node: MixNode,
        pledge: Coin,
        owner_signature: &str,
    ) -> Result<ExecuteResult, NymdError>;
    async fn vesting_unbond_mixnode(&self) -> Result<ExecuteResult, NymdError>;

    async fn vesting_track_unbond_mixnode(
        &self,
        owner: &str,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError>;

    async fn withdraw_vested_coins(&self, amount: Coin) -> Result<ExecuteResult, NymdError>;

    async fn vesting_track_undelegation(
        &self,
        address: &str,
        mix_identity: IdentityKey,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError>;

    async fn vesting_delegate_to_mixnode(
        &self,
        mix_identity: IdentityKey,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError>;

    async fn vesting_undelegate_from_mixnode(
        &self,
        mix_identity: IdentityKey,
    ) -> Result<ExecuteResult, NymdError>;

    async fn create_periodic_vesting_account(
        &self,
        address: &str,
        start_time: Option<u64>,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError>;
}

#[async_trait]
impl<C: SigningCosmWasmClient + Sync + Send> VestingSigningClient for NymdClient<C> {
    async fn vesting_bond_gateway(
        &self,
        gateway: Gateway,
        pledge: Coin,
        owner_signature: &str,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::BondGateway);
        let req = VestingExecuteMsg::BondGateway {
            gateway,
            owner_signature: owner_signature.to_string(),
        };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::BondGateway",
                vec![cosmwasm_coin_to_cosmos_coin(pledge)],
            )
            .await
    }

    async fn vesting_unbond_gateway(&self) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::UnbondGateway);
        let req = VestingExecuteMsg::UnbondGateway {};
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::UnbondGateway",
                vec![],
            )
            .await
    }

    async fn vesting_track_unbond_gateway(
        &self,
        owner: &str,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::TrackUnbondGateway);
        let req = VestingExecuteMsg::TrackUnbondGateway {
            owner: owner.to_string(),
            amount,
        };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::TrackUnbondGateway",
                vec![],
            )
            .await
    }

    async fn vesting_bond_mixnode(
        &self,
        mix_node: MixNode,
        pledge: Coin,
        owner_signature: &str,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::BondMixnode);
        let req = VestingExecuteMsg::BondMixnode {
            mix_node,
            owner_signature: owner_signature.to_string(),
        };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::BondMixnode",
                vec![cosmwasm_coin_to_cosmos_coin(pledge)],
            )
            .await
    }

    async fn vesting_unbond_mixnode(&self) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::UnbondMixnode);
        let req = VestingExecuteMsg::UnbondMixnode {};
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::UnbondMixnode",
                vec![],
            )
            .await
    }

    async fn vesting_track_unbond_mixnode(
        &self,
        owner: &str,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::TrackUnbondMixnode);
        let req = VestingExecuteMsg::TrackUnbondMixnode {
            owner: owner.to_string(),
            amount,
        };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::TrackUnbondMixnode",
                vec![],
            )
            .await
    }

    async fn withdraw_vested_coins(&self, amount: Coin) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::WithdrawVestedCoins);
        let req = VestingExecuteMsg::WithdrawVestedCoins { amount };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::WithdrawVested",
                vec![],
            )
            .await
    }

    async fn vesting_track_undelegation(
        &self,
        address: &str,
        mix_identity: IdentityKey,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::TrackUndelegation);
        let req = VestingExecuteMsg::TrackUndelegation {
            owner: address.to_string(),
            mix_identity,
            amount,
        };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::TrackUndelegation",
                vec![],
            )
            .await
    }
    async fn vesting_delegate_to_mixnode(
        &self,
        mix_identity: IdentityKey,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::DelegateToMixnode);
        let req = VestingExecuteMsg::DelegateToMixnode { mix_identity };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::DeledateToMixnode",
                vec![cosmwasm_coin_to_cosmos_coin(amount)],
            )
            .await
    }
    async fn vesting_undelegate_from_mixnode(
        &self,
        mix_identity: IdentityKey,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::UndelegateFromMixnode);
        let req = VestingExecuteMsg::UndelegateFromMixnode { mix_identity };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::UndelegateFromMixnode",
                vec![],
            )
            .await
    }
    async fn create_periodic_vesting_account(
        &self,
        address: &str,
        start_time: Option<u64>,
        amount: Coin,
    ) -> Result<ExecuteResult, NymdError> {
        let fee = self.operation_fee(Operation::CreatePeriodicVestingAccount);
        let req = VestingExecuteMsg::CreateAccount {
            address: address.to_string(),
            start_time,
        };
        self.client
            .execute(
                self.address(),
                self.vesting_contract_address()?,
                &req,
                fee,
                "VestingContract::CreatePeriodicVestingAccount",
                vec![cosmwasm_coin_to_cosmos_coin(amount)],
            )
            .await
    }
}
