use crate::helpers::{denominate, nominated_str};
use actix_web::HttpResponse;
use dharitri_sc_snippets::imports::RustBigUint;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeployReqBody {
    pub ping_amount: f64,
    pub max_funds: f64,
    pub activation_timestamp: String,
    pub duration: u64,
    pub deployer: String,
}

impl DeployReqBody {
    pub fn get_tx_sending_values(&self) -> (String, String, String, u64, String) {
        (
            denominate(self.ping_amount),
            denominate(self.max_funds),
            self.activation_timestamp.clone(),
            self.duration,
            self.deployer.clone(),
        )
    }
}

#[derive(Deserialize, Serialize)]
pub struct DeployResponse {
    response: String,
    address: String,
}

#[allow(unused)]
impl DeployResponse {
    pub fn new(tx_response: (String, String)) -> Self {
        Self {
            response: tx_response.0,
            address: tx_response.1,
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PingReqBody {
    pub value: f64,
}

impl PingReqBody {
    pub fn get_denominated_amount(&self) -> String {
        denominate(self.value)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PingResponse {
    response: String,
    amount: String,
}

#[allow(unused)]
impl PingResponse {
    pub fn new(response: String, amount: RustBigUint) -> Self {
        Self {
            response,
            amount: nominated_str(amount),
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SuccessTxResponse {
    response: String,
}

impl SuccessTxResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
