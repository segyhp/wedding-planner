<script setup lang="ts">
import { ref, computed } from 'vue'

type MenuCategory = 'Appetizer' | 'Main Course' | 'Dessert' | 'Beverage'

interface MenuItem {
  id: number
  name: string
  category: MenuCategory
  servings: number
  costPerServing: number
}

const menuItems = ref<MenuItem[]>([
  { id: 1, name: 'Lumpia Semarang', category: 'Appetizer', servings: 200, costPerServing: 15000 },
  { id: 2, name: 'Nasi Goreng Spesial', category: 'Main Course', servings: 300, costPerServing: 35000 },
  { id: 3, name: 'Ayam Panggang', category: 'Main Course', servings: 300, costPerServing: 40000 },
  { id: 4, name: 'Kue Lapis', category: 'Dessert', servings: 200, costPerServing: 20000 },
  { id: 5, name: 'Es Teh Manis', category: 'Beverage', servings: 500, costPerServing: 5000 },
  { id: 6, name: 'Jus Buah Segar', category: 'Beverage', servings: 200, costPerServing: 12000 },
])

const newItem = ref({ name: '', category: 'Appetizer' as MenuCategory, servings: null as number | null, costPerServing: null as number | null })

const categories: MenuCategory[] = ['Appetizer', 'Main Course', 'Dessert', 'Beverage']

const totalCost = computed(() =>
  menuItems.value.reduce((sum, item) => sum + item.servings * item.costPerServing, 0)
)

function getCategoryItems(category: MenuCategory) {
  return menuItems.value.filter((item) => item.category === category)
}

function getCategoryTotal(category: MenuCategory) {
  return getCategoryItems(category).reduce((sum, item) => sum + item.servings * item.costPerServing, 0)
}

function addItem() {
  if (!newItem.value.name || !newItem.value.servings || !newItem.value.costPerServing) return
  menuItems.value.push({
    id: Date.now(),
    name: newItem.value.name,
    category: newItem.value.category,
    servings: newItem.value.servings,
    costPerServing: newItem.value.costPerServing,
  })
  newItem.value = { name: '', category: 'Appetizer', servings: null, costPerServing: null }
}

function removeItem(id: number) {
  menuItems.value = menuItems.value.filter((item) => item.id !== id)
}

function formatRupiah(value: number): string {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}
</script>

<template>
  <div>
    <h1 class="page-title">Food & Beverages</h1>
    <p class="page-subtitle">Plan your wedding menu and catering</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Catering Cost</div>
        <div class="value">{{ formatRupiah(totalCost) }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Menu Items</div>
        <div class="value">{{ menuItems.length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Categories</div>
        <div class="value green">{{ categories.filter(c => getCategoryItems(c).length > 0).length }}</div>
      </div>
    </div>

    <!-- Add Menu Item Form -->
    <div class="form-section">
      <h3>Add Menu Item</h3>
      <div class="form-row">
        <input v-model="newItem.name" class="form-input" placeholder="Item Name" />
        <select v-model="newItem.category" class="form-select">
          <option v-for="cat in categories" :key="cat" :value="cat">{{ cat }}</option>
        </select>
        <input v-model.number="newItem.servings" class="form-input" placeholder="Servings" type="number" />
        <input v-model.number="newItem.costPerServing" class="form-input" placeholder="Cost/Serving (IDR)" type="number" />
        <button class="btn-add" @click="addItem">+ Add</button>
      </div>
    </div>

    <!-- Category Cards -->
    <div class="card-grid">
      <div v-for="category in categories" :key="category" class="category-card">
        <div class="category-header">
          <span>{{ category }}s</span>
          <span class="category-total">{{ formatRupiah(getCategoryTotal(category)) }}</span>
        </div>

        <div v-if="getCategoryItems(category).length === 0" style="color: #999; font-size: 13px; font-style: italic;">
          No items yet
        </div>

        <div v-for="item in getCategoryItems(category)" :key="item.id" class="menu-item">
          <div>
            <div class="item-name">{{ item.name }}</div>
            <div class="item-detail">
              {{ item.servings }} servings x {{ formatRupiah(item.costPerServing) }} = {{ formatRupiah(item.servings * item.costPerServing) }}
            </div>
          </div>
          <button class="delete-btn" @click="removeItem(item.id)">🗑</button>
        </div>
      </div>
    </div>
  </div>
</template>
