# Project status

更新：2026-09-09。

## 当前阶段

M1 · Correctness foundation。**RWB-001、RWB-002 实现与本地验收已完成**：Rust core 与只读 CLI 可解析 TSX、分类直接命名导入及同文件绑定。绑定确认仍为 `skipped / operation_not_evaluated`；两种改写操作、快照/patch、GUI、WASM 和可安装版本仍未实现，M1 尚未完成。

## 已完成

- M0：产品/架构/验证计划、roadmap、12 个任务的范围与验收、agent 交接及贡献规范；尚无实际任务计时或用户试用数据。
- RWB-001：Cargo core/CLI、Rust 1.98.1 / Oxc 0.149.0 固定、10 份 spike fixtures；已通过 [PR #13](https://github.com/August-Z/rewrite-workbench/pull/13) 合入 main，记录见 [RWB-001 证据](evidence/rwb-001.md)。
- RWB-002：新增 `classify_tsx` 与 `bindings` CLI，覆盖直接命名导入/as 别名、不同字面来源、局部遮蔽、DOM、type-only、未解析及不支持形式（含多声明 SymbolId）；明确原因与 UTF-8 byte evidence。旧 inspect 协议兼容，无 ready 改写结果。
- 验证：29 个 workspace 测试通过（10 旧 core + 11 分类 + 8 CLI），fmt、Clippy 通过；9 份新增原创 fixtures 覆盖 64 条 JSX 使用预期，README bindings 命令已运行，CLI 输入字节保持检查通过。
- 本次接口、原因优先级、环境和边界见 [RWB-002 验证记录](evidence/rwb-002.md) 与 [分类协议](architecture.md#rwb-002-已实现的只读分类协议)。

## 接入与交付状态

- GitHub：[August-Z/rewrite-workbench](https://github.com/August-Z/rewrite-workbench)，任务见 [tracking](github-tracking.md)。
- 本次起始本地/远端 main 和 PR #13 merge commit 均为 `9e5f1ac7a03a495b5447fbd66e442ff3adbaf70d`。实现分支为 `augustz/rwb-002-bindings`，实现与本地验收已完成；提交前工作区尚未暂存，后续交付状态以关联 PR 和 Issue #2 为准。
- 开始时 Issue #2 为 OPEN。本地完成不代表远端已交付，最新 Git 与 GitHub 状态仍应现场核对。
- 已有无关未跟踪 IDE 内容保持原样，未读取或纳入本次修改。后续任务按 [交接流程](agent-workflow.md) 读取实际 checkout。

## 下一任务

**[RWB-003：不可变修改计划与 patch 输出](https://github.com/August-Z/rewrite-workbench/issues/3)。** RWB-001 依赖已满足。读取 [backlog](backlog.md)、[架构](architecture.md)、[验证策略](validation.md) 及 [本次证据](evidence/rwb-002.md)，建立快照/hash、UTF-8 edits、冲突与 patch 契约。本次没有开始 RWB-003；RWB-004 的两种操作要在 003 完成后组合实施，不能把绑定确认直接变成 ready。

## 未验证事项

- 快照/修改计划/patch，以及两种操作在合成与真实 TSX 工程中的正确性。
- 最终模块/物理文件解析、跨文件符号来源、完整 TypeScript 类型检查和任意业务行为保持。
- GUI、WASM、CI、安装分发及其他平台。
- 竞品任务耗时、真实用户操作优势、性能与公开增长观察。

## 状态维护

完成任务后以事实更新本文件。保留未完成项，不以计划或原型替代实现证据；详细证据链接到对应文档或 issue。
