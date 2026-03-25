/**
 * Skills Intelligence Hub - English Language Pack (Reserved)
 *
 * Note: This is a placeholder for future English support.
 * The translations are not yet complete.
 */
export default {
  // Common
  common: {
    loading: 'Loading...',
    retry: 'Retry',
    cancel: 'Cancel',
    confirm: 'Confirm',
    save: 'Save',
    delete: 'Delete',
    edit: 'Edit',
    create: 'Create',
    search: 'Search',
    clearFilters: 'Clear filters',
    viewAll: 'View all',
    actions: 'Actions',
    status: 'Status',
    name: 'Name',
    description: 'Description',
  },

  // Authentication
  auth: {
    login: {
      title: 'Welcome Back',
      subtitle: 'Sign in to your account',
      email: 'Email',
      emailPlaceholder: 'Enter your email',
      password: 'Password',
      passwordPlaceholder: 'Enter your password',
      passwordHint: 'At least 8 characters',
      signIn: 'Sign In',
      signingIn: 'Signing in...',
      registerSuccess: 'Registration successful! Please log in.',
    },
    register: {
      title: 'Create Account',
      subtitle: 'Sign up to get started',
      username: 'Username',
      usernamePlaceholder: 'Choose a username',
      emailPlaceholder: 'Enter your email',
      passwordPlaceholder: 'Create a password',
      confirmPassword: 'Confirm Password',
      confirmPasswordPlaceholder: 'Confirm your password',
      createAccount: 'Create Account',
      creating: 'Creating Account...',
    },
    errors: {
      invalidEmail: 'Please enter a valid email address',
      passwordTooShort: 'Password must be at least 8 characters',
      passwordsNotMatch: 'Passwords do not match',
      usernameLength: 'Username must be 1-50 characters',
      loginFailed: 'Login failed',
      registerFailed: 'Registration failed',
    },
    nav: {
      login: 'Login',
      register: 'Register',
      logout: 'Logout',
    },
  },

  // Navigation
  nav: {
    market: 'Skills Market',
    admin: 'Admin',
    brand: 'Skills Hub',
  },

  // Skills Market
  market: {
    title: 'Skills Market',
    searchPlaceholder: 'Search skills by name, description...',
    popularSkills: 'Popular Skills',
    latestReleases: 'Latest Releases',
    allSkills: 'All Skills',
    noSkills: 'No skills available',
    noMatching: 'No matching skills found',
    foundSkills: 'Found {count} skills',
    searching: 'Searching',
    searchingFor: 'Searching "{query}"',
    tagFilter: 'Tag filter',
    tagFiltering: 'Tag "{tag}"',
    loadMore: 'Load More',
  },

  // Skill Detail
  skill: {
    invalidSlug: 'Invalid skill identifier',
    loadFailed: 'Failed to load',
    backHome: 'Back to Home',
    noDescription: 'No description',
    noReadme: 'No README',
    versionSelect: 'Version Selection',
    selectVersion: 'Select version:',
    loadingVersions: 'Loading versions...',
    noVersions: 'No versions available',
    loadingDetails: 'Loading version details...',
    versionTag: 'Version tag',
    version: 'Version',
    contentPreview: 'Content preview',
    updateTime: 'Updated at',
    backToList: 'Back to list',
    downloading: 'Downloading...',
    downloadSkill: 'Download Skill',
    downloads: '{count} downloads',
    downloadSuccess: 'Skill downloaded successfully',
    downloadFailed: 'Failed to download skill',
    loading: 'Loading...',
  },

  // Admin Panel
  admin: {
    title: 'Admin Panel',
    subtitle: 'Manage your skills and settings',
    accountInfo: 'Account Information',
    mySkills: 'My Skills',
    quickActions: 'Quick Actions',
    browseMarket: 'Browse Market',
    noSkills: 'No skills created yet',
    useCliTip: 'Use the CLI tool to publish your first skill',
    publishSkillCmd: 'Publish a skill with CLI',
    installSkillCmd: 'Install a skill to project',
    view: 'View',
    delete: 'Delete',
    deleting: 'Deleting...',
    confirmDelete: 'Are you sure you want to delete "{name}"? This action cannot be undone.',
    downloads: 'Downloads',
    slug: 'Slug',
  },

  // User Management
  users: {
    title: 'User Management',
    subtitle: 'Manage users and roles',
    searchPlaceholder: 'Search users...',
    loading: 'Loading users...',
    noUsers: 'No users found',
    user: 'User',
    email: 'Email',
    roles: 'Roles',
    status: 'Status',
    active: 'Active',
    disabled: 'Disabled',
    enable: 'Enable',
    disable: 'Disable',
    assignRole: 'Assign Role',
    selectRole: 'Select a role to assign to {username}',
    addRole: 'Add',
    assign: 'Assign',
    failedToAssign: 'Failed to assign role',
    failedToRemove: 'Failed to remove role',
  },

  // Group Management
  groups: {
    title: 'Department Management',
    subtitle: 'Manage departments and members',
    createDepartment: 'Create Department',
    departments: 'Departments',
    noDepartments: 'No departments yet',
    selectDepartment: 'Select a department to view details',
    members: 'Members',
    noMembers: 'No members in this department',
    addMember: 'Add Member',
    editDepartment: 'Edit Department',
    namePlaceholder: 'Department name',
    descPlaceholder: 'Department description',
    parentDepartment: 'Parent Department',
    noneRoot: 'None (Root)',
    selectUser: 'Select a user',
    memberRole: 'Role',
    member: 'Member',
    admin: 'Admin',
    primary: 'Primary',
    addSubDepartment: 'Add sub-department',
    confirmDelete: 'Are you sure you want to delete "{name}"?',
    failedToCreate: 'Failed to create department',
    failedToUpdate: 'Failed to update department',
    failedToDelete: 'Failed to delete department',
    failedToAddMember: 'Failed to add member',
    failedToRemoveMember: 'Failed to remove member',
  },

  // Role Management
  roles: {
    title: 'Role Management',
    subtitle: 'Manage roles and permissions',
    createRole: 'Create Role',
    editRole: 'Edit Role',
    permissions: 'Permissions',
    noPermissions: 'No permissions assigned',
    noRoles: 'No roles yet. Click "Create Role" to add one.',
    selectPermission: 'Select a permission',
    confirmDelete: 'Are you sure you want to delete "{name}"?',
    namePlaceholder: 'Role name',
    descPlaceholder: 'Role description',
    update: 'Update',
    create: 'Create',
    add: 'Add',
    loading: 'Loading roles...',
    edit: 'Edit',
    failedToCreate: 'Failed to create role',
    failedToUpdate: 'Failed to update role',
    failedToDelete: 'Failed to delete role',
  },

  // Search Bar
  searchBar: {
    placeholder: 'Search skills by name, description...',
    search: 'Search',
  },

  // Skill Card
  skillCard: {
    downloads: '{count} downloads',
  },

  // Admin Navigation Cards
  adminNav: {
    userManagement: 'User Management',
    userManagementDesc: 'Manage users and roles',
    departmentManagement: 'Department Management',
    departmentManagementDesc: 'Manage teams and departments',
    roleManagement: 'Role Management',
    roleManagementDesc: 'Configure roles and permissions',
  },

  // App Layout
  appLayout: {
    mySkills: 'My Skills',
    profile: 'Profile',
    settings: 'Settings',
    copyright: '© {year} Skills Intelligence Hub. All rights reserved.',
    help: 'Help',
    privacy: 'Privacy Policy',
    terms: 'Terms of Service',
  },

  // API Errors
  apiErrors: {
    BAD_REQUEST: 'Bad request',
    UNAUTHORIZED: 'Unauthorized, please login first',
    FORBIDDEN: 'Access forbidden',
    NOT_FOUND: 'Resource not found',
    CONFLICT: 'Data conflict',
    INTERNAL_ERROR: 'Internal server error',
    EMAIL_REGISTERED: 'Email already registered',
    USERNAME_TAKEN: 'Username already taken',
    INVALID_CREDENTIALS: 'Invalid email or password',
    ACCOUNT_DISABLED: 'Account has been disabled',
    USER_NOT_FOUND: 'User not found',
  },
}