use actix_web::{
    http::{header::ContentType, StatusCode},
    web, App, HttpResponse, HttpServer, Responder,
};
use bson::Document;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcError;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_program::clock::UnixTimestamp;
use solana_program::pubkey::Pubkey;
use solana_program::slot_history::Slot;
use solana_sdk::transaction::TransactionError;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature},
    system_transaction,
};
use solana_transaction_status::UiTransactionEncoding;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::mpsc;

mod utils;
use utils::{filter_data_start_end_date, ParsedData};

use std::thread;

use bson::doc;

use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn get_data(info: web::Json<ParsedData>) -> impl Responder {
    // RpcClient endpoint
    let rpc_client = RpcClient::new(
        "https://api.mainnet-beta.solana.com
    ",
    );

    // config for rpc get_signatures_for_address_with_config request
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(50),
        commitment: Some(CommitmentConfig::confirmed()),
    };

    // Chingari token id
    let chingari_token_id =
        Pubkey::from_str("CKaKtYvz6dKPyMvYq9Rh3UBrnNqYZAyd7iF4hJtjUvks").unwrap();

    // sender and receiver for thread
    let (tx, rx) = mpsc::channel();
    let (tx1, rx2) = mpsc::channel();

    thread::spawn(move || {
        let signatures =
            match rpc_client.get_signatures_for_address_with_config(&chingari_token_id, config) {
                Ok(data) => tx.send(data).unwrap(),
                Err(_) => {
                    unimplemented!()
                }
            };

        let signatures = rx.recv().unwrap();
        let mut txn: Vec<EncodedConfirmedTransactionWithStatusMeta> = vec![];

        for sig in signatures {
            txn.push(
                rpc_client
                    .get_transaction(
                        &Signature::from_str(&sig.signature).unwrap(),
                        UiTransactionEncoding::JsonParsed,
                    )
                    .unwrap(),
            );
        }

        let data = info.into_inner();
        let start_date = data.start_date;
        let end_date = data.end_date;

        // let start_date ="2022-08-04".to_string();
        // let end_date ="2022-11-14".to_string();

        let (start, end) = ParsedData {
            start_date,
            end_date,
        }
        .parse_data_to_date();

        let result = filter_data_start_end_date(txn, start, end);

        tx1.send(result);
    });

    let mut response: Vec<Document> = Vec::new();

    // recived data from thread
    let received = rx2.recv().unwrap();
    // filter new user transactions
    // new users transaction whos pre_token_balances vector lenth is 0
    let mut fiter_txn: Vec<EncodedConfirmedTransactionWithStatusMeta> = Vec::new();

    for data in received {
        let somthing = match data.transaction.meta {
            Some(ref a) => match &a.pre_token_balances {
                OptionSerializer::Some(a) => {
                    if a.len() == 0 {
                        fiter_txn.push(data);
                    }
                }
                OptionSerializer::None => todo!(),
                OptionSerializer::Skip => todo!(),
            },
            None => todo!(),
        };
    }

    for data in fiter_txn {
        let somthing = doc! {
            "slot":data.slot.to_string(),
            "transaction":match data.transaction.meta{
                Some(a)=> doc!{
                    "new_user": match a.post_token_balances{
                        OptionSerializer::Some(a) =>match &a[0]{
                            x=> doc!{
                                "owner_pubkey": format!("{:?}",x.owner),
                                "post_balance": format!("{:?}",x.ui_token_amount.amount),

                            },
                            _=> doc!{
                                "owner_pubkey":"none".to_string(),
                                "post_balance":"none".to_string(),

                            }
                        },
                        _ => {
                            doc! {
                                "none2":"none",
                            }
                        },

                    },

                },
                None=>doc!
                {"postBalances": "none",

            }
            },
            "block_time":match data.block_time{
                Some(a)=>a.to_string(),
                None=>"none".to_string()},

        };

        response.push(somthing);
    }
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("X-Hdr", "sample"))
        .json(response);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/get_new_users_data", web::get().to(get_data)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
