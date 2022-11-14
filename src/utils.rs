use chrono::{Local, NaiveDate, TimeZone};
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

pub fn filter_data_start_end_date(
    txs: Vec<EncodedConfirmedTransactionWithStatusMeta>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Vec<EncodedConfirmedTransactionWithStatusMeta> {
    let filtered_data: Vec<EncodedConfirmedTransactionWithStatusMeta> = txs
        .into_iter()
        .filter(|txn| {
            let txn_date = Local
                .timestamp_opt(txn.block_time.unwrap(), 0)
                .unwrap()
                .date_naive();
            txn_date >= start_date && txn_date <= end_date
        })
        .collect();

    filtered_data
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ParsedData {
    pub start_date: String,
    pub end_date: String,
}

impl ParsedData {
    pub fn parse_data_to_date(&self) -> (NaiveDate, NaiveDate) {
        (
            NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap(),
        )
    }
}
