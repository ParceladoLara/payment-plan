use clap::{Parser, ValueEnum};

#[derive(Default, ValueEnum, Clone, Copy, Debug)]
pub enum CalcType {
    #[default]
    Normal,
    DownPayment,
    #[clap(name = "next-disbursement-date", alias = "nd")]
    NextDisbursementDate,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short = 't', long = "type", default_value_t, value_enum)]
    pub calc_type: CalcType,
}
