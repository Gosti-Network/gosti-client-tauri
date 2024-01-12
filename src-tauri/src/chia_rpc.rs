use serde_json::Value;

mod chia_rpc_types;

use chia_rpc_types::is_valid_command;
use reqwest::tls::Identity;
use reqwest::tls::Certificate;
use dirs;



// pub const WALLET_TYPE_STANDARD_WALLET: u64 = 0;
// pub const WALLET_TYPE_ATOMIC_SWAP: u64 = 2;
// pub const WALLET_TYPE_AUTHORIZED_PAYEE: u64 = 3;
// pub const WALLET_TYPE_MULTI_SIG: u64 = 4;
// pub const WALLET_TYPE_CUSTODY: u64 = 5;
// pub const WALLET_TYPE_CAT: u64 = 6;
// pub const WALLET_TYPE_RECOVERY: u64 = 7;
// pub const WALLET_TYPE_DECENTRALIZED_ID: u64 = 8;
// pub const WALLET_TYPE_POOLING_WALLET: u64 = 9;
pub const WALLET_TYPE_NFT: u64 = 10;
// pub const WALLET_TYPE_DATA_LAYER: u64 = 11;
// pub const WALLET_TYPE_DATA_LAYER_OFFER: u64 = 12;
// pub const WALLET_TYPE_VC: u64 = 13;
// pub const WALLET_TYPE_DAO: u64 = 14;
// pub const WALLET_TYPE_DAOCAT: u64 = 15;
// pub const WALLET_TYPE_CRCAT: u64 = 57;


#[tauri::command]
pub async fn make_chia_rpc_call(command: &str, params: Value) -> Result<Value, String> {

    println!("make_chia_rpc_call: command: {}, {}", command, params);
    // Verify the command
    if !is_valid_command(&command) {
        return Err("Invalid command".to_string());
    }

    // Verify the params based on the command
    if !is_valid_params(&command, &params).0 {
        return Err("Invalid params for the command".to_string());
    }

    // Make the HTTP/JSON RPC call to the Chia server using the provided command and params
    // Replace this with your actual implementation
    let result = make_http_json_rpc_call(command, params).await;

    // Return the result or an error message
    match result {
        Ok(response) => Ok(response),
        Err(error) => Err(error.to_string()),
    }
}

async fn make_http_json_rpc_call(command: &str, params: Value) -> Result<Value, String> {

    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let cert_path = format!("{}/.chia/mainnet/config/{}", home_dir.display(), chia_rpc_types::get_ssl_cert_path(&command));
    let key_path = format!("{}/.chia/mainnet/config/{}", home_dir.display(), chia_rpc_types::get_ssl_key_path(&command));
    println!("cert_path: {}", cert_path);

    let cert_bytes = std::fs::read(cert_path).map_err(|err| err.to_string())?;
    let key_bytes = std::fs::read(key_path).map_err(|err| err.to_string())?;
    
    let ca = Certificate::from_pem(&cert_bytes).map_err(|err| err.to_string())?;
    let cert = Identity::from_pkcs8_pem(&cert_bytes, &key_bytes);


    println!("cert_bytes: {}", cert_bytes.len());
    println!("key_bytes: {}", key_bytes.len());

    let client = reqwest::Client::builder()
        .add_root_certificate(ca)
        .identity(cert.expect("Certificate invalid"))
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|err| err.to_string())?;

    println!("client: {:?}", client);

    let url = format!("https://localhost:{}/{}", chia_rpc_types::get_port_number(&command), command);
    let body = serde_json::json!(params);

    println!("url: {}", url);
    println!("body: {}", body);

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status_code = response.status();

    println!("status_code: {}", status_code);

    let response_json: Value = response.json().await.map_err(|err| err.to_string())?;

    println!("response_json: {}", response_json );

    Ok(response_json)
}

