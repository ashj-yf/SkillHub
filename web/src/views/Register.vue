<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { extractErrorMessage } from '@/api/index'
import AuthLayout from '@/design-system/layouts/AuthLayout.vue'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'

const { t } = useI18n()
const router = useRouter()
const userStore = useUserStore()

const username = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const error = ref('')

async function handleRegister() {
  error.value = ''

  // Client-side validation
  if (!username.value || username.value.length > 50) {
    error.value = t('auth.errors.usernameLength')
    return
  }

  if (!email.value || !email.value.includes('@')) {
    error.value = t('auth.errors.invalidEmail')
    return
  }

  if (password.value.length < 8) {
    error.value = t('auth.errors.passwordTooShort')
    return
  }

  if (password.value !== confirmPassword.value) {
    error.value = t('auth.errors.passwordsNotMatch')
    return
  }

  try {
    await userStore.register({
      username: username.value,
      email: email.value,
      password: password.value,
    })

    // Redirect to login page after successful registration
    router.push({ name: 'Login', query: { registered: 'true' } })
  } catch (e) {
    error.value = extractErrorMessage(e, t('auth.errors.registerFailed'))
  }
}
</script>

<template>
  <AuthLayout :title="t('auth.register.title')" :subtitle="t('auth.register.subtitle')">
    <form class="space-y-6" @submit.prevent="handleRegister">
      <!-- Error Message -->
      <div
        v-if="error"
        class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm"
      >
        {{ error }}
      </div>

      <!-- Username Input -->
      <Input
        v-model="username"
        :label="t('auth.register.username')"
        type="text"
        :placeholder="t('auth.register.usernamePlaceholder')"
        required
        :maxlength="50"
        :state="error && (!username || username.length > 50) ? 'error' : 'default'"
      />

      <!-- Email Input -->
      <Input
        v-model="email"
        :label="t('auth.login.email')"
        type="email"
        :placeholder="t('auth.register.emailPlaceholder')"
        required
        :state="error && !email.includes('@') ? 'error' : 'default'"
      />

      <!-- Password Input -->
      <Input
        v-model="password"
        :label="t('auth.login.password')"
        type="password"
        :placeholder="t('auth.register.passwordPlaceholder')"
        required
        show-password
        :hint="t('auth.login.passwordHint')"
      />

      <!-- Confirm Password Input -->
      <Input
        v-model="confirmPassword"
        :label="t('auth.register.confirmPassword')"
        type="password"
        :placeholder="t('auth.register.confirmPasswordPlaceholder')"
        required
        show-password
        :state="error && password !== confirmPassword ? 'error' : 'default'"
        :error-message="password !== confirmPassword ? t('auth.errors.passwordsNotMatch') : ''"
      />

      <!-- Submit Button -->
      <Button
        type="primary"
        block
        :loading="userStore.loading"
        native-type="submit"
      >
        {{ userStore.loading ? t('auth.register.creating') : t('auth.register.createAccount') }}
      </Button>
    </form>
  </AuthLayout>
</template>