#pragma once
#include "stdint.h"
#include "vm_logic.h"

class vm_import_instance {
public:
    static vm_import_instance * instance();

private:
    vm_import_instance() {
    }
    ~vm_import_instance() {
    }

    vm_logic m_vm_logic;

public:
    void set_vm_logic(vm_logic & vm_logic);
    vm_logic & get_vm_logic_ref();

public:
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
};
