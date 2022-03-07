#include "stdint.h"
#include "stdio.h"
#include "vm_import_logic.h"

static vm_logic vl;

extern "C" {
// #############
// # Registers #
// #############
void read_register(uint64_t register_id, uint64_t ptr) {
    vl.read_register(register_id, ptr);
    return;
}
uint64_t register_len(uint64_t register_id) {
    return vl.register_len(register_id);
}
// ###############
// # Context API #
// ###############
void current_account_id(uint64_t register_id) {
    vl.current_account_id(register_id);
    return;
}
void signer_account_id(uint64_t register_id) {
    return;
}
void signer_account_pk(uint64_t register_id) {
    return;
}
void predecessor_account_id(uint64_t register_id) {
    vl.predecessor_account_id(register_id);
    return;
}
void input(uint64_t register_id) {
    vl.input(register_id);
    return;
}
uint64_t block_index() {
    return 1;
}
uint64_t block_timestamp() {
    return 1;
}
uint64_t epoch_height() {
    return 1;
}
uint64_t storage_usage() {
    return 1;
}
// #################
// # Economics API #
// #################
void account_balance(uint64_t balance_ptr) {
    vl.account_balance(balance_ptr);
    return;
}
void attached_deposit(uint64_t balance_ptr) {
    return;
}
uint64_t prepaid_gas() {
    return 1;
}
uint64_t used_gas() {
    return 1;
}
// ############
// # Math API #
// ############
void random_seed(uint64_t register_id) {
    return;
}
void sha256(uint64_t value_len, uint64_t value_ptr, uint64_t register_id) {
    return;
}
void keccak256(uint64_t value_len, uint64_t value_ptr, uint64_t register_id) {
    vl.keccak256(value_len, value_ptr, register_id);
    return;
}
void ripemd160(uint64_t value_len, uint64_t value_ptr, uint64_t register_id) {
    return;
}
uint64_t ecrecover(uint64_t hash_len, uint64_t hash_ptr, uint64_t sig_len, uint64_t sig_ptr, uint64_t v, uint64_t malleability_flag, uint64_t register_id) {
    return 1;
}
// #####################
// # Miscellaneous API #
// #####################
void value_return(uint64_t value_len, uint64_t value_ptr) {
    return vl.value_return(value_len, value_ptr);
}
void panic() {
    return;
}
void panic_utf8(uint64_t len, uint64_t ptr) {
    return;
}
void log_utf8(uint64_t len, uint64_t ptr) {
    return;
}
void log_utf16(uint64_t len, uint64_t ptr) {
    return;
}
// void abort(uint32_t msg_ptr, uint32_t filename_ptr, uint32_t line, uint32_t col) { return; }
// ################
// # Promises API #
// ################
uint64_t promise_create(uint64_t account_id_len,
                        uint64_t account_id_ptr,
                        uint64_t method_name_len,
                        uint64_t method_name_ptr,
                        uint64_t arguments_len,
                        uint64_t arguments_ptr,
                        uint64_t amount_ptr,
                        uint64_t gas) {
    return 1;
}
uint64_t promise_then(uint64_t promise_index,
                      uint64_t account_id_len,
                      uint64_t account_id_ptr,
                      uint64_t method_name_len,
                      uint64_t method_name_ptr,
                      uint64_t arguments_len,
                      uint64_t arguments_ptr,
                      uint64_t amount_ptr,
                      uint64_t gas) {
    return 1;
}
uint64_t promise_and(uint64_t promise_idx_ptr, uint64_t promise_idx_count) {
    return 1;
}
uint64_t promise_batch_create(uint64_t account_id_len, uint64_t account_id_ptr) {
    return 1;
}
uint64_t promise_batch_then(uint64_t promise_index, uint64_t account_id_len, uint64_t account_id_ptr) {
    return 1;
}
// #######################
// # Promise API actions #
// #######################
void promise_batch_action_create_account(uint64_t promise_index) {
    return;
}
void promise_batch_action_deploy_contract(uint64_t promise_index, uint64_t code_len, uint64_t code_ptr) {
    return;
}
void promise_batch_action_function_call(uint64_t promise_index,
                                        uint64_t method_name_len,
                                        uint64_t method_name_ptr,
                                        uint64_t arguments_len,
                                        uint64_t arguments_ptr,
                                        uint64_t amount_ptr,
                                        uint64_t gas) {
    return;
}
void promise_batch_action_transfer(uint64_t promise_index, uint64_t amount_ptr) {
    return;
}
void promise_batch_action_stake(uint64_t promise_index, uint64_t amount_ptr, uint64_t public_key_len, uint64_t public_key_ptr) {
    return;
}
void promise_batch_action_add_key_with_full_access(uint64_t promise_index, uint64_t public_key_len, uint64_t public_key_ptr, uint64_t nonce) {
    return;
}
void promise_batch_action_add_key_with_function_call(uint64_t promise_index,
                                                     uint64_t public_key_len,
                                                     uint64_t public_key_ptr,
                                                     uint64_t nonce,
                                                     uint64_t allowance_ptr,
                                                     uint64_t receiver_id_len,
                                                     uint64_t receiver_id_ptr,
                                                     uint64_t method_names_len,
                                                     uint64_t method_names_ptr) {
    return;
}
void promise_batch_action_delete_key(uint64_t promise_index, uint64_t public_key_len, uint64_t public_key_ptr) {
    return;
}
void promise_batch_action_delete_account(uint64_t promise_index, uint64_t beneficiary_id_len, uint64_t beneficiary_id_ptr) {
    return;
}
// #######################
// # Promise API results #
// #######################
uint64_t promise_results_count() {
    return 1;
}
uint64_t promise_result(uint64_t result_idx, uint64_t register_id) {
    return 1;
}
void promise_return(uint64_t promise_id) {
    return;
}
// ###############
// # Storage API #
// ###############
uint64_t storage_write(uint64_t key_len, uint64_t key_ptr, uint64_t value_len, uint64_t value_ptr, uint64_t register_id) {
    return 1;
}
uint64_t storage_read(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) {
    return vl.storage_read(key_len, key_ptr, register_id);
}
uint64_t storage_remove(uint64_t key_len, uint64_t key_ptr, uint64_t register_id) {
    return 1;
}
uint64_t storage_has_key(uint64_t key_len, uint64_t key_ptr) {
    return 1;
}
uint64_t storage_iter_prefix(uint64_t prefix_len, uint64_t prefix_ptr) {
    return 1;
}
uint64_t storage_iter_range(uint64_t start_len, uint64_t start_ptr, uint64_t end_len, uint64_t end_ptr) {
    return 1;
}
uint64_t storage_iter_next(uint64_t iterator_id, uint64_t key_register_id, uint64_t value_register_id) {
    return 1;
}
// ###############
// # Validator API #
// ###############
void validator_stake(uint64_t account_id_len, uint64_t account_id_ptr, uint64_t stake_ptr) {
    return;
}
void validator_total_stake(uint64_t stake_ptr) {
    return;
}
}