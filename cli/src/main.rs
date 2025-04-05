use std::{
    io::{Read, Write},
    process::ExitCode,
};

mod args;
use args::{Args, CalcType};
use chrono::NaiveTime;
use clap::Parser;
use core_payment_plan::{DownPaymentParams, Params};
use payment_plan_cli::{
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
        CalcType::NextDisbursementDate => next_disbursement_date(buf),
    };

    return code;
}

fn calc(buf: Vec<u8>) -> ExitCode {
    let params = payment_plan_cli::deserialize_params(&buf).unwrap();
    let params: Result<Params, _> = params.try_into();
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

    let response = payment_plan_cli::serialize_responses(response);

    //Write the response to stdout
    std::io::stdout().write_all(&response).unwrap();

    return ExitCode::SUCCESS;
}

fn down_calc(buf: Vec<u8>) -> ExitCode {
    let params = deserialize_down_payment_params(&buf).unwrap();
    let params: Result<DownPaymentParams, _> = params.try_into();
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

    let response = payment_plan_cli::serialize_down_payment_responses(response);

    //Write the response to stdout
    std::io::stdout().write_all(&response).unwrap();

    return ExitCode::SUCCESS;
}

fn next_disbursement_date(buf: Vec<u8>) -> ExitCode {
    let buf: [u8; 8] = match buf.as_slice().try_into() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Failed to convert buffer to u64: {:?}", e);
            return ExitCode::FAILURE;
        }
    };
    let date = i64::from_be_bytes(buf);
    let requested_date = chrono::DateTime::from_timestamp_millis(date);
    let requested_date = match requested_date {
        Some(date) => date.date_naive(),
        None => {
            eprintln!("Error: Invalid requested date");
            return ExitCode::FAILURE;
        }
    };

    let response = core_payment_plan::next_disbursement_date(requested_date);

    let response = response
        .and_time(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
        .and_utc()
        .timestamp_millis();

    let response = response.to_be_bytes();

    //Write the response to stdout
    std::io::stdout().write_all(&response).unwrap();

    ExitCode::SUCCESS
}
