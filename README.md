# Assignment task

### Assignment task

Inputs: start_time, end_time
spl_token_id = “CKaKtYvz6dKPyMvYq9Rh3UBrnNqYZAyd7iF4hJtjUvks”
Get all transactions from solana main net for the above spl_token_id in specified start / end time range.
Parse all new users from it, pre_balance = 0, post_balance > 0

### Assignment is built on actix_web
Assignment solution is written in rust's actix framework. The user has to call one endpoint 
/get_new_users_data with a JSON blob in request including the start_date and end_date. Then the endpoint will return the data related to the new user.

Steps - 
* Cargo run to start the server.
* url path :127.0.0.1:8080/get_new_users_data
* jsob blob --{
  "start_date":"2022-08-04",
  "end_date": "2022-11-18"
}

### check new users from transactions.

1. frist we required the vec of transaction between the dates rages.
2. Then we need to check the transactions meta data (1).
3. From the transactions meta data, we must retrieve the pre token balances and post token balances (1).
4. Pre- and post-token balances are just a vector of "UITransactionTokenBalance," or "Vec" for short.
5. Pre token balances 'Vec' must be length zero in order for there to be no gari token ATA for the user prior to the transaction.
6. The user's gari token ATA has a balance of "amount: "0"" after the transaction (post token balance) since 0.000000001 gari token was airdropped to the user's gari ATA, the amount is near to 0, therefore it appears as "amount: "0"" on the transaction's meta data.
7. Get the token owner from the post token balances 'Vec' if the aforementioned is true for the transaction, which indicates that this is a new user.

#Note
This application retrieves the most recent 1000 signatures associated with the GARI token, which receives thousands of signatures every day. However, the RpcClient can only provide us with about 1000 recent signatures at once . In order to obtain an older signature, you must give the last signature of  into the RPC call ('get signatures for address with config()') with the argument 'before: last signature.
You will receive 2000 signatures if you repeat this process twice. If you perform it three times, you will receive 3000 signatures, and so on.
The software needs a lot of signatures if we enter really historical dates, such start date and end date from a few months ago.