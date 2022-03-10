cmake_minimum_required(VERSION 3.8)

aux_source_directory(./ tests_src)

add_executable(test_aurora ${tests_src})

add_dependencies(test_aurora xtop_mock_api xevm_engine)
get_target_property(EVM_ENGINE_DIR xevm_engine LOCATION)
message("EVM_ENGINE_DIR: " ${EVM_ENGINE_DIR})
target_link_libraries(test_aurora PRIVATE xtop_mock_api ${EVM_ENGINE_DIR}/libxevm_engine.a)