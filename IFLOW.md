# IFLOW.md - 项目上下文

## 项目概述

这是一个名为 `rat-trig-rs` 的 Rust 项目，实现了有理三角学（Rational Trigonometry）。有理三角学是由 Norman Wildberger 开发的一种新方法来处理经典三角学，旨在通过仅使用有理数和运算来简化和澄清主题，而不是使用无理数和极限。

项目提供了基本的三角函数计算，如：
- Archimedes 公式计算三角形面积
- Quadrance（平方距离）
- Spread（平方正弦值）
- Cross（叉积）
- 以及基于直线和平面点的计算

## 项目结构

- `Cargo.toml`: Rust 项目的配置文件，定义了依赖、版本等信息
- `src/lib.rs`: 库的入口文件，导出了 `trigonom` 模块
- `src/trigonom.rs`: 核心有理三角学函数的实现，包含详细文档和测试用例

## 技术栈

- **语言**: Rust (edition 2021)
- **依赖**:
  - `num-traits`: 提供数值特性（Zero, One 等）
  - `num-rational`: 有理数支持（开发依赖）
  - `fractions-rs`: 分数支持（开发依赖）

## 核心功能

1. **Archimedes 公式**: 计算三角形面积
2. **Quadrance**: 计算两点间平方距离
3. **Spread**: 计算两向量间平方正弦值
4. **Cross**: 计算两向量叉积
5. **基于直线的计算**: 从直线计算距离、夹角等

## 测试

项目包含全面的单元测试，覆盖了各种数值类型（i32, i64, f64, Rational32 等）的计算。

## 构建和运行

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 发布构建
cargo build --release
```

## 开发约定

- 使用泛型类型参数 `T` 来支持不同的数值类型
- 所有函数都有详细的文档说明和示例
- 遵循 Rust 编码标准和最佳实践
- 包含全面的单元测试

## 许可证

该项目采用 MIT 或 Apache-2.0 许可证双重授权。