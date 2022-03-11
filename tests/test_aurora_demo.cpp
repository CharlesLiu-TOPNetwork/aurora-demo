#include "../mock_top_api/evm_engine_interface.h"
#include "../mock_top_api/vm_import_instance.h"
#include "../mock_top_api/vm_util.h"

#include <gtest/gtest.h>

TEST(test_demo, deploy_code_twice) {
    auto & logic = vm_import_instance::instance()->get_vm_logic_ref();
    deploy_code();
    logic.context_ref().update_hex_string_input(
        "0x608060405234801561001057600080fd5b506101f7806100206000396000f3fe6080604052600436106100345760003560e01c80634f2be91f14610039578063c3da42b814610043578063c54124be1461008257"
        "5b600080fd5b61004161008c565b005b34801561004f57600080fd5b5061005861011a565b604051808267ffffffffffffffff1667ffffffffffffffff16815260200191505060405180910390f35b61008a610133"
        "565b005b60016000809054906101000a900467ffffffffffffffff16016000806101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506000809054906101000a900467ffffffff"
        "ffffffff1667ffffffffffffffff167f53777d0aaa80b55d855a69674c6ee34efa3ccb05a03ad90c38aaa0eacfe7dcd760405160405180910390a2565b6000809054906101000a900467ffffffffffffffff168156"
        "5b60016000809054906101000a900467ffffffffffffffff16036000806101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506000809054906101000a900467ffffffffffffff"
        "ff1667ffffffffffffffff167f09e6eb7229785cdf85405747f017ad07512b61743d1458c526daedb056f791a760405160405180910390a256fea2646970667358221220f5d0edd9d4d3902b094c01968b3fe1ea21"
        "1b6f8913755b092866166849deb75064736f6c63430006040033");
    deploy_code();
}

TEST(test_demo, call_contract) {
    deploy_code();
    auto & logic = vm_import_instance::instance()->get_vm_logic_ref();
    std::string contract_address = "fb29cba9b146786da16733f89982f7481effb094";
    std::string contract_params = "0xe1c7392a";  // "init()"
    logic.context_ref().update_input(serialize_function_input(contract_address, contract_params));
    call_contract();
    call_contract();
}

TEST(test_demo, test_add_contract) {
    /***************************************************
    pragma solidity 0.6.4;
    contract xevm_test {
        uint64 public global;
        event catchAdd(uint64 a, uint64 b, uint64 c);
        event catchAddOne(uint64 a, uint64 ret);
        event catchGlobal(uint64 c);
        function add(uint64 a, uint64 b) public {
            uint64 c = a + b;
            emit catchAdd(a, b, c);
        }
        function addOne(uint64 a) public {
            uint64 b = a + 1;
            emit catchAddOne(a, b);
        }
        function addGlobal() public {
            global = global + 1;
            emit catchGlobal(global);
        }
    }
    ******************************************************/
    vm_logic new_logic;
    vm_import_instance::instance()->set_vm_logic(new_logic);
    auto & logic = vm_import_instance::instance()->get_vm_logic_ref();

    logic.context_ref().update_hex_string_input(
        "0x608060405234801561001057600080fd5b50610304806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80632fb3c740146100515780636e2c732d146100"
        "5b578063a05f9906146100a7578063fad772db146100d9575b600080fd5b610059610111565b005b6100a56004803603604081101561007157600080fd5b81019080803567ffffffffffffffff1690602001909291"
        "90803567ffffffffffffffff1690602001909291905050506101b3565b005b6100af610242565b604051808267ffffffffffffffff1667ffffffffffffffff16815260200191505060405180910390f35b61010f60"
        "0480360360208110156100ef57600080fd5b81019080803567ffffffffffffffff16906020019092919050505061025b565b005b60016000809054906101000a900467ffffffffffffffff16016000806101000a81"
        "548167ffffffffffffffff021916908367ffffffffffffffff1602179055507f4558e87569f3778927a0c0ab7e6c04d9cc3a08694412ac6b8b8bbd8276c8fb3d6000809054906101000a900467ffffffffffffffff"
        "16604051808267ffffffffffffffff1667ffffffffffffffff16815260200191505060405180910390a1565b600081830190507fba81e10edd752f92a850d20e6ca5897bc0ff54393985bc25b2c2bdc92521864983"
        "8383604051808467ffffffffffffffff1667ffffffffffffffff1681526020018367ffffffffffffffff1667ffffffffffffffff1681526020018267ffffffffffffffff1667ffffffffffffffff16815260200193"
        "5050505060405180910390a1505050565b6000809054906101000a900467ffffffffffffffff1681565b60006001820190507f76b87589c0efe817c6ec312c8fa2ab35ac24bbbd1e5fb8d3e3c3b4b789fdc7d48282"
        "604051808367ffffffffffffffff1667ffffffffffffffff1681526020018267ffffffffffffffff1667ffffffffffffffff1681526020019250505060405180910390a1505056fea2646970667358221220aa046f"
        "634f0927440a2dc3e5b0298f8101a60505f1d303bc416a90fcc0db54fa64736f6c63430006040033");
    deploy_code();

    std::string contract_address = "fb29cba9b146786da16733f89982f7481effb094";

    // add(123, 321) => (123,321,444)
    std::string contract_params = "0x6e2c732d000000000000000000000000000000000000000000000000000000000000007b0000000000000000000000000000000000000000000000000000000000000141";
    logic.context_ref().update_input(serialize_function_input(contract_address, contract_params));
    call_contract();

    // addOne(12345) => (12346)
    contract_params = "0xfad772db0000000000000000000000000000000000000000000000000000000000003039";
    logic.context_ref().update_input(serialize_function_input(contract_address, contract_params));
    call_contract();

    // addGlobal => (1)
    contract_params = "0x2fb3c740";
    logic.context_ref().update_input(serialize_function_input(contract_address, contract_params));
    call_contract();
}

TEST(test_demo, erc20) {
}