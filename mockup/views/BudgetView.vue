<script setup lang="ts">
import { ref, computed } from 'vue'

// Reactive state for budget items
const budgetItems = ref([
  { id: 1, category: 'Venue', estimated: 5000, actual: 5200 },
  { id: 2, category: 'Catering', estimated: 8000, actual: null },
  { id: 3, category: 'Photography', estimated: 2500, actual: 2500 },
])

// Form state
const newCategory = ref('')
const newEstimated = ref<number | null>(null)

// Computed totals
const totalBudget = computed(() =>
  budgetItems.value.reduce((sum, item) => sum + item.estimated, 0)
)

const totalSpent = computed(() =>
  budgetItems.value.reduce((sum, item) => sum + (item.actual ?? 0), 0)
)

const remaining = computed(() => totalBudget.value - totalSpent.value)

function addItem() {
  if (!newCategory.value || !newEstimated.value) return
  budgetItems.value.push({
    id: Date.now(),
    category: newCategory.value,
    estimated: newEstimated.value,
    actual: null,
  })
  newCategory.value = ''
  newEstimated.value = null
}

function removeItem(id: number) {
  budgetItems.value = budgetItems.value.filter((item) => item.id !== id)
}

function getDifference(item: { estimated: number; actual: number | null }) {
  if (item.actual === null) return { text: `$${item.estimated.toLocaleString()} under`, class: 'under' }
  const diff = item.actual - item.estimated
  if (diff > 0) return { text: `$${diff.toLocaleString()} over`, class: 'over' }
  if (diff < 0) return { text: `$${Math.abs(diff).toLocaleString()} under`, class: 'under' }
  return { text: '$0 under', class: 'under' }
}
</script>

<template>
  <div>
    <h1 class="page-title">Budget</h1>
    <p class="page-subtitle">Track your wedding expenses and stay on budget</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Budget</div>
        <div class="value">${{ totalBudget.toLocaleString() }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Total Spent</div>
        <div class="value">${{ totalSpent.toLocaleString() }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Remaining</div>
        <div class="value green">${{ remaining.toLocaleString() }}</div>
      </div>
    </div>

    <!-- Add Budget Item Form -->
    <div class="form-section">
      <h3>Add Budget Item</h3>
      <div class="form-row">
        <input
          v-model="newCategory"
          class="form-input"
          placeholder="Category"
          @keyup.enter="addItem"
        />
        <input
          v-model.number="newEstimated"
          class="form-input"
          placeholder="Estimated Cost"
          type="number"
          @keyup.enter="addItem"
        />
        <button class="btn-add" @click="addItem">+ Add</button>
      </div>
    </div>

    <!-- Budget Table -->
    <table class="data-table">
      <thead>
        <tr>
          <th>Category</th>
          <th>Estimated</th>
          <th>Actual</th>
          <th>Difference</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in budgetItems" :key="item.id">
          <td>{{ item.category }}</td>
          <td>${{ item.estimated.toLocaleString() }}</td>
          <td>
            <input
              v-model.number="item.actual"
              class="form-input"
              type="number"
              style="width: 100px; padding: 6px 10px;"
            />
          </td>
          <td :class="getDifference(item).class === 'over' ? 'text-over' : 'text-under'">
            {{ getDifference(item).text }}
          </td>
          <td>
            <button class="delete-btn" @click="removeItem(item.id)">🗑</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.text-over {
  color: #E91E63;
}

.text-under {
  color: #4CAF50;
}
</style>
