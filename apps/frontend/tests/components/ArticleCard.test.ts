import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import ArticleCard from '@/components/articles/ArticleCard.vue'

describe('ArticleCard', () => {
  const mockArticle = {
    id: '123',
    title: 'Test Article',
    content: '<p>This is test content for the article card.</p>',
    author_id: 'user-1',
    is_published: true,
    created_at: '2024-01-15T10:00:00Z',
    updated_at: '2024-01-15T10:00:00Z',
  }

  it('renders article title', () => {
    const wrapper = mount(ArticleCard, {
      props: { article: mockArticle },
      global: {
        stubs: ['router-link'],
      },
    })

    expect(wrapper.text()).toContain('Test Article')
  })

  it('shows draft badge for unpublished articles', () => {
    const draftArticle = { ...mockArticle, is_published: false }
    const wrapper = mount(ArticleCard, {
      props: { article: draftArticle },
      global: {
        stubs: ['router-link'],
      },
    })

    expect(wrapper.text()).toContain('Draft')
  })

  it('does not show draft badge for published articles', () => {
    const wrapper = mount(ArticleCard, {
      props: { article: mockArticle },
      global: {
        stubs: ['router-link'],
      },
    })

    expect(wrapper.text()).not.toContain('Draft')
  })

  it('truncates long content', () => {
    const longContent = { ...mockArticle, content: 'a'.repeat(300) }
    const wrapper = mount(ArticleCard, {
      props: { article: longContent },
      global: {
        stubs: ['router-link'],
      },
    })

    expect(wrapper.text()).toContain('...')
  })

  it('formats date correctly', () => {
    const wrapper = mount(ArticleCard, {
      props: { article: mockArticle },
      global: {
        stubs: ['router-link'],
      },
    })

    // Date should be formatted (locale-dependent)
    expect(wrapper.text()).toMatch(/\d{1,2}\/\d{1,2}\/\d{4}|\d{4}-\d{2}-\d{2}/)
  })
})
