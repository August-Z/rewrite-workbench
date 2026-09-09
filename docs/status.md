# Project status

更新：2026-09-09。

## 当前阶段

M1 · Correctness foundation。**RWB-001 实现与本地验收已完成**：Rust core 与只读 CLI 可以解析 TSX、报告语法错误并观察同文件绑定。完整候选分类、两种改写操作、patch、GUI、WASM 和可安装版本仍未实现，M1 尚未完成。

## 已完成

- M0：产品/架构/验证计划、roadmap、12 个任务的范围与验收、agent 交接及贡献规范；竞品官方文档核查已记录，尚无实际任务计时或用户试用数据。
- RWB-001：两个 Cargo crate，Rust 1.98.1 / Oxc 0.149.0 及 Cargo.lock 固定；10 份原创合成 fixtures 与 expected 分类。
- 验证：10 个 core fixture 与 4 个 CLI 集成测试通过；fmt、Clippy 通过；README 的有效输入/错误输入命令已运行；CLI 输入字节保持检查通过。
- 绑定 evidence 覆盖 alias、参数/块级遮蔽、不同 import 字面来源和 Unicode/BOM/CRLF 坐标。Oxc 节点存储需显式开启，已由测试确认。
- 实现、命令、环境与边界详见 [RWB-001 验证记录](evidence/rwb-001.md)，复现入口见 [README](../README.md#run-the-development-spike)。
- 后续结项复核完成：最终源码的 14 个测试、fmt、Clippy 及交付文件检查通过，未发现 RWB-001 阻塞项；远端交付通过 [Issue #1](https://github.com/August-Z/rewrite-workbench/issues/1) 关联的 PR 跟踪。

## 接入状态

- GitHub 仓库：[August-Z/rewrite-workbench](https://github.com/August-Z/rewrite-workbench)，公开，MIT，默认分支 main。
- Git：RWB-001 起始于 `main` / `8a2ff2b`，在 `codex/rwb-001-workspace` 分支实现；当前 checkout、提交与合入状态以实际 Git 和关联 PR 为准。
- Codex 项目：已添加为 rewrite-workbench；官方项目列表已确认它是本地 Git 项目。
- 执行跟踪：[5 个里程碑与 12 个任务](github-tracking.md)，M0 已关闭；RWB-001 验收完成，交付记录见 [Issue #1](https://github.com/August-Z/rewrite-workbench/issues/1)；其余任务待实施。
- 新任务接续：本次开发会话已从本仓库 AGENTS.md、status 和 Issue #1 成功接续；后续按 [交接流程](agent-workflow.md) 读取当前工作区。

## 下一任务

**[RWB-002：TSX 直接导入与绑定分类](https://github.com/August-Z/rewrite-workbench/issues/2)。**

RWB-001 依赖已本地满足。先读 [任务说明](backlog.md)、[架构](architecture.md)、[验证策略](validation.md) 及 [spike 记录](evidence/rwb-001.md)，扩展导入选择、候选原因和不支持情况分类；不要将现有绑定观察直接作为 `ready` 改写候选。RWB-003 的依赖也已满足，但本次未开始实施。

## 未验证事项

- RWB-002 的完整来源/不支持形式分类，以及 RWB-003 的快照、edit 与 patch 契约。
- 两种操作在合成与真实 TSX 项目中的正确性。
- 竞品任务耗时和真实用户操作优势。
- 安装分发、支持平台和公开传播效果。

## 状态维护

完成任务后以事实更新本文件。保留未完成项，不以计划或原型替代实现证据；详细日志与研究记录链接到对应文档或 issue。
