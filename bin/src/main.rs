use std::{
    io::{Read, Write},
    process::ExitCode,
};

mod args;
use args::{Args, CalcType};
use clap::Parser;
use core_payment_plan::types::{down_payment_plan, plan};
use payment_plan::{
    deserialize_down_payment_params,
    types::{DownPaymentResponses, PlanResponses},
};

fn main() -> ExitCode {
    //Args is a struct implements the trait clap::Parser
    let args = Args::parse();

    let mut buf: Vec<u8> = Vec::new();

    //Read the input from stdin, this will be binary non human readable data
    std::io::stdin().read_to_end(&mut buf).unwrap();
    let c_type = args.calc_type;

    let code = match c_type {
        CalcType::Normal => calc(buf),
        CalcType::DownPayment => down_calc(buf),
    };

    return code;
}

fn calc(buf: Vec<u8>) -> ExitCode {
    let params = payment_plan::deserialize_params(&buf).unwrap();
    let params: Result<plan::Params, _> = params.try_into();
    let params = match params {
        Ok(params) => params,
        Err(e) => {
            //eprintln! is a macro that prints to stderr
            eprintln!("Error: Invalid input: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let response = core_payment_plan::calculate_payment_plan(params);

    let response = match response {
        Ok(response) => response,
        Err(e) => {
            //eprintln! is a macro that prints to stderr
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let response: PlanResponses = response.into();

    let response = payment_plan::serialize_responses(response);

    //Write the response to stdout
    std::io::stdout().write_all(&response).unwrap();

    return ExitCode::SUCCESS;
}

fn down_calc(buf: Vec<u8>) -> ExitCode {
    let params = deserialize_down_payment_params(&buf).unwrap();
    let params: Result<down_payment_plan::Params, _> = params.try_into();
    let params = match params {
        Ok(params) => params,
        Err(e) => {
            //eprintln! is a macro that prints to stderr
            eprintln!("Error: Invalid input: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let response = core_payment_plan::calculate_down_payment_plan(params);

    let response = match response {
        Ok(response) => response,
        Err(e) => {
            //eprintln! is a macro that prints to stderr
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let response: DownPaymentResponses = response.into();

    let response = payment_plan::serialize_down_payment_responses(response);

    //Write the response to stdout
    std::io::stdout().write_all(&response).unwrap();

    return ExitCode::SUCCESS;
}
