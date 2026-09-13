# 为 patchsplit 做出贡献

感谢您有兴趣为 patchsplit 做出贡献！

无论是修复错误、改进文档还是添加新功能，我们都欢迎您的贡献。

## 开发环境

patchsplit 是用 Rust 编写的。

在开始开发之前，请确保您已：

* 安装了 [Rust](https://www.rust-lang.org/tools/install)。
* 安装了 Git。
* 搭建好了适用于您平台的开发环境。

克隆仓库：

```bash
git clone https://github.com/zitzhen/patchsplit.git
cd patchsplit
```

## 构建

以调试模式构建 patchsplit：

```bash
cargo build
```

构建优化后的发布版本：

```bash
cargo build --release
```

## 测试与检查

在提交 Pull Request 之前，请运行相关检查：

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

如果某项检查不适用于您的更改，请在 Pull Request 中说明原因。

## 进行更改

1. 为您的更改创建一个新分支。
2. 进行更改。
3. 运行相关的测试和检查。
4. 提交更改，并附上清晰的提交信息。
5. 提交 Pull Request。

请保持更改专注于特定目标，避免包含不相关的修改。

## Pull Request

提交 Pull Request 时，请包含以下内容：

* 对更改内容的清晰描述。
* 更改的原因。
* 已进行的测试。
* 相关的截图或命令输出（如适用）。

对于错误修复，请尽可能提供复现问题的步骤。

## 报告错误

请使用 [GitHub Issues](https://github.com/zitzhen/patchsplit/issues) 报告错误。

请包含：

* patchsplit 版本。
* 操作系统和架构。
* 复现步骤。
* 预期行为。
* 实际行为。
* 相关的错误信息或日志。

请避免在错误报告中包含敏感信息。

## 代码风格

* 遵循现有的 Rust 代码风格。
* 提交更改前运行 `cargo fmt`。
* 优先编写清晰、易于维护的代码。 * 在适当时添加或更新测试。
* 当功能行为发生变化时，请同步更新相关文档。

## 打包

与打包相关的更改应包含相应的打包文件以及必要的构建或验证步骤。

如果您的更改涉及 Debian、RPM 或其他发行版的软件包，请在 Pull Request 中说明具体影响。

## 许可证

向 patchsplit 贡献代码即表示您同意：您的贡献将采用与本项目相同的许可证（具体见仓库中的 `LICENSE` 文件）进行授权。

感谢您为改进 patchsplit 所做的贡献！