cmake_minimum_required(VERSION 3.8)

aux_source_directory(./ mock_api_src)

add_library(xtop_mock_api STATIC ${mock_api_src})

