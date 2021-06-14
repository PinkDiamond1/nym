// due to code generated by JsonSchema
#![allow(clippy::field_reassign_with_default)]

use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct Delegation {
    node_identity: String,
    amount: Coin,
}

impl Delegation {
    pub fn new(node_identity: String, amount: Coin) -> Self {
        Delegation {
            node_identity,
            amount,
        }
    }

    pub fn amount(&self) -> &Coin {
        &self.amount
    }

    pub fn node_identity(&self) -> &String {
        &self.node_identity
    }
}

impl Display for Delegation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} delegated to {}",
            self.amount.amount, self.amount.denom, self.node_identity
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedMixDelegationsResponse {
    pub node_identity: String,
    pub delegations: Vec<Delegation>,
    pub start_next_after: Option<String>,
}

impl PagedMixDelegationsResponse {
    pub fn new(
        node_identity: String,
        delegations: Vec<Delegation>,
        start_next_after: Option<String>,
    ) -> Self {
        PagedMixDelegationsResponse {
            node_identity,
            delegations,
            start_next_after,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedGatewayDelegationsResponse {
    pub node_identity: String,
    pub delegations: Vec<Delegation>,
    pub start_next_after: Option<String>,
}

impl PagedGatewayDelegationsResponse {
    pub fn new(
        node_identity: String,
        delegations: Vec<Delegation>,
        start_next_after: Option<String>,
    ) -> Self {
        PagedGatewayDelegationsResponse {
            node_identity,
            delegations,
            start_next_after,
        }
    }
}
