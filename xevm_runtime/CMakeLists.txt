cmake_minimum_required(VERSION 3.8)

aux_source_directory(./ mock_api_src)

add_library(xevm_runtime STATIC ${mock_api_src})

