#pragma once

#include "assert.h"
#include "ripemd160.h"
#include "sh256.h"
#include "stdint.h"

#include <string>
#include <vector>

typedef __int128 int128_t;
typedef unsigned __int128 uint128_t;

std::vector<uint8_t> serialize_function_input(std::string const & contract_address, std::string const & contract_function, uint64_t value = 0);

std::vector<uint8_t> to_le_bytes(uint128_t value);
void hex_string_bytes_char(std::string const & input, unsigned char * output);
std::vector<uint8_t> hex_string_to_bytes(std::string const & input);
std::vector<uint8_t> string_to_bytes(std::string const & input);
std::string uint8_vector_to_hex_string(std::vector<uint8_t> const & v);
std::vector<uint8_t> get_sha256(std::vector<uint8_t> const & input);
std::vector<uint8_t> get_ripemd160(std::vector<uint8_t> const & input);