

extern "C" void get_version();
extern "C" void deploy_code();
extern "C" void call();
extern "C" void deploy_erc20_token();

int main() {
    // get_version();
    // deploy_erc20_token();
    deploy_code();
    call();
    // call
    return 0;
}