<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { extractErrorMessage } from '@/api/index'
import AuthLayout from '@/design-system/layouts/AuthLayout.vue'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const email = ref('')
const password = ref('')
const error = ref('')

async function handleLogin() {
  error.value = ''

  if (!email.value || !email.value.includes('@')) {
    error.value = t('auth.errors.invalidEmail')
    return
  }

  if (password.value.length < 8) {
    error.value = t('auth.errors.passwordTooShort')
    return
  }

  try {
    await userStore.login({
      email: email.value,
      password: password.value,
    })

    // Redirect to original target page or home
    const redirect = route.query.redirect as string
    router.push(redirect || '/')
  } catch (e) {
    error.value = extractErrorMessage(e, t('auth.errors.loginFailed'))
  }
}
</script>

<template>
  <AuthLayout :title="t('auth.login.title')" :subtitle="t('auth.login.subtitle')">
    <form class="space-y-6" @submit.prevent="handleLogin">
      <!-- Error Message -->
      <div
        v-if="error"
        class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm"
      >
        {{ error }}
      </div>

      <!-- Success Message from Registration -->
      <div
        v-if="route.query.registered === 'true'"
        class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm"
      >
        {{ t('auth.login.registerSuccess') }}
      </div>

      <!-- Email Input -->
      <Input
        v-model="email"
        :label="t('auth.login.email')"
        type="email"
        :placeholder="t('auth.login.emailPlaceholder')"
        required
        :state="error && !email.includes('@') ? 'error' : 'default'"
      />

      <!-- Password Input -->
      <Input
        v-model="password"
        :label="t('auth.login.password')"
        type="password"
        :placeholder="t('auth.login.passwordPlaceholder')"
        required
        show-password
        :hint="t('auth.login.passwordHint')"
      />

      <!-- Submit Button -->
      <Button
        type="primary"
        block
        :loading="userStore.loading"
        native-type="submit"
      >
        {{ userStore.loading ? t('auth.login.signingIn') : t('auth.login.signIn') }}
      </Button>
    </form>
  </AuthLayout>
</template>