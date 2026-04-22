# Tasks
- [x] Task 1: 初始化项目结构
  - [x] SubTask 1.1: 搭建前端脚手架及路由
  - [x] SubTask 1.2: 搭建后端服务基础架构及数据库连接
- [x] Task 2: 实现用户认证系统
  - [x] SubTask 2.1: 设计用户数据库模型
  - [x] SubTask 2.2: 开发注册、登录API及JWT鉴权
  - [x] SubTask 2.3: 开发前端登录注册页面及状态管理
- [x] Task 3: 开发分级课程体系
  - [x] SubTask 3.1: 设计课程、章节、知识点数据结构
  - [x] SubTask 3.2: 开发课程列表与课程详情API
  - [x] SubTask 3.3: 构建前端课程浏览页面
- [x] Task 4: 构建互动式学习模块
  - [x] SubTask 4.1: 实现单词记忆组件与逻辑
  - [x] SubTask 4.2: 实现语法练习与题目检验功能
  - [x] SubTask 4.3: 集成语音API，实现口语跟读与听力播放模块
- [x] Task 5: 学习进度追踪
  - [x] SubTask 5.1: 开发学习记录上报与统计API
  - [x] SubTask 5.2: 在前端个人主页展示进度图表
- [x] Task 6: 个性化推荐与社区激励系统
  - [x] SubTask 6.1: 实现简单的学习路径推荐算法
  - [x] SubTask 6.2: 开发社区发帖、评论功能
  - [x] SubTask 6.3: 实现成就系统（积分、徽章发放）

- [x] Task 7: 修复前端与后端认证接口字段不匹配的问题：前端注册需增加传递 `email` 字段，前端登录需使用 `email` 字段进行认证。
- [x] Task 8: 修复前端项目中所有的 ESLint 报错，确保 `npm run lint` 零错误通过。

# Task Dependencies
- [Task 2] depends on [Task 1]
- [Task 3] depends on [Task 1]
- [Task 4] depends on [Task 3]
- [Task 5] depends on [Task 2, Task 4]
- [Task 6] depends on [Task 2, Task 5]

- [x] Task 9: 修复前端 `CourseDetail.tsx` 中的“开始学习”按钮，使其正确绑定 `onClick` 或使用 `Link` 跳转至对应的 `/lesson/:id` 路由。
- [x] Task 10: 修复前后端题目类型（`type`）枚举不一致的问题：调整 `Lesson.tsx` 以兼容后端的 `VOCAB`, `GRAMMAR`, `SPEAKING`, `LISTENING` 类型，确保题目能够正常渲染。
- [x] Task 11: 修复前端 `Community.tsx` 及其他需要在挂载时发起请求的页面中的 Token 竞态问题，确保 Axios 拦截器或请求配置能够在 `AuthContext` 注入 Token 后再进行调用。
- [x] Task 12: 优化前端 `Lesson.tsx` 学习页面体验，增加顶部导航栏或返回按钮，允许用户中途退出练习。
