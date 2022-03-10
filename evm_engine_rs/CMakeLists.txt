if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

message("Cargo cmd: " ${CARGO_CMD})
set(EVM_A "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libxevm_engine.a")
message( "EVM_A: " ${EVM_A} )
message( "CMAKE_CURRENT_BINARY_DIR: " ${CMAKE_CURRENT_BINARY_DIR})
message( "CMAKE_CURRENT_SOURCE_DIR: " ${CMAKE_CURRENT_SOURCE_DIR})
message( "LIBRARY_OUTPUT_PATH: " ${LIBRARY_OUTPUT_PATH})

add_custom_target(xevm_engine ALL
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMAND cp -f ${EVM_A} ${LIBRARY_OUTPUT_PATH})

set_target_properties(xevm_engine PROPERTIES LOCATION ${LIBRARY_OUTPUT_PATH})

# mock:
add_dependencies(xevm_engine xtop_mock_api)