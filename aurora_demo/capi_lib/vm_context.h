#pragma once
#include "vm_util.h"

#include <string>
#include <vector>

class vm_context {
public:
    std::vector<uint8_t> input = hex_string_to_bytes("0x608060405234801561001057600080fd5b5060f88061001f6000396000f3fe60806040526004361060265760003560e01c8063c3da42b814602b578063e1c7392a146067575b600080fd5b348015603657600080fd5b50603d606f565b604051808267ffffffffffffffff1667ffffffffffffffff16815260200191505060405180910390f35b606d6088565b005b6000809054906101000a900467ffffffffffffffff1681565b6000600190506000600290508082016000806101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550505056fea264697066735822122010a734c8fc147d6b16804571cd80edc56595724b780c3895480b88338264e4fb64736f6c63430006040033");
    std::vector<uint8_t> account_id = string_to_bytes("aurora");
    std::vector<uint8_t> predecessor_account_id = string_to_bytes("carol");
    // std::vector<uint8_t> input{'1', '2', '3'};
    // std::vector<uint8_t> account_id{'a', 'u', 'r', 'o', 'r', 'a'};
};