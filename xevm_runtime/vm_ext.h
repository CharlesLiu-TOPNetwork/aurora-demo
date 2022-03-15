#pragma once
#include "vm_util.h"

#include <map>
#include <string>

enum class ext_type {
    Config = 0x0,
    Nonce = 0x1,
    Balance = 0x2,
    Code = 0x3,
    Storage = 0x4,
    Generation = 0x5,

    ALL = 0x6  // show all
};

// should be a vitrual base class `External` in C++
// Trait: `runtime/near-vm-logic/src/dependencies.rs`
// Impl: `runtime/runtime/src/ext.rs`
class vm_ext {
public:
    vm_ext() {
        // ext_kv_datas.insert(
        //     {hex_string_to_bytes("0x01005354415445"),
        //      hex_string_to_bytes(
        //          "0x000000000000000000000000000000000000000000000000000000004e454154060000006175726f7261120000006272696467655f70726f7665722e6e6561720100000000000000")});
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

    void debug(ext_type const & debug_type = ext_type::ALL) {
        for (auto & pair : ext_kv_datas) {
            if (debug_type != ext_type::ALL && ext_type(pair.first[1]) != debug_type) {
                continue;
            }
            switch (ext_type(pair.first[1])) {
            case ext_type::Nonce:
                printf("[key - nonce]: ");
                break;
            case ext_type::Balance:
                printf("[key - balance]: ");
                break;
            case ext_type::Code:
                printf("[key - code]: ");
                break;
            case ext_type::Storage:
                printf("[key - storage]: ");
                break;
            case ext_type::Generation:
                printf("[key - generation]: ");
                break;
            default:
                printf("[key - unknown]: ");
            }

            for (auto & c : pair.first) {
                printf("%02x", c);
            }

            printf("size: %zu \n", pair.first.size());
            printf("[     value]: ");
            for (auto & c : pair.second) {
                printf("%02x", c);
            }
            printf("\n");
        }
    }

private:
    std::map<std::vector<uint8_t>, std::vector<uint8_t>> ext_kv_datas;
};