cmake_minimum_required(VERSION 3.8)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

project(XTOP_EVM_DEMO CXX C)

add_compile_options(-static -Wall -Werror)
add_compile_options(-pthread)
add_compile_options(-fPIC)

# options for Debug
#add_compile_options($<$<OR:$<CONFIG:Debug>,$<CONFIG:debug>>:-g2 -ggdb -O0 -fno-omit-frame-pointer>)
set_property(DIRECTORY APPEND PROPERTY COMPILE_OPTIONS $<$<OR:$<CONFIG:Debug>,$<CONFIG:debug>>:-g3 -ggdb -O0 -fno-omit-frame-pointer>)
set_property(DIRECTORY APPEND PROPERTY COMPILE_DEFINITIONS $<$<OR:$<CONFIG:Debug>,$<CONFIG:debug>>:DEBUG _DEBUG>)

# options for Release & RelWithDebInfo
# add_compile_options($<$<OR:$<CONFIG:Release>,$<CONFIG:RelWithDebInfo>,$<CONFIG:release>>:-fomit-frame-pointer -ffunction-sections -fdata-sections>)
set_property(DIRECTORY APPEND PROPERTY COMPILE_OPTIONS $<$<OR:$<CONFIG:Release>,$<CONFIG:RelWithDebInfo>,$<CONFIG:release>>:-ffunction-sections -fdata-sections>)
set_property(DIRECTORY APPEND PROPERTY COMPILE_DEFINITIONS $<$<OR:$<CONFIG:Release>,$<CONFIG:RelWithDebInfo>,$<CONFIG:release>>:NDEBUG>)

# options for Release
add_compile_options($<$<OR:$<CONFIG:Release>,$<CONFIG:release>>:-g0>)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()


message(STATUS "CMAKE_BUILD_TYPE:" ${CMAKE_BUILD_TYPE})
message(STATUS "CMAKE_SYSTEM_NAME:" ${CMAKE_SYSTEM_NAME})
message(STATUS "CMAKE_CXX_COMPILER_ID:" ${CMAKE_CXX_COMPILER_ID})

set(EXECUTABLE_OUTPUT_PATH ${XTOP_EVM_DEMO_BINARY_DIR}/bin/${CMAKE_SYSTEM_NAME}/)
set(LIBRARY_OUTPUT_PATH ${XTOP_EVM_DEMO_BINARY_DIR}/lib/${CMAKE_SYSTEM_NAME}/)


message(STATUS "EXECUTABLE_OUTPUT_PATH:" ${EXECUTABLE_OUTPUT_PATH})
message(STATUS "LIBRARY_OUTPUT_PATH:" ${LIBRARY_OUTPUT_PATH})

add_subdirectory(evm_engine_rs)
add_subdirectory(mock_top_api)
add_subdirectory(tests)
