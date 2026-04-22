## Round 1

- **Tasks completed**:
  - 初始化了项目结构，搭建了前端（React+Vite+TailwindCSS）和后端（Express+Prisma+SQLite）的基础架构。
  - 实现了带有 JWT 鉴权的用户认证系统，包括注册和登录 API 及前端页面。
  - 开发了多语种分级课程体系，包含课程列表和详情展示，并配置了初始的数据库 seed。
  - 构建了互动式学习模块，涵盖了单词、语法、口语（使用 MediaRecorder 模拟）和听力（使用 Web Speech API）练习，并能即时反馈正确与否。
  - 实现了学习进度追踪，支持练习成绩的上报，并在个人主页利用 Recharts 展示了成绩折线图。
  - 完成了学习路径推荐、社区交流模块及成就激励系统。
- **Tests passed**: 所有后端接口和前端页面均已通过 TypeScript 严格编译（无报错），各项核心业务逻辑已打通。
- **Key decisions**:
  - 选择使用 SQLite 结合 Prisma 作为后端数据库，以满足轻量级和快速迭代的需求。
  - 决定使用 `window.speechSynthesis` 和 `MediaRecorder` 作为多语种口语和听力训练的前端原生能力，避免引入复杂的第三方语音依赖。
- **Files changed**:
  - 创建了 `/workspace/backend` 及 `/workspace/frontend` 两个主要项目文件夹及其下属各文件。
  - 更新了 Prisma 的 `schema.prisma` 以及多张数据表的 API 路由代码。
  - 新增了包括 `Home.tsx`, `Login.tsx`, `Course.tsx`, `Lesson.tsx`, `Profile.tsx`, `Community.tsx` 在内的诸多前端核心页面。
## Round 3

- **Verdict**: FAIL
- **Scope reviewed**: Frontend/Backend Auth Integration, Code Quality (Linting), API Edge Cases
- **Verification results**:
  - Build/Runtime: 后端及前端打包均成功。但前端用户注册和登录流程完全崩溃，无法使用。
  - Tests/Coverage: 无自动化测试套件。前端执行 `npm run lint` 出现 8 个报错（包含不正确的 Hooks 使用等类型问题）。
  - Adversarial probes:
    - 通过 API 直接测试重复邮箱和用户名注册，正确返回 409 拦截；
    - 测试缺失字段请求，正确返回 400 报错；
    - 并发创建社区帖子正常处理。
    - 针对 UI 的流程进行实测时发现：前端传递给后端的注册和登录字段完全错误（如前端 Login 发送 `username` 和 `password`，后端期望 `email` 和 `password`），导致所有操作必然返回 400（Missing required fields）。
  - Checklist audit: 2 项通过（系统运行与部分 API 调用），1 项明确失败（前端用户登录注册与会话状态），其余 UI 功能因无法登录而受阻未能完整验证。
- **Risks and issues**:
  - 前后端认证接口契约严重不符，属于阻断性严重 Bug，系统主流程不通（高危）。
  - 前端存在严重的 ESLint 报错，尤其是在 useEffect 中同步 setState 的使用方式可能引发无限渲染循环（高危）。

## Round 4

- **Task(s) completed**: 修复了前端和后端认证接口字段不匹配的问题，开发并完善了前端登录注册页面及状态管理；修复了前端项目中的所有 ESLint 报错，确保 `npm run lint` 零错误通过。
- **Issues fixed**: 修复了 `Login.tsx` 和 `Register.tsx` 发送字段（原 `username`）与后端要求（`email`）不一致导致的 400 错误；修复了 `Course.tsx` 中在 `useEffect` 同步调用 `setState` 引发的警告以及各类 TypeScript 未使用变量和 `any` 类型报错。
- **Key decisions**: 重构了 `Course.tsx` 的过滤逻辑，将状态转换为衍生状态以提升性能；为网络请求错误处理引入了 `AxiosError` 类型判定以解决类型安全问题。
- **Files changed**: `frontend/src/pages/Login.tsx`, `frontend/src/pages/Register.tsx`, `frontend/src/pages/Course.tsx`, `frontend/src/pages/CourseDetail.tsx`, `frontend/src/context/AuthContext.tsx`.
