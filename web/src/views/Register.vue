<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { extractErrorMessage } from '@/api/index'
import AuthLayout from '@/design-system/layouts/AuthLayout.vue'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'

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
    error.value = 'Username must be 1-50 characters'
    return
  }

  if (!email.value || !email.value.includes('@')) {
    error.value = 'Please enter a valid email address'
    return
  }

  if (password.value.length < 8) {
    error.value = 'Password must be at least 8 characters'
    return
  }

  if (password.value !== confirmPassword.value) {
    error.value = 'Passwords do not match'
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
    error.value = extractErrorMessage(e, 'Registration failed')
  }
}
</script>

<template>
  <AuthLayout title="Create Account" subtitle="Sign up to get started">
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
        label="Username"
        type="text"
        placeholder="Choose a username"
        required
        :maxlength="50"
        :state="error && (!username || username.length > 50) ? 'error' : 'default'"
      />

      <!-- Email Input -->
      <Input
        v-model="email"
        label="Email"
        type="email"
        placeholder="Enter your email"
        required
        :state="error && !email.includes('@') ? 'error' : 'default'"
      />

      <!-- Password Input -->
      <Input
        v-model="password"
        label="Password"
        type="password"
        placeholder="Create a password"
        required
        show-password
        hint="At least 8 characters"
      />

      <!-- Confirm Password Input -->
      <Input
        v-model="confirmPassword"
        label="Confirm Password"
        type="password"
        placeholder="Confirm your password"
        required
        show-password
        :state="error && password !== confirmPassword ? 'error' : 'default'"
        :error-message="password !== confirmPassword ? 'Passwords do not match' : ''"
      />

      <!-- Submit Button -->
      <Button
        type="primary"
        block
        :loading="userStore.loading"
        native-type="submit"
      >
        {{ userStore.loading ? 'Creating Account...' : 'Create Account' }}
      </Button>
    </form>
  </AuthLayout>
</template>