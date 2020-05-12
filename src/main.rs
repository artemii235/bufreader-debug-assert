use async_std::task::sleep;
use futures::{
    future::select,
    FutureExt, StreamExt,
};
use std::{
    net::SocketAddr,
    str::FromStr,
    thread,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime::Builder,
};

pub async fn server_loop() {
    let mut listener = TcpListener::bind(SocketAddr::from_str("127.0.0.1:9000").unwrap()).await.unwrap();
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                continue;
            },
        };
        let (read, mut write) = stream.into_split();
        let read_loop = async move {
            let mut read = tokio::io::BufReader::new(read);
            let mut buffer = String::with_capacity(1024);
            loop {
                match read.read_line(&mut buffer).await {
                    Ok(read) => if read > 0 && buffer.len() > 0 {
                        println!("{}", buffer);
                        buffer.clear();
                    } else if read == 0 {
                        // ctx_read.log.log("😟", &[&"incoming_connection", &peer_addr.to_string().as_str()], &format!("Reached EOF, dropping connection"));
                        break;
                    },
                    Err(e) => {
                        // ctx_read.log.log("😟", &[&"incoming_connection", &peer_addr.to_string().as_str()], &format!("Error {} reading from socket, dropping connection", e));
                        break;
                    }
                }
            }
        };
        let write_loop = async move {
            loop {
                match write.write_all(b"Hello").await {
                    Ok(_) => (),
                    Err(_) => break,
                };
                sleep(Duration::from_secs(5)).await;
            }
        };
        // selecting over the read and write parts processing loops in order to
        // drop both parts and close connection in case of errors
        tokio::spawn(select(Box::pin(read_loop), Box::pin(write_loop)));
    }
}