fn is_valid_params<'a>(command: &'a str, _: &'a Value) -> (bool, &'a str) {
    match command {
        "cancel_offer" => {
            // Add your criteria to validate the params for the "cancel_offer" command
            (true, "success")
        }
        "dao_add_funds_to_treasury" => {
            (true, "success")
        }
        "dao_adjust_filter_level" => {
            (true, "success")
        }
        "dao_close_proposal" => {
            (true, "success")
        }
        "dao_create_proposal" => {
            (true, "success")
        }
        "dao_exit_lockup" => {
            (true, "success")
        }
        "dao_free_coins_from_finished_proposals" => {
            (true, "success")
        }
        "dao_get_proposals" => {
            (true, "success")
        }
        "dao_get_proposal_state" => {
            (true, "success")
        }
        "dao_get_rules" => {
            (true, "success")
        }
        "dao_get_treasury_balance" => {
            (true, "success")
        }
        "dao_get_treasury_id" => {
            (true, "success")
        }
        "dao_parse_proposal" => {
            (true, "success")
        }
        "dao_send_to_lockup" => {
            (true, "success")
        }
        "dao_vote_on_proposal" => {
            (true, "success")
        }

        "exit" => {
            (true, "success")
        }
        "get_routes" => {
            (true, "success")
        }
        "get_status" => {
            (true, "success")
        }
        "get_version" => {
            (true, "success")
        }
        "get_wallet_addresses" => {
            (true, "success")
        }
        "is_keyring_locked" => {
            (true, "success")
        }
        "is_running" => {
            (true, "success")
        }
        "keyring_status" => {
            (true, "success")
        }
        "register_service" => {
            (true, "success")
        }
        "remove_keyring_passphrase" => {
            (true, "success")
        }
        "running_services" => {
            (true, "success")
        }
        "set_keyring_passphrase" => {
            (true, "success")
        }
        "start_service" => {
            (true, "success")
        }
        "stop_service" => {
            (true, "success")
        }
        "unlock_keyring" => {
            (true, "success")
        }
        "validate_keyring_passphrase" => {
            (true, "success")
        }

        "get_keys_for_plotting" => {
            (true, "success")
        }
        "get_plotters" => {
            (true, "success")
        }
        "start_plotting" => {
            (true, "success")
        }
        "stop_plotting" => {
            (true, "success")
        }

        "add_private_key" => {
            (true, "success")
        }
        "check_keys" => {
            (true, "success")
        }
        "delete_all_keys" => {
            (true, "success")
        }
        "delete_key_by_fingerprint" => {
            (true, "success")
        }
        "get_all_private_keys" => {
            (true, "success")
        }
        "get_first_private_key" => {
            (true, "success")
        }
        "get_key_for_fingerprint" => {
            (true, "success")
        }
        "get_key" => {
            (true, "success")
        }
        "get_keys" => {
            (true, "success")
        }
        "get_public_key" => {
            (true, "success")
        }
        "get_public_keys" => {
            (true, "success")
        }
        "set_label" => {
            (true, "success")
        }
        "delete_label" => {
            (true, "success")
        }

        "add_mirror" => {
            (true, "success")
        }
        "add_missing_files" => {
            (true, "success")
        }
        "batch_update" => {
            (true, "success")
        }
        "check_plugins" => {
            (true, "success")
        }
        "clear_pending_roots" => {
            (true, "success")
        }
        "create_data_store" => {
            (true, "success")
        }
        "delete_key" => {
            (true, "success")
        }
        "delete_mirror" => {
            (true, "success")
        }
        "get_ancestors" => {
            (true, "success")
        }
        "get_keys_values" => {
            (true, "success")
        }
        "get_kv_diff" => {
            (true, "success")
        }
        "get_local_root" => {
            (true, "success")
        }
        "get_mirrors" => {
            (true, "success")
        }
        "get_owned_stores" => {
            (true, "success")
        }
        "get_root" => {
            (true, "success")
        }
        "get_roots" => {
            (true, "success")
        }
        "get_root_history" => {
            (true, "success")
        }
        "get_sync_status" => {
            (true, "success")
        }
        "get_value" => {
            (true, "success")
        }
        "insert" => {
            (true, "success")
        }
        "make_offer" => {
            (true, "success")
        }
        "remove_subscriptions" => {
            (true, "success")
        }
        "subscribe" => {
            (true, "success")
        }
        "subscriptions" => {
            (true, "success")
        }
        "take_offer" => {
            (true, "success")
        }
        "unsubscribe" => {
            (true, "success")
        }
        "verify_offer" => {
            (true, "success")
        }
        "wallet_log_in" => {
            (true, "success")
        }

        "close_connection" => {
            (true, "success")
        }
        "get_connections" => {
            (true, "success")
        }
        "open_connection" => {
            (true, "success")
        }
        "stop_node" => {
            (true, "success")
        }

        "create_new_wallet" => {
            (true, "success")
        }
        "did_create_attest" => {
            (true, "success")
        }
        "did_create_backup_file" => {
            (true, "success")
        }
        "did_find_lost_did" => {
            (true, "success")
        }
        "did_get_current_coin_info" => {
            (true, "success")
        }
        "did_get_did" => {
            (true, "success")
        }
        "did_get_info" => {
            (true, "success")
        }
        "did_get_information_needed_for_recovery" => {
            (true, "success")
        }
        "did_get_metadata" => {
            (true, "success")
        }
        "did_get_pubkey" => {
            (true, "success")
        }
        "did_get_recovery_list" => {
            (true, "success")
        }
        "did_get_wallet_name" => {
            (true, "success")
        }
        "did_message_spend" => {
            (true, "success")
        }
        "did_recovery_spend" => {
            (true, "success")
        }
        "did_set_wallet_name" => {
            (true, "success")
        }
        "did_transfer_did" => {
            (true, "success")
        }
        "did_update_metadata" => {
            (true, "success")
        }
        "did_update_recovery_ids" => {
            (true, "success")
        }

        "nft_add_uri" => {
            (true, "success")
        }
        "nft_calculate_royalties" => {
            (true, "success")
        }
        "nft_count_nfts" => {
            (true, "success")
        }
        "nft_get_by_did" => {
            (true, "success")
        }
        "nft_get_info" => {
            (true, "success")
        }
        "nft_get_nfts" => {
            (true, "success")
        }
        "nft_get_wallets_with_dids" => {
            (true, "success")
        }
        "nft_get_wallet_did" => {
            (true, "success")
        }
        "nft_mint_bulk" => {
            (true, "success")
        }
        "nft_mint_nft" => {
            (true, "success")
        }
        "nft_set_did_bulk" => {
            (true, "success")
        }
        "nft_set_nft_did" => {
            (true, "success")
        }
        "nft_set_nft_status" => {
            (true, "success")
        }
        "nft_transfer_bulk" => {
            (true, "success")
        }
        "nft_transfer_nft" => {
            (true, "success")
        }

        "cancel_offers" => {
            (true, "success")
        }
        "check_offer_validity" => {
            (true, "success")
        }
        "create_offer_for_ids" => {
            (true, "success")
        }
        "get_all_offers" => {
            (true, "success")
        }
        "get_offer" => {
            (true, "success")
        }
        "get_offers_count" => {
            (true, "success")
        }
        "get_offer_summary" => {
            (true, "success")
        }

        "farm_block" => {
            (true, "success")
        }
        "get_all_blocks" => {
            (true, "success")
        }
        "get_all_coins" => {
            (true, "success")
        }
        "get_all_puzzle_hashes" => {
            (true, "success")
        }
        "get_auto_farming" => {
            (true, "success")
        }
        "get_farming_ph" => {
            (true, "success")
        }
        "reorg_blocks" => {
            (true, "success")
        }
        "revert_blocks" => {
            (true, "success")
        }
        "set_auto_farming" => {
            (true, "success")
        }

        "vc_add_proofs" => {
            (true, "success")
        }
        "vc_get" => {
            (true, "success")
        }
        "vc_get_list" => {
            (true, "success")
        }
        "vc_get_proofs_for_root" => {
            (true, "success")
        }
        "vc_mint" => {
            (true, "success")
        }
        "vc_spend" => {
            (true, "success")
        }
        "vc_revoke" => {
            (true, "success")
        }

        "add_key" => {
            (true, "success")
        }
        "check_delete_key" => {
            (true, "success")
        }
        "generate_mnemonic" => {
            (true, "success")
        }
        "get_logged_in_fingerprint" => {
            (true, "success")
        }
        "get_private_key" => {
            (true, "success")
        }
        "log_in" => {
            (true, "success")
        }

        "get_auto_claim" => {
            (true, "success")
        }
        "get_height_info" => {
            (true, "success")
        }
        "get_network_info" => {
            (true, "success")
        }
        "get_timestamp_for_height" => {
            (true, "success")
        }
        "push_transactions" => {
            (true, "success")
        }
        "push_tx" => {
            (true, "success")
        }
        "set_auto_claim" => {
            (true, "success")
        }
        "set_wallet_resync_on_startup" => {
            (true, "success")
        }

        "get_wallets" => {
            (true, "success")
        }

        "create_signed_transaction" => {
            (true, "success")
        }
        "delete_notifications" => {
            (true, "success")
        }
        "delete_unconfirmed_transactions" => {
            (true, "success")
        }
        "extend_derivation_index" => {
            (true, "success")
        }
        "get_coin_records" => {
            (true, "success")
        }
        "get_coin_records_by_names" => {
            (true, "success")
        }
        "get_current_derivation_index" => {
            (true, "success")
        }
        "get_farmed_amount" => {
            (true, "success")
        }
        "get_next_address" => {
            (true, "success")
        }
        "get_notifications" => {
            (true, "success")
        }
        "get_spendable_coins" => {
            (true, "success")
        }
        "get_transaction" => {
            (true, "success")
        }
        "get_transactions" => {
            (true, "success")
        }
        "get_transaction_count" => {
            (true, "success")
        }
        "get_transaction_memo" => {
            (true, "success")
        }
        "get_wallet_balance" => {
            (true, "success")
        }
        "get_wallet_balances" => {
            (true, "success")
        }
        "select_coins" => {
            (true, "success")
        }
        "send_notification" => {
            (true, "success")
        }
        "send_transaction" => {
            (true, "success")
        }
        "send_transaction_multi" => {
            (true, "success")
        }
        "sign_message_by_address" => {
            (true, "success")
        }
        "sign_message_by_id" => {
            (true, "success")
        }
        "spend_clawback_coins" => {
            (true, "success")
        }
        "verify_signature" => {
            (true, "success")
        }

        "cat_asset_id_to_name" => {
            (true, "success")
        }
        "cat_get_asset_id" => {
            (true, "success")
        }
        "cat_get_name" => {
            (true, "success")
        }
        "cat_set_name" => {
            (true, "success")
        }
        "cat_spend" => {
            (true, "success")
        }
        "get_cat_list" => {
            (true, "success")
        }
        "get_stray_cats" => {
            (true, "success")
        }

        "pw_absorb_rewards" => {
            (true, "success")
        }
        "pw_join_pool" => {
            (true, "success")
        }
        "pw_self_pool" => {
            (true, "success")
        }
        "pw_status" => {
            (true, "success")
        }

        "create_new_dl" => {
            (true, "success")
        }
        "dl_delete_mirror" => {
            (true, "success")
        }
        "dl_get_mirrors" => {
            (true, "success")
        }
        "dl_history" => {
            (true, "success")
        }
        "dl_latest_singleton" => {
            (true, "success")
        }
        "dl_new_mirror" => {
            (true, "success")
        }
        "dl_owned_singletons" => {
            (true, "success")
        }
        "dl_singletons_by_root" => {
            (true, "success")
        }
        "dl_stop_tracking" => {
            (true, "success")
        }
        "dl_track_new" => {
            (true, "success")
        }
        "dl_update_multiple" => {
            (true, "success")
        }
        "dl_update_root" => {
            (true, "success")
        }

        "get_harvesters" => {
            (true, "success")
        }
        "get_harvesters_summary" => {
            (true, "success")
        }
        "get_harvester_plots_duplicates" => {
            (true, "success")
        }
        "get_harvester_plots_invalid" => {
            (true, "success")
        }
        "get_harvester_plots_keys_missing" => {
            (true, "success")
        }
        "get_harvester_plots_valid" => {
            (true, "success")
        }
        "get_pool_login_link" => {
            (true, "success")
        }
        "get_pool_state" => {
            (true, "success")
        }
        "get_reward_targets" => {
            (true, "success")
        }
        "get_signage_point" => {
            (true, "success")
        }
        "get_signage_points" => {
            (true, "success")
        }
        "set_payout_instructions" => {
            (true, "success")
        }
        "set_reward_targets" => {
            (true, "success")
        }

        "get_additions_and_removals" => {
            (true, "success")
        }
        "get_all_mempool_items" => {
            (true, "success")
        }
        "get_all_mempool_tx_ids" => {
            (true, "success")
        }
        "get_block" => {
            (true, "success")
        }
        "get_blockchain_state" => {
            (true, "success")
        }
        "get_blocks" => {
            (true, "success")
        }
        "get_block_count_metrics" => {
            (true, "success")
        }
        "get_block_record" => {
            (true, "success")
        }
        "get_block_records" => {
            (true, "success")
        }
        "get_block_record_by_height" => {
            (true, "success")
        }
        "get_block_spends" => {
            (true, "success")
        }
        "get_block_spends_with_conditions" => {
            (true, "success")
        }
        "get_coin_records_by_hint" => {
            (true, "success")
        }
        "get_coin_records_by_parent_ids" => {
            (true, "success")
        }
        "get_coin_records_by_puzzle_hash" => {
            (true, "success")
        }
        "get_coin_records_by_puzzle_hashes" => {
            (true, "success")
        }
        "get_coin_record_by_name" => {
            (true, "success")
        }
        "get_fee_estimate" => {
            (true, "success")
        }
        "get_mempool_item_by_tx_id" => {
            (true, "success")
        }
        "get_mempool_items_by_coin_name" => {
            (true, "success")
        }
        "get_network_space" => {
            (true, "success")
        }
        "get_puzzle_and_solution" => {
            (true, "success")
        }
        "get_recent_signage_point_or_eos" => {
            (true, "success")
        }
        "get_unfinished_block_headers" => {
            (true, "success")
        }
        "healthz" => {
            (true, "success")
        }
        _ => (false, "Unknown command"),
    }
}
