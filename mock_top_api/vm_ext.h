#pragma once
#include "vm_util.h"

#include <map>
#include <string>

// should be a vitrual base class `External` in C++
// Trait: `runtime/near-vm-logic/src/dependencies.rs`
// Impl: `runtime/runtime/src/ext.rs`
class vm_ext {
public:
    vm_ext() {
        ext_kv_datas.insert(
            {hex_string_to_bytes("0x01005354415445"),
             hex_string_to_bytes(
                 "0x000000000000000000000000000000000000000000000000000000004e454154060000006175726f7261120000006272696467655f70726f7665722e6e6561720100000000000000")});
    }
    std::vector<uint8_t> storage_get(std::vector<uint8_t> const & key) {
        auto res = ext_kv_datas[key];

        return res;
    }
    void storage_set(std::vector<uint8_t> const & key, std::vector<uint8_t> const & value) {
        ext_kv_datas[key] = value;
    }

    void storage_remove(std::vector<uint8_t> const & key) {
        ext_kv_datas.erase(key);
    }

private:
    std::map<std::vector<uint8_t>, std::vector<uint8_t>> ext_kv_datas;
};