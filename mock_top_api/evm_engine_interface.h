#pragma once
#include "stdint.h"

extern "C" void deploy_code();
extern "C" void call_contract();
extern "C" void serial_noparam_function_callargs(const char * address,
                                                 uint64_t address_len,
                                                 const char * function,
                                                 uint64_t function_len,
                                                 uint64_t max_output_len,
                                                 unsigned char * output,
                                                 uint64_t * output_len);