# Project status

更新：2026-09-09。

## 当前阶段

M0 · Project bootstrap 已完成，下一阶段为 M1。项目计划、GitHub 仓库、任务跟踪与 Codex 本地项目已建立。Rust 引擎、CLI、GUI、WASM 和可安装版本均尚未实现。

## 已完成

- 产品范围、技术方案、验证策略与 M0–M4 roadmap。
- RWB-001 至 RWB-012 的依赖、交付和验收。
- `AGENTS.md`、agent 交接流程、贡献与 issue 模板。
- 竞品能力的官方文档核查；没有实际任务计时或用户试用数据。
- 独立文档审查与修正：统一候选状态、首版节点预填范围、外部试用非阻塞边界。
- 本地 Markdown 引用、JSON 示例与 Git diff 检查；文档阶段未运行项目构建。

## 接入状态

- GitHub 仓库：[August-Z/rewrite-workbench](https://github.com/August-Z/rewrite-workbench)，公开，MIT，默认分支 main。
- 本地 Git：已初始化 main 并推送 origin；后续更新以实际 Git 状态为准。
- Codex 项目：已添加为 rewrite-workbench；官方项目列表已确认它是本地 Git 项目。
- 执行跟踪：[5 个里程碑与 12 个任务](github-tracking.md)，M0 已关闭，其余待实施。
- 新任务接续：尚未启动开发任务。可在 Codex 的 rewrite-workbench 项目中新建任务，使用 [启动提示词](agent-workflow.md)。尚未以新会话运行验证指令加载。

## 下一任务

**[RWB-001：初始化最小 workspace、依赖与合成 fixtures](https://github.com/August-Z/rewrite-workbench/issues/1)。**

先读取 [任务说明](backlog.md)、[架构](architecture.md) 与 [验证策略](validation.md)，建立最小可编译垂直路径。运行时依赖与工具链在实施 spike 时统一固定。不要从完整 UI 或多语言框架开始。

## 未验证事项

- Oxc 选定版本的实际编译/API 兼容性。
- 两种操作在合成与真实 TSX 项目中的正确性。
- 竞品任务耗时和真实用户操作优势。
- 安装分发、支持平台和公开传播效果。

## 状态维护

完成任务后以事实更新本文件。保留未完成项，不以计划或原型替代实现证据；详细日志与研究记录链接到对应文档或 issue。
