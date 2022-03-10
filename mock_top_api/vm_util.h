#pragma once

#include "assert.h"
#include "ripemd160.h"
#include "sh256.h"
#include "stdint.h"

#include <string>
#include <vector>

typedef __int128 int128_t;
typedef unsigned __int128 uint128_t;

#include <vector>

std::vector<uint8_t> to_le_bytes(uint128_t value);
std::vector<uint8_t> hex_string_to_bytes(std::string const & input);
std::vector<uint8_t> string_to_bytes(std::string const & input);
std::vector<uint8_t> get_sha256(std::vector<uint8_t> const & input);
std::vector<uint8_t> get_ripemd160(std::vector<uint8_t> const & input);