pub async fn client_loop() {
    loop {
        let mut stream = TcpStream::connect(SocketAddr::from_str("127.0.0.1:9000").unwrap()).await.unwrap();
        stream.write_all(r#"{"type":"Taker","uuid":"5acb0e63-8b26-469e-81df-7dd9e4a9ad15","events":[{"timestamp":1588244036253,"event":{"type":"Started","data":{"taker_coin":"MYCOIN1","maker_coin":"MYCOIN","maker":"987d5f82205a55d789616f470ae9df48537f050ee050d501aa316651642a0a4d","my_persistent_pub":"02859a80b83941e4e8ff2f511080e3ea5021db4ba95caec30eb37864e71ae73521","lock_duration":7800,"maker_amount":"999.99999","taker_amount":"999.99999","maker_payment_confirmations":1,"maker_payment_requires_nota":false,"taker_payment_confirmations":1,"taker_payment_requires_nota":false,"taker_payment_lock":1588251836,"uuid":"5acb0e63-8b26-469e-81df-7dd9e4a9ad15","started_at":1588244036,"maker_payment_wait":1588247156,"maker_coin_start_block":12,"taker_coin_start_block":11}}},{"timestamp":1588244038239,"event":{"type":"Negotiated","data":{"maker_payment_locktime":1588259636,"maker_pubkey":"02987d5f82205a55d789616f470ae9df48537f050ee050d501aa316651642a0a4d","secret_hash":"9304bce3196f344b2a22dc99db406e95ab6f3107"}}},{"timestamp":1588244038271,"event":{"type":"TakerFeeSent","data":{"tx_hex":"0400008085202f8901bdde9bca02870787441f6068e4c2a869a3aac3d1d0925f6a6e27874343544d0a010000006a47304402206694a794693b55fbe8205cb1cbb992d92fa2a9a851763ad1bf1628c16deaf73e02203cfff465504bdd6c51e8fbd45dd2e1187142fdc29e82f36f577616d9d6097d7a012102859a80b83941e4e8ff2f511080e3ea5021db4ba95caec30eb37864e71ae73521ffffffff02dfceab07000000001976a914ca1e04745e8ca0c60d8c5881531d51bec470743f88ac39fd41892e0000001976a914d24e799df360da3ca3158d63b89ffaff27722c1588ac46aeaa5e000000000000000000000000000000","tx_hash":"7118d7484d1cbfdd6126673a848a228856f8748d946f6a9a440c90f0d62e27c6","from":["RUTBzLtJNTn89Wkb6oZocbesKrjBDTRMrC"],"to":["RThtXup6Zo7LZAi8kRWgjAyi1s4u6U9Cpf","RUTBzLtJNTn89Wkb6oZocbesKrjBDTRMrC"],"total_amount":"2000","spent_by_me":"2000","received_by_me":"1998.71298873","my_balance_change":"-1.28701127","block_height":0,"timestamp":0,"fee_details":{"amount":"0.00001"},"coin":"MYCOIN1","internal_id":"7118d7484d1cbfdd6126673a848a228856f8748d946f6a9a440c90f0d62e27c6"}}},{"timestamp":1588244038698,"event":{"type":"MakerPaymentReceived","data":{"tx_hex":"0400008085202f890163cc0aceb3b432f84c1407991a0389b74b7842f030aa261203aa0b4b9a9a15fd010000006b483045022100f3239794b7b0e1c75aae084a65535270f154231ce86a4739976ff69eeff1ebd402206c0aeb28b7b4b2e77e98b3c3efd9a20d85c2cd0627acc3f45d577c0e88c34fb4012102987d5f82205a55d789616f470ae9df48537f050ee050d501aa316651642a0a4dffffffff0118e476481700000017a914dc4ed4686174503b85374bfb0aefe07a9fb37bcf8746aeaa5e000000000000000000000000000000","tx_hash":"9a4c0b3f85ed0bb24dc9575ce7c2fd6bc50ad9d37c91c478946f9c33d15abfdf","from":["RAzSdYQhjCFdyhjrBz1AZQDVg3Hu8DrzYc"],"to":["bYp9ncp3V7FYsymipriVbdd3QL72hK9hio"],"total_amount":"1000","spent_by_me":"0","received_by_me":"0","my_balance_change":"0","block_height":0,"timestamp":0,"fee_details":{"amount":"0.00001"},"coin":"MYCOIN","internal_id":"9a4c0b3f85ed0bb24dc9575ce7c2fd6bc50ad9d37c91c478946f9c33d15abfdf"}}},{"timestamp":1588244038699,"event":{"type":"MakerPaymentWaitConfirmStarted"}},{"timestamp":1588244053712,"event":{"type":"MakerPaymentValidatedAndConfirmed"}},{"timestamp":1588244053745,"event":{"type":"TakerPaymentSent","data":{"tx_hex":"0400008085202f8901c6272ed6f0900c449a6a6f948d74f85688228a843a672661ddbf1c4d48d71871010000006b483045022100c37627385c66b7bdf466b4dd4e7095b7551e6f5ea35f9bcc6344eb629f2edcb202203280eaba64b4d72010500166fab62cf34a687a516b2fe83d4eceaf8572cb37a7012102859a80b83941e4e8ff2f511080e3ea5021db4ba95caec30eb37864e71ae73521ffffffff0218e476481700000017a91431ff75ed72cd135a7ce50d121e71efc37066b9f9873915cb40170000001976a914d24e799df360da3ca3158d63b89ffaff27722c1588ac55aeaa5e000000000000000000000000000000","tx_hash":"b4718ce94aa43f9073ab0b70c18ab4ea4b587338fb7110f20ff7d1bb452df08f","from":["RUTBzLtJNTn89Wkb6oZocbesKrjBDTRMrC"],"to":["RUTBzLtJNTn89Wkb6oZocbesKrjBDTRMrC","bHHdqM8XWDHee2oyzydwUJKEE2BjofEtZH"],"total_amount":"1998.71298873","spent_by_me":"1998.71298873","received_by_me":"998.71298873","my_balance_change":"-1000","block_height":0,"timestamp":0,"fee_details":{"amount":"0.00001"},"coin":"MYCOIN1","internal_id":"b4718ce94aa43f9073ab0b70c18ab4ea4b587338fb7110f20ff7d1bb452df08f"}}},{"timestamp":1588244078860,"event":{"type":"TakerPaymentSpent","data":{"transaction":{"tx_hex":"0400008085202f89018ff02d45bbd1f70ff21071fb3873584beab48ac1700bab73903fa44ae98c71b400000000d747304402204f0d641a3916e54d6788744c3229110a431eff18634c66fbd1741f9ca7dba99d02202315ee1d9317cc4d5d75d01f066f2c8a59876f790106d310144cdc03c25f985e0120dedf3c8dcfff9ee2787b4bf211f960fd044fdab7fa8e922ef613a0115848a498004c6b6304bcccaa5eb1752102859a80b83941e4e8ff2f511080e3ea5021db4ba95caec30eb37864e71ae73521ac6782012088a9149304bce3196f344b2a22dc99db406e95ab6f3107882102987d5f82205a55d789616f470ae9df48537f050ee050d501aa316651642a0a4dac68ffffffff0130e07648170000001976a91412c553e8469363f2d30268c475af1e9186cc90af88ac54a0aa5e000000000000000000000000000000","tx_hash":"e7aed7a77e47b44dc9d12166589bbade70faea10b64888f73ed4be04bcc9f9a9","from":["bHHdqM8XWDHee2oyzydwUJKEE2BjofEtZH"],"to":["RAzSdYQhjCFdyhjrBz1AZQDVg3Hu8DrzYc"],"total_amount":"999.99999","spent_by_me":"0","received_by_me":"0","my_balance_change":"0","block_height":29,"timestamp":1588244070,"fee_details":{"amount":"0.00001"},"coin":"MYCOIN1","internal_id":"e7aed7a77e47b44dc9d12166589bbade70faea10b64888f73ed4be04bcc9f9a9"},"secret":"dedf3c8dcfff9ee2787b4bf211f960fd044fdab7fa8e922ef613a0115848a498"}}},{"timestamp":1588244078870,"event":{"type":"MakerPaymentSpent","data":{"tx_hex":"0400008085202f8901dfbf5ad1339c6f9478c4917cd3d90ac56bfdc2e75c57c94db20bed853f0b4c9a00000000d848304502210092535c081325ba5261699d7cfd4c503fb6125dde86389b83f40f3e2c006039bb022063cfd72aa15558dee874cac08b22dbcf11d3f06c8e48b0ddaf75b86887d604410120dedf3c8dcfff9ee2787b4bf211f960fd044fdab7fa8e922ef613a0115848a498004c6b630434ebaa5eb1752102987d5f82205a55d789616f470ae9df48537f050ee050d501aa316651642a0a4dac6782012088a9149304bce3196f344b2a22dc99db406e95ab6f3107882102859a80b83941e4e8ff2f511080e3ea5021db4ba95caec30eb37864e71ae73521ac68ffffffff0130e07648170000001976a914d24e799df360da3ca3158d63b89ffaff27722c1588ac5ea0aa5e000000000000000000000000000000","tx_hash":"caea128b1c85a88abd5924e512780ee18952dadc217b0c06f4b2820eb71d03bc","from":["bYp9ncp3V7FYsymipriVbdd3QL72hK9hio"],"to":["RUTBzLtJNTn89Wkb6oZocbesKrjBDTRMrC"],"total_amount":"999.99999","spent_by_me":"0","received_by_me":"999.99998","my_balance_change":"999.99998","block_height":0,"timestamp":0,"fee_details":{"amount":"0.00001"},"coin":"MYCOIN","internal_id":"caea128b1c85a88abd5924e512780ee18952dadc217b0c06f4b2820eb71d03bc"}}},{"timestamp":1588244078871,"event":{"type":"Finished"}}],"maker_amount":"999.99999","maker_coin":"MYCOIN","taker_amount":"999.99999","taker_coin":"MYCOIN1","gui":"nogui","mm_version":"UNKNOWN","success_events":["Started","Negotiated","TakerFeeSent","MakerPaymentReceived","MakerPaymentWaitConfirmStarted","MakerPaymentValidatedAndConfirmed","TakerPaymentSent","TakerPaymentSpent","MakerPaymentSpent","Finished"],"error_events":["StartFailed","NegotiateFailed","TakerFeeSendFailed","MakerPaymentValidateFailed","MakerPaymentWaitConfirmFailed","TakerPaymentTransactionFailed","TakerPaymentWaitConfirmFailed","TakerPaymentDataSendFailed","TakerPaymentWaitForSpendFailed","MakerPaymentSpendFailed","TakerPaymentWaitRefundStarted","TakerPaymentRefunded","TakerPaymentRefundFailed"]}
        "#.as_bytes()).await.unwrap();
        sleep(Duration::from_millis(3000)).await;
    }
}

fn main() {
    // let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let mut runtime = Builder::new().basic_scheduler().enable_all().build().unwrap();
    runtime.block_on(select(Box::pin(server_loop()), Box::pin(client_loop())));
}
