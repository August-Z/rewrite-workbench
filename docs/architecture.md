# rewrite-workbench 架构计划

本文描述目标架构与协议；除下述 RWB-001 实现边界外，模块、recipe JSON 示例和阶段能力仍为计划。实际验证见 [RWB-001 记录](evidence/rwb-001.md)。

RWB-001 已建立 `rewrite-core` 与 `rewrite-cli` 两个 crate，固定 Rust 1.98.1 和 Oxc 0.149.0。core 的 `inspect_tsx(&str)` 使用显式 TSX module 语法、开启正则解析和 Semantic syntax checks，仅返回导入/引用绑定观察及 UTF-8 byte spans；CLI 负责单文件读取和 JSON 输出。报告使用独立的 `schemaVersion: 1`，不是下文 Recipe v1 或 EditPlan 协议。正常输入为 `skipped / binding_spike_only`，错误输入为 `invalid / parse_error` 或 `invalid / semantic_syntax_error`；绑定成立不代表操作前置条件已经成立。完整候选分类、文件 hash、版本化请求、edit 与 patch 尚未实现，当前观察不能保存为可应用计划。

首版目标是：以 TSX 文件为用户入口和验收范围，依据直接导入的组件身份，预览两种确定规则的 JSX 改写，并导出可以审查的 patch。共享解析基础可考虑 JS、JSX、TS，但不因能够解析就宣称相关用户流程已验证。Alpha 不直接写入源文件；v0.1 再增加带内容 hash 检查的直接应用。正确性验收见 [validation.md](./validation.md)。

## 1. 首版范围

首版只计划实现以下两种操作：

| 操作 | 例子 | 命中条件 |
| --- | --- | --- |
| JSX 属性改名 | size 变为 density | 组件直接导入来源匹配，存在唯一的显式旧属性，不存在目标属性 |
| 固定属性字符串值映射 | size="small" 变为 size="compact" | 组件来源匹配，存在唯一的显式属性，其值为与规则相等的带引号字符串字面量 |

来源匹配首先限定为静态 ESM 命名导入，支持本地 as 别名。文件内的局部遮蔽必须排除。JS 和 TS 文件可以参与工程扫描，但只有其已选定语法模式允许且实际包含 JSX 的文件才会产生这两类修改；不得通过改用宽松语法模式掩盖解析错误。

以下能力计划延期：通用代码模式语言、根据任意前后样例推导规则、完整 TypeScript 类型检查、跨文件符号来源追踪、namespace/default import 组件改写、组件包装及变量赋值后的来源推断、对象展开属性、JSX 表达式中的常量求值、任意脚本插件执行、自动修复项目原有错误。

批量扫描多个文件只表示逐文件运行同一规则，不表示已具备跨文件语义分析。

## 2. 核心选型

计划采用 Rust + Oxc Parser + Oxc Semantic，前端采用 React、TypeScript、Monaco。Rust CLI 与本地 HTTP 服务共用同一个改写核心。首版不引入第二套 AST，也不重新实现通用结构匹配引擎。

