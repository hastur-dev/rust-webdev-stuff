import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import SearchBar from '@/components/search/SearchBar.vue'

describe('SearchBar', () => {
  it('renders input with placeholder', () => {
    const wrapper = mount(SearchBar, {
      props: {
        modelValue: '',
        placeholder: 'Search here...',
      },
    })

    const input = wrapper.find('input')
    expect(input.attributes('placeholder')).toBe('Search here...')
  })

  it('emits update:modelValue on input', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' },
    })

    const input = wrapper.find('input')
    await input.setValue('test query')

    expect(wrapper.emitted('update:modelValue')?.[0]).toEqual(['test query'])
  })

  it('emits search on form submit', async () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: 'test' },
    })

    await wrapper.find('form').trigger('submit')

    expect(wrapper.emitted('search')?.[0]).toEqual(['test'])
  })

  it('uses default placeholder when none provided', () => {
    const wrapper = mount(SearchBar, {
      props: { modelValue: '' },
    })

    const input = wrapper.find('input')
    expect(input.attributes('placeholder')).toBe('Search articles...')
  })
})
