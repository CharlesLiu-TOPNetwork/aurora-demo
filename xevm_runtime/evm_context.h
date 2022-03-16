#pragma once
#include "xevm_runtime/evm_util.h"

#include <string>
namespace top {
namespace evm {

class xtop_evm_context {
public:
    bytes m_random_seed;
    bytes m_input;
    bytes m_predecessor_account_id;

public:
    xtop_evm_context() {
        // ! test only .
        m_random_seed = utils::hex_string_to_bytes("0x1234567890");
        m_input = utils::hex_string_to_bytes(
            "0x608060405234801561001057600080fd5b50610168806100206000396000f3fe6080604052600436106100295760003560e01c8063c3da42b81461002e578063e1c7392a1461006d575b600080fd5b348015"
            "61003a57600080fd5b50610043610077565b604051808267ffffffffffffffff1667ffffffffffffffff16815260200191505060405180910390f35b610075610090565b005b6000809054906101000a900467"
            "ffffffffffffffff1681565b60016000809054906101000a900467ffffffffffffffff16016000806101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fce5afaa0bb50"
            "af954d5977431c65c35b4417c30984ca04e53ce06ffeb4fc27146000809054906101000a900467ffffffffffffffff16604051808267ffffffffffffffff1667ffffffffffffffff1681526020019150506040"
            "5180910390a156fea2646970667358221220609918375f4be9388512c0f6c641dae865436156ff2b6382f7157cfe819f424a64736f6c63430006040033");
        m_predecessor_account_id = utils::string_to_bytes("carol");
    }
    xtop_evm_context(bytes const & random_seed, bytes const & input, bytes const & predecessor_account_id)
      : m_random_seed{random_seed}, m_input{input}, m_predecessor_account_id{predecessor_account_id} {
    }

    void update_random_seed(std::string const & input_hex) {
        m_random_seed = utils::hex_string_to_bytes(input_hex);
    }

    void update_input(bytes const & input_vec_u8) {
        m_input = input_vec_u8;
    }

    void update_hex_string_input(std::string const & input_hex) {
        m_input = utils::hex_string_to_bytes(input_hex);
    }

    void update_string_predecessor_account_id(std::string const & input_str) {
        m_predecessor_account_id = utils::string_to_bytes(input_str);
    }
};
using xevm_context_t = xtop_evm_context;
}  // namespace evm
}  // namespace top