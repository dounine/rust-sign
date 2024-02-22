# rust-ffi
rust同时调用c静态库跟动态库方法 

# 生成动态库
```bash
cd lib
mkdir build
cd build
cmake ..
make
```
# 编译程序
```bash
cargo build
```
# 运行
为了能够找到动态库，需要将动态库所在目录加入到环境变量中
这里使用的是MacOS，所以使用的是DYLD_LIBRARY_PATH
```bash
export DYLD_LIBRARY_PATH=./lib/build
cargo run
```