/**
 * Skills Intelligence Hub - 中文语言包
 */
export default {
  // 通用
  common: {
    loading: '加载中...',
    retry: '重试',
    cancel: '取消',
    confirm: '确认',
    save: '保存',
    delete: '删除',
    edit: '编辑',
    create: '创建',
    search: '搜索',
    clearFilters: '清除筛选',
    viewAll: '查看全部',
    actions: '操作',
    status: '状态',
    name: '名称',
    description: '描述',
  },

  // 认证
  auth: {
    // 登录页
    login: {
      title: '欢迎回来',
      subtitle: '登录您的账户',
      email: '邮箱',
      emailPlaceholder: '请输入邮箱',
      password: '密码',
      passwordPlaceholder: '请输入密码',
      passwordHint: '至少 8 个字符',
      signIn: '登录',
      signingIn: '登录中...',
      registerSuccess: '注册成功！请登录。',
    },
    // 注册页
    register: {
      title: '创建账户',
      subtitle: '注册开始使用',
      username: '用户名',
      usernamePlaceholder: '请输入用户名',
      emailPlaceholder: '请输入邮箱',
      passwordPlaceholder: '请设置密码',
      confirmPassword: '确认密码',
      confirmPasswordPlaceholder: '请再次输入密码',
      createAccount: '创建账户',
      creating: '创建中...',
    },
    // 错误消息
    errors: {
      invalidEmail: '请输入有效的邮箱地址',
      passwordTooShort: '密码至少需要 8 个字符',
      passwordsNotMatch: '两次密码输入不一致',
      usernameLength: '用户名长度应为 1-50 个字符',
      loginFailed: '登录失败',
      registerFailed: '注册失败',
    },
    // 导航
    nav: {
      login: '登录',
      register: '注册',
      logout: '退出',
    },
  },

  // 导航
  nav: {
    market: '技能市场',
    admin: '管理',
    brand: 'Skills Hub',
  },

  // 技能市场
  market: {
    title: '技能市场',
    searchPlaceholder: '搜索技能名称、描述...',
    popularSkills: '热门技能',
    latestReleases: '最新发布',
    allSkills: '全部技能',
    noSkills: '暂无技能',
    noMatching: '未找到匹配的技能',
    foundSkills: '找到 {count} 个技能',
    searching: '搜索中',
    searchingFor: '搜索 "{query}"',
    tagFilter: '标签筛选',
    tagFiltering: '标签 "{tag}"',
    loadMore: '加载更多',
  },

  // 技能详情
  skill: {
    invalidSlug: '无效的技能标识',
    loadFailed: '加载失败',
    backHome: '返回首页',
    noDescription: '暂无描述',
    noReadme: '暂无说明',
    versionSelect: '版本选择',
    selectVersion: '选择版本:',
    loadingVersions: '加载版本列表...',
    noVersions: '暂无可用版本',
    loadingDetails: '加载版本详情...',
    versionTag: '版本标签',
    version: '版本',
    contentPreview: '内容预览',
    updateTime: '更新时间',
    backToList: '返回列表',
    downloading: '下载中...',
    downloadSkill: '下载技能',
    downloads: '{count} 次下载',
    downloadSuccess: '技能下载成功',
    downloadFailed: '技能下载失败',
    loading: '加载中...',
  },

  // 管理面板
  admin: {
    title: '管理面板',
    subtitle: '管理您的技能和设置',
    accountInfo: '账户信息',
    mySkills: '我的技能',
    quickActions: '快速操作',
    browseMarket: '浏览市场',
    noSkills: '暂无创建的技能',
    useCliTip: '使用 CLI 工具发布您的第一个技能',
    publishSkillCmd: '使用 CLI 发布技能',
    installSkillCmd: '安装技能到项目',
    view: '查看',
    delete: '删除',
    deleting: '删除中...',
    confirmDelete: '确定要删除 "{name}" 吗？此操作不可撤销。',
    downloads: '下载量',
    slug: '标识',
  },

  // 用户管理
  users: {
    title: '用户管理',
    subtitle: '管理用户和角色',
    searchPlaceholder: '搜索用户...',
    loading: '加载用户列表...',
    noUsers: '暂无用户',
    user: '用户',
    email: '邮箱',
    roles: '角色',
    status: '状态',
    active: '已激活',
    disabled: '已禁用',
    enable: '启用',
    disable: '禁用',
    assignRole: '分配角色',
    selectRole: '选择要分配给 {username} 的角色',
    addRole: '添加角色',
    assign: '分配',
    failedToAssign: '分配角色失败',
    failedToRemove: '移除角色失败',
  },

  // 组织管理
  groups: {
    title: '组织管理',
    subtitle: '管理部门和成员',
    createDepartment: '创建部门',
    departments: '部门',
    noDepartments: '暂无部门',
    selectDepartment: '选择部门查看详情',
    members: '成员',
    noMembers: '该部门暂无成员',
    addMember: '添加成员',
    editDepartment: '编辑部门',
    namePlaceholder: '部门名称',
    descPlaceholder: '部门描述',
    parentDepartment: '上级部门',
    noneRoot: '无（顶级）',
    selectUser: '选择用户',
    memberRole: '成员角色',
    member: '成员',
    admin: '管理员',
    primary: '主管理员',
    addSubDepartment: '添加子部门',
    confirmDelete: '确定要删除 "{name}" 吗？',
    failedToCreate: '创建部门失败',
    failedToUpdate: '更新部门失败',
    failedToDelete: '删除部门失败',
    failedToAddMember: '添加成员失败',
    failedToRemoveMember: '移除成员失败',
  },

  // 角色管理
  roles: {
    title: '角色管理',
    subtitle: '管理角色和权限',
    createRole: '创建角色',
    editRole: '编辑角色',
    permissions: '权限',
    noPermissions: '暂无分配权限',
    noRoles: '暂无角色，点击"创建角色"添加',
    selectPermission: '选择权限',
    confirmDelete: '确定要删除 "{name}" 吗？',
    namePlaceholder: '角色名称',
    descPlaceholder: '角色描述',
    update: '更新',
    create: '创建',
    add: '添加',
    loading: '加载角色中...',
    edit: '编辑',
    failedToCreate: '创建角色失败',
    failedToUpdate: '更新角色失败',
    failedToDelete: '删除角色失败',
  },

  // 搜索栏
  searchBar: {
    placeholder: '搜索技能名称、描述...',
    search: '搜索',
  },

  // 技能卡片
  skillCard: {
    downloads: '{count} 次下载',
  },

  // 管理面板导航卡片
  adminNav: {
    userManagement: '用户管理',
    userManagementDesc: '管理用户和角色',
    departmentManagement: '部门管理',
    departmentManagementDesc: '管理团队和部门',
    roleManagement: '角色管理',
    roleManagementDesc: '配置角色和权限',
  },

  // 应用布局
  appLayout: {
    mySkills: '我的技能',
    profile: '个人中心',
    settings: '设置',
    copyright: '© {year} Skills Intelligence Hub. 保留所有权利。',
    help: '帮助文档',
    privacy: '隐私政策',
    terms: '服务条款',
  },

  // API 错误消息（前端显示）
  apiErrors: {
    BAD_REQUEST: '请求参数错误',
    UNAUTHORIZED: '未授权，请先登录',
    FORBIDDEN: '禁止访问',
    NOT_FOUND: '资源不存在',
    CONFLICT: '数据冲突',
    INTERNAL_ERROR: '服务器内部错误',
    // 业务错误
    EMAIL_REGISTERED: '邮箱已被注册',
    USERNAME_TAKEN: '用户名已被使用',
    INVALID_CREDENTIALS: '邮箱或密码错误',
    ACCOUNT_DISABLED: '账户已被禁用',
    USER_NOT_FOUND: '用户不存在',
  },
}