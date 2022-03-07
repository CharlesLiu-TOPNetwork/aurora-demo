#pragma once
#include <map>
#include <string>

// should be a vitrual base class `External` in C++
// Trait: `runtime/near-vm-logic/src/dependencies.rs`
// Impl: `runtime/runtime/src/ext.rs`
class vm_ext {
public:
    vm_ext() {
        ext_kv_datas.insert({{'\0' + 7, '\0' + 0, '\0' + 83, '\0' + 84, '\0' + 65, '\0' + 84, '\0' + 69},
                             {'\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,
                              '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,
                              '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 78,  '\0' + 69,  '\0' + 65,  '\0' + 84,  '\0' + 6,   '\0' + 0,   '\0' + 0,   '\0' + 0,
                              '\0' + 97,  '\0' + 117, '\0' + 114, '\0' + 111, '\0' + 114, '\0' + 97,  '\0' + 18,  '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 98,  '\0' + 114,
                              '\0' + 105, '\0' + 100, '\0' + 103, '\0' + 101, '\0' + 95,  '\0' + 112, '\0' + 114, '\0' + 111, '\0' + 118, '\0' + 101, '\0' + 114, '\0' + 46,
                              '\0' + 110, '\0' + 101, '\0' + 97,  '\0' + 114, '\0' + 1,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0,   '\0' + 0}});
    }
    std::vector<uint8_t> storage_get(std::vector<uint8_t> const & key) {
        printf("[debug][vm_ext][storage_get] size: %zu key:", key.size());
        for (auto _c : key) {
            printf("%c", _c);
        }
        printf("\n");
        auto res = ext_kv_datas[key];
        printf("[debug][vm_ext][storage_get] size: %zu value:", res.size());
        for (auto _c : res) {
            printf("%c", _c);
        }
        printf("\n");

        return res;
        return ext_kv_datas[key];
    }
    void storage_set(std::vector<uint8_t> const & key, std::vector<uint8_t> const & value) {
        printf("[debug][vm_ext][storage_set] size: %zu key:", key.size());
        for (auto _c : key) {
            printf("%c", _c);
        }
        printf("\n");
        printf("[debug][vm_ext][storage_set] size: %zu value:", value.size());
        for (auto _c : value) {
            printf("%c", _c);
        }
        printf("\n");
        ext_kv_datas[key] = value;
    }

private:
    std::map<std::vector<uint8_t>, std::vector<uint8_t>> ext_kv_datas;
};