#pragma once
#include "xevm_runtime/evm_storage_face.h"

#include <map>

namespace top {
namespace evm {
namespace tests {
class xmock_evm_storage : public xtop_evm_storage_face {
public:
    xmock_evm_storage() = default;
    xmock_evm_storage(xmock_evm_storage const &) = delete;
    xmock_evm_storage & operator=(xmock_evm_storage const &) = delete;
    xmock_evm_storage(xmock_evm_storage &&) = default;
    xmock_evm_storage & operator=(xmock_evm_storage &&) = default;
    ~xmock_evm_storage() override = default;

    bytes storage_get(bytes const & key) override;

    void storage_set(bytes const & key, bytes const & value) override;

    void storage_remove(bytes const & key) override;

    // void debug(ext_type const & debug_type = ext_type::ALL) {
    //     for (auto & pair : ext_kv_datas) {
    //         if (debug_type != ext_type::ALL && ext_type(pair.first[1]) != debug_type) {
    //             continue;
    //         }
    //         switch (ext_type(pair.first[1])) {
    //         case ext_type::Nonce:
    //             printf("[key - nonce]: ");
    //             break;
    //         case ext_type::Balance:
    //             printf("[key - balance]: ");
    //             break;
    //         case ext_type::Code:
    //             printf("[key - code]: ");
    //             break;
    //         case ext_type::Storage:
    //             printf("[key - storage]: ");
    //             break;
    //         case ext_type::Generation:
    //             printf("[key - generation]: ");
    //             break;
    //         default:
    //             printf("[key - unknown]: ");
    //         }

    //         for (auto & c : pair.first) {
    //             printf("%02x", c);
    //         }

    //         printf("size: %zu \n", pair.first.size());
    //         printf("[     value]: ");
    //         for (auto & c : pair.second) {
    //             printf("%02x", c);
    //         }
    //         printf("\n");
    //     }
    // }

private:
    std::map<std::vector<uint8_t>, std::vector<uint8_t>> ext_kv_datas;
};

}  // namespace tests
}  // namespace evm
}  // namespace top
