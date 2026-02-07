<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const navItems = [
  { path: '/budget', label: 'Budget', icon: '$' },
  { path: '/venue', label: 'Venue', icon: '◎' },
  { path: '/rentals', label: 'Rentals', icon: '⚙' },
  { path: '/guest-list', label: 'Guest List', icon: '👥' },
  { path: '/seating-plan', label: 'Seating Plan', icon: '⊞' },
  { path: '/todo-list', label: 'To-Do List', icon: '☰' },
  { path: '/vendors', label: 'Vendors', icon: '🏠' },
  { path: '/food-beverages', label: 'Food & Beverages', icon: '🍴' },
  { path: '/notes', label: 'Notes', icon: '📝' },
]
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="heart-icon">💖</span>
      <div>
        <div class="brand-title">Wedding Planner</div>
        <div class="brand-subtitle">Plan your perfect day</div>
      </div>
    </div>

    <nav class="sidebar-nav">
      <router-link
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ active: route.path === item.path }"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </router-link>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  position: fixed;
  left: 0;
  top: 0;
  width: 240px;
  height: 100vh;
  background: #fff;
  border-right: 1px solid #f0f0f0;
  padding: 24px 0;
  overflow-y: auto;
}

.sidebar-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 20px;
  margin-bottom: 32px;
}

.heart-icon {
  font-size: 28px;
}

.brand-title {
  font-size: 16px;
  font-weight: 700;
  color: #1a1a1a;
}

.brand-subtitle {
  font-size: 12px;
  color: #999;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  text-decoration: none;
  color: #555;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background: #FFF0F3;
  color: #E91E63;
}

.nav-item.active {
  background: #FFF0F3;
  color: #E91E63;
  border-left-color: #E91E63;
}

.nav-icon {
  font-size: 18px;
  width: 24px;
  text-align: center;
}
</style>
