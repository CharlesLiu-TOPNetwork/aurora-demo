#pragma once
#include "xevm_runtime/evm_storage_face.h"

namespace top {
namespace evm {

enum class ext_type {
    Config = 0x0,
    Nonce = 0x1,
    Balance = 0x2,
    Code = 0x3,
    Storage = 0x4,
    Generation = 0x5,

    ALL = 0x6  // show all
};

class xtop_evm_storage_base : public xevm_storage_face_t {
public:
    xtop_evm_storage_base() = default;
    xtop_evm_storage_base(xtop_evm_storage_base const &) = delete;
    xtop_evm_storage_base & operator=(xtop_evm_storage_base const &) = delete;
    xtop_evm_storage_base(xtop_evm_storage_base &&) = default;
    xtop_evm_storage_base & operator=(xtop_evm_storage_base &&) = default;
    ~xtop_evm_storage_base() override = default;

public:
    // todo might add some utl function here to decode key type.
};
using xevm_storage_base_t = xtop_evm_storage_base;
}  // namespace evm
}  // namespace top