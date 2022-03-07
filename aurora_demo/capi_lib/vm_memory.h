#pragma once
#include <map>
#include <string>
#include <vector>

class vm_memory {
public:
    void read_memory(uint64_t offset, std::vector<uint8_t> & buffer) {
        // std::printf("[vm_memory][read] : from: %lu - %lu  size: %zu \n", offset, offset + buffer.size(), buffer.size());
        char * begin_address = (char *)offset;
        for (std::size_t i = 0; i < buffer.size(); ++i) {
            buffer[i] = *begin_address;
            begin_address++;
        }
        // printf("[debug][vm_memory][read] result:");
        // for (auto _c : buffer) {
        //     printf("%x",_c);
        // }
        // printf("\n");
    }

    void write_memory(uint64_t offset, std::vector<uint8_t> const & buffer) {
        // std::printf("[vm_memory][write] : from: %lu - %lu  size: %zu  \n", offset, offset + buffer.size(), buffer.size());
        char * begin_address = (char *)offset;
        for (std::size_t i = 0; i < buffer.size(); ++i) {
            *begin_address = buffer[i];
            begin_address++;
        }
    }
};