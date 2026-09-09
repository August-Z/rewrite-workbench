# Codex 与 Agent 工作流

本仓库把可持续上下文放在 Git 文档中；Codex 项目负责定位 checkout 与创建后续任务。无需复制之前聊天，也无需把个人全局配置提交到仓库。

## 新对话读取顺序

1. 仓库根 `AGENTS.md`。
2. [status](status.md)：实际完成项、验证边界、下一任务。
3. [roadmap](../ROADMAP.md) 与 [backlog](backlog.md)：当前阶段、依赖和验收。
4. 对应的 [architecture](architecture.md)、[validation](validation.md) 或研究文档。

Codex 会读取项目范围的 `AGENTS.md`；具体发现机制见 [官方说明](https://learn.chatgpt.com/docs/agent-configuration/agents-md)。本文其他操作规则是本项目约定，不是对所有 Codex 版本界面的保证。

## 开始实施的提示词

> 请读取仓库 AGENTS.md、docs/status.md、ROADMAP.md 和 docs/backlog.md。先核对 branch、HEAD 与 dirty state，然后从 RWB-001 开始实施。只完成该任务的最小可编译 workspace、依赖固定、合成 fixtures 和解析/绑定 spike，不扩展到完整 UI。运行相应检查，更新真实命令与状态，并列明未验证项。提交或推送按我在本任务中的授权执行。

后续可把 RWB-001 替换为已满足依赖的任务 ID。不要一次性向多个任务发送“完成整个 roadmap”，以免重复工作和状态冲突。

## 并行协作

- 一个主任务拥有范围、共享契约、Git 合并和外部发布。
- 独立 writer 先分配明确文件/模块；例如 core fixtures、UI 与文档可以在契约确定后并行。
- 同一个 workspace 的 sub-agent 共享文件；不要假设它们自动隔离。
- 独立 Codex 开发任务优先使用 worktree；验证当前任务是否在工作副本，不把另一 checkout 的状态当作本任务状态。
- 接口变更先通知消费者；避免两个 agent 同时改根 manifest、锁文件与 status。
- review 任务只报告可行动问题，主负责人决定和实施合并；不要为了形式重复已有效的检查。

## 每次交接留下什么

任务结束时只更新必要材料：

- 已完成任务与对应 commit/issue 或实际文件。
- 实际运行的命令与结果，附上环境/版本对结论的影响。
- 尚未完成或未验证的部分；把实现、测试、打包和真实用户观察分开。
- 下一项 task ID、依赖是否满足、需要重点阅读的文档。

不把原始聊天、个人路径、凭据、私有业务代码或大段运行日志粘贴进公共状态文件。

## GitHub 与 Codex 的分工

GitHub issue/milestone 记录排期、讨论和执行状态；版本化文档记录范围与技术契约；Codex 任务负责具体工作。

默认执行小而完整的任务。是否 commit、push、创建 PR、发布版本和对外宣传，遵循用户当前授权。初始建仓授权不等于对所有未来发布的长期授权。

## 当前命令边界

当前仓库仅完成项目资料启动；尚无 `Cargo.toml`、前端 package scripts 或安装包。可用 Git 命令检查仓库状态，但不要声称 `cargo test`、`pnpm dev` 或安装命令已经存在。RWB-001 与前端启动任务完成后更新这一节及 README。

## 接入验收

- 本地路径对应期望仓库，remote origin 指向 GitHub 项目。
- Codex 的项目列表显示该 checkout。
- 新任务读取本仓库 `AGENTS.md` 和状态文件，能够指出当前阶段与 RWB-001。
- 若使用 worktree，记录该任务自己的 branch/HEAD；不把“项目已添加”当作远程云环境也已配置。

本地项目工作无需另外开通云执行。云环境、自动审查、定时任务等在实际需要时单独配置。
