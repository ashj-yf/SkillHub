import { api } from './index'

export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
}

export interface AuthResponse {
  token: string
}

export async function login(data: LoginRequest): Promise<string> {
  const response = await api.post<AuthResponse>('/auth/login', data)
  return response.data.token
}

export async function register(data: RegisterRequest): Promise<void> {
  await api.post('/auth/register', data)
}