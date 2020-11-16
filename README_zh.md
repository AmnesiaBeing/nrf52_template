# `nrf52_template`

> 一个适用于NRF52 MCU的开发模板，使用Rust语言。（作者仅在NRF52832芯片上验证过）

## 开发环境搭建

推荐使用VSCODE、Linux操作系统，其他操作系统应该也可以。

1. VSCODE 当前模板下已经配置好VSCODE调试和生成任务，推荐安装cortex-debug、rls插件。
2. Rust版本需要稳定版1.31以上（作者仅在1.47版本验证过），使用rustup安装目标编译架构
```
# 在合适的终端下执行以下命令
rustup target add thumbv7em-none-eabihf
```
3. 调试器软件部分使用openocd，强烈建议安装git版本
    > 作者是Archlinux用户，在archlinuxcn源中虽然有openocd，但是这个版本不好使，需要从AUR源中安装openocd-git版本，其他发行版建议从openocd官方源中下载源码自行编译。
4. 编译器与调试器
    > Archlinux源中有arm-none-eabi-gcc/gdb，安装即可，其他发行版需自行研究如何安装
4. 推荐的cargo插件
```
# cargo-binutils提供了一些查看生成文件的工具，包括nm、objdump等等
cargo install cargo-binutils
```

## 模板使用

1. 首先clone项目，并对项目名称做修改，以符合使用者喜欢的命令规则，修改的相关文件有：`Cargo.toml`,`.vscode/launch.json`

```
git clone https://github.com/AmnesiaBeing/nrf52_template
```

2. 连接好开发板后，（SWD方式，示例程序使用了半主机，还需要连接SWO接口），在VSCODE中运行调试配置（快捷键F5）观察即可，理论上可以输出Hello World!

## 许可证

作者不负责那种。
