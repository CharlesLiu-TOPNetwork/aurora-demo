### 准备工作：
3个项目：
aurora_demo: https://github.com/CharlesLiu-TOPNetwork/aurora-demo
aurora-engine: https://github.com/aurora-is-near/aurora-engine
nearcore: https://github.com/near/nearcore

### 编译运行：
``` BASH
cd aurora_demo
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:.
bash run.sh
```

### 修改libaurora_engine.so
编译`aurora-engine`项目：`cargo build --features mainnet`
然后复制过来。

### libcapi
全部来自于`aurora-engine`需求的外部接口：`engine-sdk/src/near_runtime.rs`: `pub(crate) mod exports`
https://github.com/near/nearcore/blob/master/runtime/near-vm-logic/src/logic.rs 同名方法

### test
可以调用的api在`aurora-engine`项目，`engine/src/lib.rs` 用`#[no_mangle]`修饰的。