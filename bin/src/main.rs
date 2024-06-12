use std::{
    io::{Read, Write},
    process::ExitCode,
};

use core_payment_plan::Params;
use payment_plan::types::PlanResponses;

fn main() -> ExitCode {
    let mut buf: Vec<u8> = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap();
    let params = payment_plan::deserialize_params(&buf).unwrap();
    let params: Result<Params, _> = params.try_into();
    let params = match params {
        Ok(params) => params,
        Err(e) => {
            eprintln!("Error: Invalid input: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let response = core_payment_plan::calc::calculate_payment_plan(params);

    let response = match response {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let response: PlanResponses = response.into();

    let response = payment_plan::serialize_responses(response);
    std::io::stdout().write_all(&response).unwrap();

    return ExitCode::SUCCESS;
}
