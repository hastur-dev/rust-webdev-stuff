// User types
export type UserRole = 'super_admin' | 'admin' | 'editor' | 'viewer'

export interface User {
  id: string
  username: string
  email: string
  role: UserRole
  is_active: boolean
  created_at: string
}

export interface LoginRequest {
  username: string
  password: string
}

export interface LoginResponse {
  user: User
  message: string
}

// Article types
export interface Article {
  id: string
  title: string
  content: string
  author_id: string
  is_published: boolean
  created_at: string
  updated_at: string
}

export interface CreateArticle {
  title: string
  content: string
  tags: string[]
}

export interface UpdateArticle {
  title?: string
  content?: string
  is_published?: boolean
}

// Tag types
export interface Tag {
  id: string
  name: string
  created_at: string
}

// Favorite types
export interface Favorite {
  article_id: string
  article_title: string
  favorited_at: string
}

// Audit types
export interface AuditEntry {
  id: string
  user_id: string | null
  username?: string
  action: string
  resource_type: string
  resource_id: string | null
  details: string | null
  ip_address: string | null
  user_agent: string | null
  created_at: string
}

// API response types
export interface PaginatedResponse<T> {
  items: T[]
  total: number
  offset: number
  limit: number
}

export interface ApiError {
  error: string
  status: number
}

// Search types
export interface SearchResult {
  query: string
  results: Article[]
  count: number
}
