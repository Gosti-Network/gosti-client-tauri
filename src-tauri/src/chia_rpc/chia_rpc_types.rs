use serde::{Deserialize, Serialize};
use std::collections::HashMap;


// DAO RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoAddFundsToTreasuryParams {
    wallet_id: u32,
    funding_wallet_id: u32,
    amount: u32,
    fee: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoAdjustFilterLevelParams {
    wallet_id: u32,
    filter_level: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoCloseProposalParams {
    wallet_id: u32,
    proposal_id: String,
    fee: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoCreateProposalParams {
    wallet_id: u32,
    proposal_type: String,
    vote_amount: Option<u32>,
    fee: Option<u32>,
    additions: Option<Vec<Addition>>,
    asset_id: Option<String>,
    inner_address: Option<String>,
    new_dao_rules: Option<Vec<NewDaoRule>>,
    amount: Option<u32>,
    cat_target_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Addition {
    asset_id: Option<String>,
    puzzle_hash: String,
    amount: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewDaoRule {
    proposal_timelock: Option<u32>,
    soft_close_length: Option<u32>,
    attendance_required: Option<u32>,
    pass_percentage: Option<u32>,
    self_destruct_length: Option<u32>,
    oracle_spend_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoExitLockupParams {
    wallet_id: u32,
    coins: Option<Vec<String>>,
    fee: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoFreeCoinsFromFinishedProposalsParams {
    wallet_id: u32,
    fee: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoGetProposalsParams {
    wallet_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoGetProposalStateParams {
    wallet_id: u32,
    proposal_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoGetRulesParams {
    wallet_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoGetTreasuryBalanceParams {
    wallet_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoGetTreasuryIdParams {
    wallet_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoParseProposalParams {
    wallet_id: u32,
    proposal_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoSendToLockupParams {
    wallet_id: u32,
    amount: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DaoVoteOnProposalParams {
    wallet_id: u32,
    proposal_id: String,
    vote_amount: Option<u32>,
    is_yes_vote: bool,
    fee: Option<u32>,
}


// Daemon Server RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ExitParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetRoutesParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetStatusParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetVersionParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetWalletAddressesParams {
    fingerprints: Option<u32>,
    index: Option<u32>,
    count: Option<u32>,
    non_observer_derivation: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IsKeyringLockedParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IsRunningParams {
    service: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct KeyringStatusParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RegisterServiceParams {
    service: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RemoveKeyringPassphraseParams {
    current_passphrase: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RunningServicesParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetKeyringPassphraseParams {
    current_passphrase: String,
    new_passphrase: String,
    passphrase_hint: Option<String>,
    save_passphrase: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StartServiceParams {
    service: String,
    testing: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StopServiceParams {
    service: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UnlockKeyringParams {
    key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ValidateKeyringPassphraseParams {
    key: String,
}


// Daemon Plotter RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKeysForPlottingParams {
    fingerprints: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPlottersParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StartPlottingParams {
    service: String,
    plotter: Option<String>,
    delay: Option<u32>,
    parallel: Option<bool>,
    k: u32,
    override_k: bool,
    r: u32,
    b: u32,
    u: u32,
    e: bool,
    t: String,
    d: String,
    x: bool,
    n: Option<u32>,
    queue: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StopPlottingParams {
    id: String,
}


// Daemon Keychain Server RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddPrivateKeyParams {
    mnemonic: String,
    label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CheckKeysParams {
    root_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteAllKeysParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteKeyByFingerprintParams {
    fingerprint: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllPrivateKeysParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetFirstPrivateKeyParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKeyForFingerprintParams {
    fingerprint: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKeyParams {
    fingerprint: u32,
    include_secrets: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKeysParams {
    include_secrets: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPublicKeyParams {
    fingerprint: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPublicKeysParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetLabelParams {
    fingerprint: u32,
    label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteLabelParams {
    fingerprint: u32,
}


// Datalayer RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddMirrorParams {
    id: String,
    urls: Vec<String>,
    amount: i64,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddMissingFilesParams {
    ids: Option<Vec<String>>,
    _override: Option<bool>,
    foldername: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BatchUpdateParams {
    id: String,
    changelist: String,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CheckPluginsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ClearPendingRootsParams {
    store_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateDataStoreParams {
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteKeyParams {
    id: String,
    key: String,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteMirrorParams {
    id: String,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAncestorsParams {
    id: String,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKeysParamsDL {
    id: String,
    root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKeysValuesParams {
    id: String,
    root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetKvDiffParams {
    id: String,
    root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetLocalRootParams {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetMirrorsParams {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetOwnedStoresParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetRootParams {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetRootHistoryParams {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetSyncStatusParams {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetValueParams {
    id: String,
    key: String,
    root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InsertParams {
    id: String,
    key: String,
    value: String,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MakeOfferParams {
    maker: String,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RemoveSubscriptionsParams {
    id: String,
    urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubscribeParams {
    id: String,
    urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TakeOfferParams {
    offer: Option<String>,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UnsubscribeParams {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UpdateDataStoreParams {
    store_id: String,
    changelist: Vec<serde_json::Value>,
    fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VerifyOfferParams {
    offer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WalletLogInParams {
    fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CloseConnectionParams {
    node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenConnectionParams {
    host: String,
    port: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StopNodeParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetConnectionsParams {}


// DID RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateNewWalletParamsDID {
    wallet_type: String,
    did_type: String,
    amount: u64,
    backup_dids: Vec<String>,
    num_of_backup_ids_needed: u64,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidCreateAttestParams {
    wallet_id: String,
    coin_name: String,
    pubkey: String,
    puzhash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidCreateBackupFile {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidFindLostDidParams {
    coin_id: String,
    recover_list_hash: String,
    num_verification: u64,
    metadata: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetCurrentCoinInfoParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetDidParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetInfoParams {
    coin_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetInformationNeededForRecoveryParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetMetadataParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetPubkeyParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetRecoveryListParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidGetWalletNameParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidMessageSpendParams {
    wallet_id: String,
    coin_announcements: Vec<String>,
    puzzle_announcements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidRecoverySpendParams {
    wallet_id: String,
    attest_data: Vec<String>,
    pubkey: Option<String>,
    puzhash: Option<String>,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidSetWalletNameParams {
    wallet_id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidTransferDidParams {
    wallet_id: String,
    inner_address: String,
    fee: Option<u64>,
    with_recovery_info: Option<bool>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DidUpdateMetadataParams {
    wallet_id: String,
    metadata: Option<String>,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}


// NFT RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftAddUriParams {
    wallet_id: u64,
    nft_coin_id: String,
    key: String,
    uri: String,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftCalculateRoyaltiesParams {
    royalty_assets: Option<Vec<RoyaltyAsset>>,
    fungible_assets: Option<Vec<FungibleAsset>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RoyaltyAsset {
    asset: String,
    royalty_address: String,
    royalty_percentage: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FungibleAsset {
    asset: String,
    amount: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftCountNftsParams {
    wallet_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftGetByDidParams {
    did_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftGetInfoParams {
    coin_id: u64,
    wallet_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftGetNftsParams {
    wallet_id: u64,
    start_index: Option<u64>,
    num: Option<u64>,
    ignore_size_limit: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftGetWalletsWithDidsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftGetWalletDidParams {
    wallet_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftMintBulkParams {
    wallet_id: u64,
    metadata_list: Vec<String>,
    royalty_percentage: Option<u64>,
    royalty_address: Option<String>,
    target_list: Option<Vec<String>>,
    mint_number_start: Option<u64>,
    mint_total: Option<u64>,
    xch_coin_list: Option<Vec<String>>,
    xch_change_target: Option<String>,
    new_innerpuzhash: Option<String>,
    new_p2_puzhash: Option<String>,
    did_coin_dict: Option<serde_json::Value>,
    did_lineage_parent_hex: Option<String>,
    mint_from_did: Option<bool>,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftMintNftParams {
    wallet_id: u64,
    uris: Vec<String>,
    hash: String,
    did_id: Option<String>,
    meta_uris: Option<Vec<String>>,
    meta_hash: Option<String>,
    license_uris: Option<Vec<String>>,
    license_hash: Option<String>,
    royalty_address: Option<String>,
    royalty_percentage: Option<u64>,
    target_address: Option<String>,
    edition_number: Option<u64>,
    edition_total: Option<u64>,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftSetDidBulkParams {
    nft_coin_list: Vec<NftCoinList>,
    did_id: Option<String>,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftCoinList {
    nft_coin_id: String,
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftSetNftDidParams {
    wallet_id: u64,
    nft_coin_id: u64,
    did_id: Option<u64>,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftSetNftStatusParams {
    wallet_id: u64,
    coin_id: u64,
    in_transaction: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftTransferBulkParams {
    nft_coin_list: Vec<NftCoinList>,
    target_address: String,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NftTransferNftParams {
    wallet_id: u64,
    target_address: String,
    nft_coin_id: String,
    fee: Option<u64>,
    reuse_puzhash: Option<bool>,
}


// Offer RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CancelOfferParams {
    trade_id: String,
    fee: Option<u64>,
    secure: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CancelOffersParams {
    secure: bool,
    fee: Option<u64>,
    batch_size: Option<u64>,
    cancel_all: Option<bool>,
    asset_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CheckOfferValidityParams {
    offer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateOfferForIdsParams {
    offer: HashMap<String, i32>,
    fee: Option<u64>,
    validate_only: Option<bool>,
    driver_dict: Option<HashMap<String, serde_json::Value>>,
    min_coin_amount: Option<u64>,
    max_coin_amount: Option<u64>,
    solver: Option<serde_json::Value>,
    min_height: Option<u64>,
    min_time: Option<u64>,
    max_height: Option<u64>,
    max_time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllOffersParams {
    start: Option<u64>,
    end: Option<u64>,
    exclude_my_offers: Option<bool>,
    exclude_taken_offers: Option<bool>,
    include_completed: Option<bool>,
    sort_key: Option<String>,
    reverse: Option<bool>,
    file_contents: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetOfferParams {
    trade_id: String,
    file_contents: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetOffersCountParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetOfferSummaryParams {
    offer: String,
    advanced: Option<bool>,
}


// Simulator RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FarmBlockParams {
    address: String,
    guarantee_tx_block: Option<bool>,
    blocks: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllBlocksParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllCoinsParams {
    include_spent_coins: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllPuzzleHashesParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAutoFarmingParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetFarmingPhParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ReorgBlocksParams {
    num_of_blocks_to_rev: Option<u64>,
    num_of_new_blocks: Option<u64>,
    revert_all_blocks: Option<bool>,
    random_seed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RevertBlocksParams {
    num_of_blocks: Option<u64>,
    delete_all_blocks: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetAutoFarmingParams {
    auto_farm: bool,
}



// Verifiable Credentials RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcAddProofsParams {
    proofs: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcGetParams {
    vc_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcGetListParams {
    start: Option<i32>,
    count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcGetProofsForRootParams {
    root: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcMintParams {
    did_id: String,
    target_address: Option<String>,
    fee: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcSpendParams {
    vc_id: String,
    new_puzhash: Option<String>,
    new_proof_hash: Option<String>,
    provider_inner_puzhash: Option<String>,
    reuse_puzhash: Option<bool>,
    fee: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VcRevokeParams {
    vc_parent_id: String,
    reuse_puzhash: Option<bool>,
    fee: Option<i32>,
}


// Wallet - Key Management RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddKeyParams {
    mnemonic: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CheckDeleteKeyParams {
    fingerprint: u64,
    max_ph_to_search: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GenerateMnemonicParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetLoggedInFingerprintParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPrivateKeyParams {
    fingerprint: u64,
}



// Wallet - Wallet Node RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAutoClaimParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHeightInfoParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetNetworkInfoParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetTimestampForHeightParams {
    height: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PushTransactionsParams {
    transactions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PushTxParams {
    spend_bundle: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetAutoClaimParams {
    enabled: bool,
    tx_fee: u64,
    min_amount: u64,
    batch_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetWalletResyncOnStartupParams {
    enable: bool,
}


// Wallet - Wallet Management RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateNewWalletParams {
    wallet_type: String,
    did_type: String,
    amount: u64,
    backup_dids: Vec<String>,
    num_of_backup_ids_needed: u64,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetWalletsParams {
    include_data: bool,
    wallet_type: u64,
}


// Wallet - Wallet RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateSignedTransactionParams {
    wallet_id: String,
    additions: Vec<String>,
    min_coin_amount: Option<u64>,
    max_coin_amount: Option<u64>,
    exclude_coin_amounts: Option<Vec<u64>>,
    coins: Vec<String>,
    exclude_coins: Vec<String>,
    coin_announcements: Option<Vec<String>>,
    puzzle_announcements: Option<Vec<String>>,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteNotificationsParams {
    ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeleteUnconfirmedTransactionsParams {
    wallet_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ExtendDerivationIndexParams {
    index: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordsByNamesParams {
    names: Vec<String>,
    start_height: Option<u64>,
    end_height: Option<u64>,
    include_spent_coins: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SelectCoinsParams {
    wallet_id: u64,
    amount: u64,
    min_coin_amount: Option<u64>,
    max_coin_amount: Option<u64>,
    excluded_coin_amounts: Option<Vec<u64>>,
    excluded_coins: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCurrentDerivationIndexParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetFarmedAmountParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetNextAddressParams {
    wallet_id: String,
    new_address: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetNotificationsParams {
    ids: Option<Vec<String>>,
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetSpendableCoinsParams {
    wallet_id: u64,
    min_coin_amount: Option<u64>,
    max_coin_amount: Option<u64>,
    excluded_coin_amounts: Option<Vec<u64>>,
    excluded_coins: Option<Vec<String>>,
    excluded_coin_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetTransactionParams {
    transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetTransactionsParams {
    wallet_id: u64,
    start: Option<u64>,
    end: Option<u64>,
    sort_key: Option<u64>,
    reverse: Option<bool>,
    to_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetTransactionCountParams {
    wallet_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetTransactionMemoParams {
    transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetWalletBalanceParams {
    wallet_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetWalletBalancesParams {
    wallet_ids: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SendNotificationParams {
    target: String,
    message: String,
    amount: u64,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SendTransactionParams {
    wallet_id: String,
    address: String,
    amount: u64,
    fee: Option<u64>,
    memos: Option<Vec<String>>,
    min_coin_amount: Option<u64>,
    max_coin_amount: Option<u64>,
    exclude_coin_amounts: Option<Vec<u64>>,
    exclude_coin_ids: Option<Vec<String>>,
    reuse_puzhash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SendTransactionMultiParams {
    wallet_id: String,
    address: String,
    amount: u64,
    fee: Option<u64>,
    memos: Option<Vec<String>>,
    additions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SignMessageByAddressParams {
    address: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SignMessageByIdParams {
    id: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SpendClawbackCoinsParams {
    coin_ids: Vec<String>,
    batch_size: Option<u64>,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VerifySignatureParams {
    signing_mode: Option<String>,
    pubkey: String,
    message: String,
    signature: String,
    address: String,
}


// Wallet - CATS & Trading RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CatAssetIdToNameParams {
    asset_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CatGetAssetIdParams {
    wallet_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CatGetNameParams {
    wallet_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CatSetNameParams {
    wallet_id: u64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCatListParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CatSpendParams {
    wallet_id: String,
    additions: Vec<String>,
    amount: u64,
    inner_address: String,
    memos: Option<Vec<String>>,
    coins: Option<Vec<String>>,
    min_coin_amount: Option<u64>,
    max_coin_amount: Option<u64>,
    exclude_coin_amounts: Option<Vec<u64>>,
    exclude_coin_ids: Option<Vec<String>>,
    fee: Option<u64>,
    extra_delta: Option<String>,
    tail_reveal: Option<String>,
    tail_solution: Option<String>,
    reuse_puzhash: Option<bool>,
}

// Wallet - Pool Wallet RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PwAbsorbRewardsParams {
    wallet_id: u64,
    max_spends_in_tx: Option<u64>,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PwJoinPoolParams {
    wallet_id: u64,
    target_puzzlehash: String,
    pool_url: String,
    relative_lock_height: u64,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PwSelfPoolParams {
    wallet_id: u64,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PwStatusParams {
    wallet_id: u64,
}


// Wallet - DataLayer Wallet RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateNewDlParams {
    root: String,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlDeleteMirrorParams {
    coin_id: String,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlGetMirrorsParams {
    launcher_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlHistoryParams {
    launcher_id: String,
    min_generation: Option<u64>,
    max_generation: Option<u64>,
    num_results: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlLatestSingletonParams {
    launcher_id: String,
    only_confirmed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlNewMirrorParams {
    launcher_id: String,
    urls: Vec<String>,
    amount: u64,
    fee: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlOwnedSingletonsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlSingletonsByRootParams {
    launcher_id: String,
    root: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlStopTrackingParams {
    launcher_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlTrackNewParams {
    launcher_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlUpdateMultipleParams {
    updates: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DlUpdateRootParams {
    launcher_id: String,
    new_root: String,
    fee: Option<u64>,
}


// Farmer RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHarvestersParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHarvestersSummaryParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHarvesterPlotsDuplicatesParams {
    node_id: String,
    page: u32,
    page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHarvesterPlotsInvalidParams {
    node_id: String,
    page: u32,
    page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHarvesterPlotsKeysMissingParams {
    node_id: String,
    page: u32,
    page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetHarvesterPlotsValidParams {
    node_id: String,
    page: u32,
    page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPoolLoginLinkParams {
    launcher_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPoolStateParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetRewardTargetsParams {
    search_for_private_key: bool,
    max_ph_to_search: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetSignagePointParams {
    sp_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetSignagePointsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetPayoutInstructionsParams {
    launcher_id: String,
    payout_instructions: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetRewardTargetsParams {
    farmer_target: Option<String>,
    pool_target: Option<String>,
}


// Full Node RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlocksParams {
    start: i32,
    end: i32,
    exclude_header_hash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAdditionsAndRemovalsParams {
    header_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllMempoolItemsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetAllMempoolTxIdsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockParams {
    header_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockchainStateParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockSpendsParams {
    header_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockSpendsWithConditionsParams {
    header_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockCountMetricsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockRecordParams {
    header_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockRecordsParams {
    start: i32,
    end: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetBlockRecordByHeightParams {
    height: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordsByParentIdsParams {
    parent_ids: Vec<String>,
    start_height: Option<i32>,
    end_height: Option<i32>,
    include_spent_coins: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordsByPuzzleHashParams {
    puzzle_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordsByHintParams {
    hint: String,
    start_height: Option<i32>,
    end_height: Option<i32>,
    include_spent_coins: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetFeeEstimateParams {
        spend_bundle: Option<String>,
        cost: Option<i32>,
        target_times: Vec<i32>,
    }

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetMempoolItemByTxIdParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordsByPuzzleHashesParams {
    puzzle_hashes: Vec<String>,
    start_height: Option<i32>,
    end_height: Option<i32>,
    include_spent_coins: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetCoinRecordByNameParams {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetMempoolItemsByCoinNameParams {
    coin_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetNetworkSpaceParams {
    older_block_header_hash: String,
    newer_block_header_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPuzzleAndSolutionParams {
    coin_id: String,
    height: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetRecentSignagePointOrEosParams {
    sp_hash: String,
    challenge_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetUnfinishedBlockHeadersParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct HealthzParams {}



// Harvester RPC command Params
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddPlotDirectoryParams {
    dirname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DeletePlotParams {
    filename: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetPlotsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RefreshPlotsParams {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RemovePlotDirectoryParams {
    dirname: String,
}

pub struct CommandDetails<'a> {
    port: &'a str,
    ssl_cert_path: &'a str,
    ssl_key_path: &'a str,
}



pub const AVAILABLE_COMMANDS: phf::Map<&'static str, &'static CommandDetails> = phf::phf_map! {

    "dao_add_funds_to_treasury" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_adjust_filter_level" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_close_proposal" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_create_proposal" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_exit_lockup" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_free_coins_from_finished_proposals" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_get_proposals" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_get_proposal_state" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_get_rules" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_get_treasury_balance" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_get_treasury_id" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_parse_proposal" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_send_to_lockup" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dao_vote_on_proposal" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "exit" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_routes" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_status" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_version" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_wallet_addresses" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "is_keyring_locked" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "is_running" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "keyring_status" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "register_service" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "remove_keyring_passphrase" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "running_services" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "set_keyring_passphrase" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "start_service" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "stop_service" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "unlock_keyring" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "validate_keyring_passphrase" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},

    "get_keys_for_plotting" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_plotters" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "start_plotting" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "stop_plotting" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},

    "add_private_key" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "check_keys" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "delete_all_keys" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "delete_key_by_fingerprint" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_all_private_keys" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_first_private_key" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_key_for_fingerprint" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_key" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_keys" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_public_key" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "get_public_keys" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "set_label" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},
    "delete_label" => &CommandDetails {port: "55400", ssl_cert_path: "ssl/daemon/private_daemon.crt", ssl_key_path: "ssl/daemon/private_daemon.key"},

    "add_mirror" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "add_missing_files" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "batch_update" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "cancel_offer" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "check_plugins" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "clear_pending_roots" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "create_data_store" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "delete_key" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "delete_mirror" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_ancestors" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_keys_values" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_kv_diff" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_local_root" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_mirrors" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_owned_stores" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_root" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_roots" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_root_history" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_sync_status" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_value" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "insert" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "make_offer" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "remove_subscriptions" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "subscribe" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "subscriptions" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "take_offer" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "unsubscribe" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "update_data_store" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "verify_offer" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "wallet_log_in" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},

    "close_connection" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "get_connections" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "open_connection" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},
    "stop_node" => &CommandDetails {port: "8562", ssl_cert_path: "ssl/data_layer/private_data_layer.crt", ssl_key_path: "ssl/data_layer/private_data_layer.key"},

    "create_new_wallet" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_create_attest" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_create_backup_file" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_find_lost_did" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_current_coin_info" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_did" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_info" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_information_needed_for_recovery" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_metadata" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_pubkey" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_recovery_list" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_get_wallet_name" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_message_spend" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_recovery_spend" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_set_wallet_name" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_transfer_did" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_update_metadata" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "did_update_recovery_ids" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "nft_add_uri" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_calculate_royalties" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_count_nfts" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_get_by_did" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_get_info" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_get_nfts" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_get_wallets_with_dids" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_get_wallet_did" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_mint_bulk" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_mint_nft" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_set_did_bulk" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_set_nft_did" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_set_nft_status" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_transfer_bulk" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "nft_transfer_nft" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "check_offer_validity" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "create_offer_for_ids" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_all_offers" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_offer" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_offers_count" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_offer_summary" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "farm_block" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_all_blocks" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_all_coins" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_all_puzzle_hashes" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_auto_farming" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_farming_ph" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "reorg_blocks" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "revert_blocks" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "set_auto_farming" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},

    "vc_add_proofs" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "vc_get" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "vc_get_list" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "vc_get_proofs_for_root" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "vc_mint" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "vc_spend" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "vc_revoke" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "add_key" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "check_delete_key" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "generate_mnemonic" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_logged_in_fingerprint" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_private_key" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "log_in" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "get_auto_claim" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_height_info" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_network_info" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_timestamp_for_height" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "push_transactions" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "push_tx" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "set_auto_claim" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "set_wallet_resync_on_startup" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "get_wallets" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "create_signed_transaction" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "delete_notifications" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "delete_unconfirmed_transactions" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "extend_derivation_index" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_coin_records" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_coin_records_by_names" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_current_derivation_index" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_farmed_amount" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_next_address" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_notifications" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_spendable_coins" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_transaction" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_transactions" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_transaction_count" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_transaction_memo" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_wallet_balance" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_wallet_balances" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "select_coins" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "send_notification" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "send_transaction" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "send_transaction_multi" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "sign_message_by_address" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "sign_message_by_id" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "spend_clawback_coins" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "verify_signature" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "cancel_offers" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "cat_asset_id_to_name" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "cat_get_asset_id" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "cat_get_name" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "cat_set_name" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "cat_spend" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_cat_list" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "get_stray_cats" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "pw_absorb_rewards" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "pw_join_pool" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "pw_self_pool" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "pw_status" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "create_new_dl" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_delete_mirror" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_get_mirrors" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_history" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_latest_singleton" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_new_mirror" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_owned_singletons" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_singletons_by_root" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_stop_tracking" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_track_new" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_update_multiple" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},
    "dl_update_root" => &CommandDetails {port: "9256", ssl_cert_path: "ssl/wallet/private_wallet.crt", ssl_key_path: "ssl/wallet/private_wallet.key"},

    "get_harvesters" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_harvesters_summary" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_harvester_plots_duplicates" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_harvester_plots_invalid" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_harvester_plots_keys_missing" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_harvester_plots_valid" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_pool_login_link" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_pool_state" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_reward_targets" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_signage_point" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "get_signage_points" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "set_payout_instructions" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},
    "set_reward_targets" => &CommandDetails {port: "8559", ssl_cert_path: "ssl/farmer/private_farmer.crt", ssl_key_path: "ssl/farmer/private_farmer.key"},

    "get_additions_and_removals" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_all_mempool_items" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_all_mempool_tx_ids" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_blockchain_state" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_blocks" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block_count_metrics" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block_record" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block_records" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block_record_by_height" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block_spends" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_block_spends_with_conditions" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_coin_records_by_hint" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_coin_records_by_parent_ids" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_coin_records_by_puzzle_hash" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_coin_records_by_puzzle_hashes" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_coin_record_by_name" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_fee_estimate" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_mempool_item_by_tx_id" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_mempool_items_by_coin_name" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_network_space" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_puzzle_and_solution" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_recent_signage_point_or_eos" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "get_unfinished_block_headers" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},
    "healthz" => &CommandDetails {port: "8555", ssl_cert_path: "ssl/full_node/private_full_node.crt", ssl_key_path: "ssl/full_node/private_full_node.key"},

    "add_plot_directory" => &CommandDetails {port: "8560", ssl_cert_path: "ssl/harvester/private_harvester.crt", ssl_key_path: "ssl/harvester/private_harvester.key"},
    "delete_plot" => &CommandDetails {port: "8560", ssl_cert_path: "ssl/harvester/private_harvester.crt", ssl_key_path: "ssl/harvester/private_harvester.key"},
    "get_plots" => &CommandDetails {port: "8560", ssl_cert_path: "ssl/harvester/private_harvester.crt", ssl_key_path: "ssl/harvester/private_harvester.key"},
    "get_plot_directories" => &CommandDetails {port: "8560", ssl_cert_path: "ssl/harvester/private_harvester.crt", ssl_key_path: "ssl/harvester/private_harvester.key"},
    "refresh_plots" => &CommandDetails {port: "8560", ssl_cert_path: "ssl/harvester/private_harvester.crt", ssl_key_path: "ssl/harvester/private_harvester.key"},
    "remove_plot_directory" => &CommandDetails {port: "8560", ssl_cert_path: "ssl/harvester/private_harvester.crt", ssl_key_path: "ssl/harvester/private_harvester.key"},
};

pub(crate) fn get_port_number(command: &str) -> &str {
    AVAILABLE_COMMANDS.get(command).map(|value| value.port).unwrap_or_default()
}

pub(crate) fn get_ssl_cert_path(command: &str) -> &str {
    AVAILABLE_COMMANDS.get(command).map(|value| value.ssl_cert_path).unwrap_or_default()
}

pub(crate) fn get_ssl_key_path(command: &str) -> &str {
    AVAILABLE_COMMANDS.get(command).map(|value| value.ssl_key_path).unwrap_or_default()
}

pub(crate) fn is_valid_command(command: &str) -> bool {
    AVAILABLE_COMMANDS.contains_key(command)
}
