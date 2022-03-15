#pragma once
#include "stdint.h"
#include "vm_context.h"
#include "vm_ext.h"
#include "vm_memory.h"

#include <map>
#include <vector>
class vm_logic {
public:
    vm_logic() = default;
    vm_logic(vm_context const & context, vm_ext const & ext);

public:
    // interface to vm_import_instance:

    // register:
    void read_register(uint64_t register_id, uint64_t ptr);
    uint64_t register_len(uint64_t register_id);

    // context:
    // void current_account_id(uint64_t register_id);
    void signer_account_id(uint64_t register_id);
    void predecessor_account_id(uint64_t register_id);
    void input(uint64_t register_id);
    void account_balance(uint64_t balance_ptr);

    // math:
    void random_seed(uint64_t register_id);
    void sha256(uint64_t value_len, uint64_t value_ptr, uint64_t register_id);
    void keccak256(uint64_t value_len, uint64_t value_ptr, uint64_t register_id);
    void ripemd160(uint64_t value_len, uint64_t value_ptr, uint64_t register_id);

    // others:
    void value_return(uint64_t value_len, uint64_t value_ptr);
    void log_utf8(uint64_t len, uint64_t ptr);

    // storage:
    uint64_t storage_write(uint64_t key_len, uint64_t key_ptr, uint64_t value_len, uint64_t value_ptr, uint64_t register_id);
    uint64_t storage_read(uint64_t key_len, uint64_t key_ptr, uint64_t register_id);
    uint64_t storage_remove(uint64_t key_len, uint64_t key_ptr, uint64_t register_id);

private:
    vm_context m_context;
    vm_ext m_ext;  // todo make storage a shared_ptr. more real.
    std::map<uint64_t, std::vector<uint8_t>> m_registers;
    vm_memory m_memory;
    uint128_t current_account_balance;
    std::vector<uint8_t> m_return_data_value;

public:
    vm_ext & ext_ref();
    vm_context & context_ref();
    std::vector<uint8_t> return_value();

private:
    // inner api

    std::string get_utf8_string(uint64_t len, uint64_t ptr);

    void internal_write_register(uint64_t register_id, std::vector<uint8_t> const & context_input);

    std::vector<uint8_t> get_vec_from_memory_or_register(uint64_t offset, uint64_t len);

    void memory_set_slice(uint64_t offset, std::vector<uint8_t> buf);

    std::vector<uint8_t> memory_get_vec(uint64_t offset, uint64_t len);

    std::vector<uint8_t> internal_read_register(uint64_t register_id);
};