Oxc Semantic 提供作用域、声明和引用绑定；它不等同于完整 TypeScript typechecker。当前官方 API 可以取得 ReferenceId、SymbolId、声明节点以及作用域绑定，适合验证同一文件中的 import alias 和 shadowing。[Oxc Semantic](https://docs.rs/oxc_semantic/latest/oxc_semantic/)、[Scoping API](https://docs.rs/oxc_semantic/latest/oxc_semantic/struct.Scoping.html)

ast-grep 已有通用结构匹配与改写能力，但官方明确表示它本身不提供作用域和类型分析。本项目首版通过限制操作种类使用单 Oxc 核心；未来若引入任意模式，需重新比较复用 ast-grep 与扩展 Oxc 操作层的成本，不能把通用 matcher 当作现成 Oxc 能力。[ast-grep FAQ](https://ast-grep.github.io/advanced/faq.html)

## 3. 模块边界

下表为拟定的逻辑模块，可在原型阶段合并为少量 crate，避免过早拆包。

| 模块 | 职责 | 不负责 |
| --- | --- | --- |
| WorkspaceReader | 解析用户选定的工作区、枚举允许文件、读取原始字节、构建快照 | 修改文件、执行工程脚本 |
| SourceSnapshot | 保存原始文本、文件 hash、换行/BOM 元信息及位置索引 | 跨快照复用节点 ID |
| Analyzer | 依据明确的 SourceType 解析文件，构建 Oxc Semantic 和 import 索引 | 完整 TS 类型推断、追踪其他文件的实现 |
| RecipeCompiler | 校验 JSON 版本和字段，生成两种操作的内部配置 | 执行 recipe 内脚本或联网 |
| CandidateEngine | 识别候选、查验组件绑定、检查属性冲突、输出原因 | 直接生成磁盘副作用 |
| EditPlanner | 将可处理候选转为最小文本 edit，检测重叠，生成不可变计划 | 自动选择冲突规则的优先级 |
| PreviewValidator | 在内存中应用 edit，重新解析并生成 diff | 宣称改写后行为等价 |
| PatchExporter | 从原始快照和候选结果生成 patch 及摘要 | Alpha 阶段写回源文件 |
| ApplyService | v0.1 中重新核对 hash、应用计划、报告写入结果 | 承诺跨文件事务或覆盖并发修改 |
| SessionTransport | 本地 HTTP、任务取消、请求 generation、计划生命周期 | 将项目代码上传云端 |
| WebUI | 配置规则、选择候选、解释命中/跳过、显示 Monaco diff | 自行重新实现匹配逻辑 |

计划的数据流为：

~~~text
选择工作区与 recipe
  → 建立文件快照
  → Oxc 解析与文件内绑定分析
  → 生成候选及原因
  → 选择候选并生成 EditPlan
  → 在内存应用与检查
  → Monaco diff
  → Alpha 导出 patch / v0.1 检测冲突后应用
~~~

## 4. Recipe JSON v1：拟定协议

以下 JSON 是计划采用的协议，不是当前已支持的接口。

属性改名示例：

~~~json
{
  "schemaVersion": 1,
  "id": "button-size-to-density",
  "operation": "jsx_attribute_rename",
  "selector": {
    "moduleSpecifier": "@example/ui",
    "importedName": "Button",
    "files": ["src/**/*.tsx"]
  },
  "change": {
    "from": "size",
    "to": "density"
  },
  "onUnknown": "skip"
}
~~~

固定字符串值映射示例：

~~~json
{
  "schemaVersion": 1,
  "id": "button-small-to-compact",
  "operation": "jsx_string_value_map",
  "selector": {
    "moduleSpecifier": "@example/ui",
    "importedName": "Button",
    "files": ["src/**/*.tsx"]
  },
  "change": {
    "attribute": "size",
    "from": "small",
    "to": "compact"
  },
  "onUnknown": "skip"
}
~~~

拟定的校验与执行约定：

- schemaVersion 只接受明确支持的整数版本；未知版本、未知字段及不适用于该 operation 的 change 字段均报错，避免拼写错误被静默忽略。
- id 是 recipe 的稳定标识；内容变化另外计算 recipeHash，不用 id 判断两个规则是否相同。
- selector.moduleSpecifier 按解析后的 import 声明字符串精确比较，不作路径归一化或包身份推断；importedName 是被导入的导出名，本地别名由 Semantic 解析。
- selector.files 是相对工作区根的包含 glob，统一使用正斜杠；绝对路径、越出工作区的路径和空范围均在配置校验阶段处理。工具同时应用默认排除范围，至少排除 Git 元数据、依赖目录与常见构建输出。
- Alpha 一次运行一份 recipe，不隐式串联多份规则。属性名必须是首版支持的合法 JSX 属性名；新旧名称或字符串相同视为无效配置。
- 字符串映射只处理显式带引号、单行的 JSX 字符串属性。表达式形式、模板字符串、标识符和计算表达式跳过；不会推断 size={"small"} 或 size={variable}。
- 含实体、特殊字符的字面量必须由原型验证解析值、原文和重新编码的一致性。Alpha 在相关用例通过前应跳过这些形式，不可使用 JavaScript JSON 转义代替 JSX 属性编码。
- onUnknown 在首版只接受 skip。未知来源或不支持形式必须出现在结果摘要中，不转换成“已安全检查”。
- 修改规则语义、支持范围或匹配结果的依赖更新需要评估 schema/engine 兼容性；已生成的计划不得自动使用新的引擎重解释。

## 5. 文件快照与数据契约

计划使用以下数据结构。字段名属于拟定接口，实现时可补充字段，但需要维护可序列化版本与兼容测试。

| 数据 | 必要字段 | 约束 |
| --- | --- | --- |
| WorkspaceSnapshot | snapshotId、文件清单、创建时间 | 对应一次不可变扫描输入；创建时间不作为一致性依据 |
| FileSnapshot | relativePath、sourceHash、原始 UTF-8 文本、换行/BOM 元信息 | hash 以原始字节为输入；不能只比较 mtime 或文件大小 |
| Candidate | candidateId、snapshotId、relativePath、sourceHash、目标范围、status、reasonCode、evidence | evidence 包含命中的 import、绑定关系和属性位置；不保存可跨快照复用的裸 SymbolId |
| TextEdit | startByte、endByte、expectedText、replacement | 使用原始文本的 UTF-8 半开区间；相对同一个 FileSnapshot |
| EditPlan | planVersion、planId、snapshotId、recipeHash、engineIdentity、选中候选、按文件分组的 edit、检查结果 | 生成后不可变；包含所有参与文件的源 hash |
| RunSummary | scanned、matched、ready、skipped、conflicted、invalidFiles、changedFiles | 各计数口径固定，零命中不等于所有语义情况均已支持 |

内容 hash 算法将在技术原型中固定；计划同时固定算法标识，避免将不同算法的结果直接比较。engineIdentity 至少记录核心版本、recipe 协议版本、Oxc 版本及影响解析的配置。

Alpha 建议将项目源代码视图设为只读，减少浏览器编辑缓冲区和磁盘同时变化的同步范围。工作区的外部变化仍会使旧快照和计划过期。未来若允许修改源文本，编辑器 modelVersion 与 sourceHash 必须一起进入协议。

## 6. 组件匹配与跳过原因

计划按以下顺序判断：

1. 确认文件可读取、UTF-8 有效、在范围内，且按预定语法模式解析无错误。
2. 收集匹配 moduleSpecifier 和 importedName 的值导入声明，取得本地 BindingIdentifier 的 SymbolId。
3. 遍历 JSX opening element，仅对组件名称引用解析到该 SymbolId 的节点继续处理。
4. 检查操作对应的属性结构、重复属性、目标属性冲突和 spread。
5. 生成最小 edit 与说明，重新解析内存结果。

Oxc JSX AST 区分小写内建标签和组件 IdentifierReference，因此可以基于绑定识别组件，而不只是比较标签字符串。[JSXElementName API](https://docs.rs/oxc_ast/latest/oxc_ast/ast/enum.JSXElementName.html)

~~~tsx
import { Button as B } from "@example/ui";

<B size="small" />; // 计划命中：B 绑定到所选 import。

function Example(B: LocalComponent) {
  return <B size="small" />; // 计划排除：B 绑定到局部参数。
}
~~~

状态与原因计划分层表示，前端展示可读文案，CLI/JSON 保留稳定 reasonCode：

| 状态 | 典型 reasonCode | 含义 |
| --- | --- | --- |
| ready | direct_import_binding_confirmed | 所有当前操作所需条件均确认，可进入修改计划 |
| not_matched | module_not_selected、different_binding、attribute_not_present、value_not_selected | 已分析但不满足规则；聚合计数即可，避免为每个普通节点生成噪声 |
| skipped | unsupported_import_form、type_only_import、jsx_spread_present、unsupported_value_expression、unsupported_literal_encoding、unsupported_component_indirection | 当前能力不能确定，保留理由 |
| conflicted | duplicate_source_attribute、target_attribute_exists、overlapping_edits、snapshot_changed | 继续应用可能不明确或覆盖更新内容 |
| invalid | invalid_recipe、unreadable_file、invalid_utf8、parse_error、semantic_syntax_error、invalid_generated_syntax | 输入或候选结果不可进入应用流程 |

任何包含 JSX spread 的目标元素在 Alpha 中整体跳过，以免把未知对象属性与显式属性的覆盖关系误判为安全迁移。已有目标属性时不自动合并或覆盖。语法错误文件整体不生成 edit，不能依靠错误恢复树继续写入。

## 7. 最小文本修改与坐标

计划以原始文本为事实来源，AST 用于定位和判断，不默认重新打印整份文件：

- 属性改名只替换属性名 span；值映射只替换已确认的字符串内容，保留原有引号形式。
- edit 应包含 expectedText；应用前验证边界、原文和源 hash。所有边界必须是合法 UTF-8 字符边界。
- 同一文件的 edit 先排序并验证不重叠，再按从后向前的顺序应用。重复 edit 只能在内容完全一致且来源说明可合并时去重；不同 edit 重叠应报冲突。
- 未被 edit 覆盖的字节必须保持一致，包括空格、注释、换行和文件末尾换行。BOM 与 CRLF 的位置转换要由 fixture 验证，不能在读取阶段静默规范化。
- 生成代码需重新解析并执行所需语法检查；检查通过只证明所声明的结构条件，不证明业务行为等价。

Oxc Span 使用文本偏移，Rust 路径应统一保留 UTF-8 字节坐标。Oxc 提供 UTF-8/UTF-16 转换实现，可作为 UI 适配基础。[Span API](https://docs.rs/oxc/latest/oxc/span/struct.Span.html)、[Oxc 转换源码](https://github.com/oxc-project/oxc/blob/main/crates/oxc_ast_visit/src/utf8_to_utf16/mod.rs)

Monaco 侧使用其文本模型的位置/offset 接口，在边界处换算成 UTF-16 code unit。需要单独处理 BOM、换行处理与代理对；UI 展示文本与分析文本的差异必须进入映射，不能将 Rust byte offset 直接作为 Monaco offset。不得原地把核心 AST span 全部变为 UTF-16 后继续切 Rust 字符串。[Monaco/VS Code 文本模型接口](https://github.com/microsoft/vscode/blob/main/src/vs/editor/common/model.ts)

## 8. 请求顺序、缓存与计划过期

每次配置变化或重新扫描计划生成新的 requestGeneration。响应必须携带 generation、snapshotId、recipeHash；UI 只接收当前 generation 的结果。取消是尽力而为，正确性依赖过期结果不被接受，而不是假定取消一定立即生效。

缓存键计划包含 sourceHash、SourceType、解析配置、engineIdentity 和 recipeHash。文件与 recipe 不变时可复用仍然有效的结果；不在 Alpha 引入复杂的增量 AST 持久化。SymbolId、ReferenceId 和 AST 节点 ID 只在本次分析中有效。

选择候选后生成的 EditPlan 必须引用相同的快照；配置变化、文件变化或候选集合变化均需新计划。导出 patch 也需标明依据的原始内容，旧计划不能在 UI 中继续显示为最新结果。

## 9. Alpha 导出与 v0.1 应用

Alpha 计划仅生成 patch 和摘要，源文件保持未写入状态。导出内容应只包含选定、验证通过的修改，不能混入工作区已有的其他变更；不自动提交、暂存或运行 Git 清理操作。

v0.1 的 ApplyService 计划增加：

1. 对所有目标文件重新读取并核对完整内容 hash；任何变化都使计划失效，不自动重定位旧 edit。
2. 预检全部文件的路径和 edit 后，按明确顺序执行写入；单文件使用同目录临时文件和替换机制，减少半写文件。
3. 记录每个文件的原 hash、新 hash 和应用状态；发生错误时准确报告哪些文件已写入、哪些未写入。
4. 回滚或恢复必须再次检查当前文件仍等于本工具写入结果，不能用旧备份覆盖其他程序的新修改。

hash 检查和原子文件替换不是通用 compare-and-swap，也不是跨文件事务。外部编辑器在检查与替换之间写入仍可能产生竞态。直接应用将明确采用乐观并发保护；严格隔离的批量流程可使用独立工作副本及 patch 导出，而不承诺消除所有外部并发。

## 10. 本地 HTTP 基本边界

计划默认只监听回环地址，由 CLI 为当前工作区启动临时会话。浏览器页面与 API 同源，使用随机会话凭据；拒绝非预期 Host/Origin、跨域请求和无会话请求，防止其他网页借用本地服务读取或修改工程。

API 只接受工作区内的相对文件标识，不提供任意路径读写接口。路径规范化后必须仍在所选工作区内；Alpha 默认不跟随越界符号链接。设置请求体、文件大小、文件数量及任务时长的合理上限，关闭会话时取消后台任务。

recipe 只包含数据，不执行 JavaScript、shell 或远程插件。运行项目 typecheck/test 属于后续独立验证动作，不是打开页面或扫描目录的隐式副作用。本地源码、recipe 和 diff 默认不上传，公开样例使用专门准备的可分发代码。

## 11. 延期的跨文件语义

首版来源校验确认的是“这个引用绑定到当前文件中的指定 import 声明”。它不确认 moduleSpecifier 在 tsconfig paths、包 exports 或构建插件处理后最终指向哪个物理模块。

Oxc Resolver 已支持路径别名、tsconfig extends 和 project references，但 resolver 只解决模块定位的一部分；还需处理 re-export、barrel、export-star 冲突、条件导出和循环依赖等，才能宣称追踪符号来源。[Oxc Resolver](https://github.com/oxc-project/oxc-resolver)

完整 TypeScript 类型推断和项目编译检查同样不属于首版核心。Oxc 官方 TS 转换文档明确记录逐文件转换及类型推断限制；后续可接入项目原有 typecheck 作为额外验证，而不是将 Semantic 的存在视为类型安全证明。[TypeScript 转换边界](https://oxc.rs/docs/guide/usage/transformer/typescript.html)

## 12. 后续 WASM 与依赖固定

未来公开样例计划把同一 Rust core 编译为 WASM，运行在 Web Worker 中。core 只接受文本快照与 recipe、返回候选和 edit；文件系统、HTTP、线程调度与 patch 下载留在外层适配器。

原生版和 WASM 版计划共享 recipe schema、匹配逻辑、原因代码及 golden fixture。WASM 可采用不同并发策略，但不能单独复制匹配算法或使用另一套未经对齐的解析版本。公开样例仅处理用户主动输入或项目准备的样例文本，不承诺浏览器直接操作任意本地工程。

RWB-001 的工具链与 Rust 依赖已在根 manifest、rust-toolchain.toml 和 Cargo.lock 中固定，并通过本地原生编译与 fixture 验证。前端依赖、WASM、CI 和打包工具尚未建立，在对应任务实施时固定并验证。依赖升级只重跑与解析、绑定、坐标、渲染或打包变化相关的检查；规则和文档措辞修改按最小充分验证处理。
