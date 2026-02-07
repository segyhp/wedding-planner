<script setup lang="ts">
import { ref, computed } from 'vue'

const rentals = ref([
  { id: 1, item: 'Round Tables', quantity: 50, unitCost: 150000, vendor: 'Dalang WO', status: 'Confirmed' },
  { id: 2, item: 'Gold Chairs', quantity: 500, unitCost: 25000, vendor: 'Dalang WO', status: 'Confirmed' },
  { id: 3, item: 'Table Linens', quantity: 50, unitCost: 75000, vendor: 'Decor House', status: 'Pending' },
  { id: 4, item: 'Lighting Set', quantity: 1, unitCost: 5000000, vendor: 'Light Pro', status: 'Quoted' },
])

const newRental = ref({ item: '', quantity: null as number | null, unitCost: null as number | null, vendor: '' })

const totalCost = computed(() =>
  rentals.value.reduce((sum, r) => sum + r.quantity * r.unitCost, 0)
)

function addRental() {
  if (!newRental.value.item) return
  rentals.value.push({
    id: Date.now(),
    item: newRental.value.item,
    quantity: newRental.value.quantity ?? 0,
    unitCost: newRental.value.unitCost ?? 0,
    vendor: newRental.value.vendor,
    status: 'Pending',
  })
  newRental.value = { item: '', quantity: null, unitCost: null, vendor: '' }
}

function removeRental(id: number) {
  rentals.value = rentals.value.filter((r) => r.id !== id)
}

function formatRupiah(value: number): string {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}
</script>

<template>
  <div>
    <h1 class="page-title">Rentals</h1>
    <p class="page-subtitle">Manage rental items for your wedding</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Items</div>
        <div class="value">{{ rentals.length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Total Cost</div>
        <div class="value">{{ formatRupiah(totalCost) }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Confirmed</div>
        <div class="value green">{{ rentals.filter(r => r.status === 'Confirmed').length }}</div>
      </div>
    </div>

    <!-- Add Rental Form -->
    <div class="form-section">
      <h3>Add Rental Item</h3>
      <div class="form-row">
        <input v-model="newRental.item" class="form-input" placeholder="Item Name" />
        <input v-model.number="newRental.quantity" class="form-input" placeholder="Quantity" type="number" />
        <input v-model.number="newRental.unitCost" class="form-input" placeholder="Unit Cost (IDR)" type="number" />
      </div>
      <div class="form-row">
        <input v-model="newRental.vendor" class="form-input" placeholder="Vendor" />
        <button class="btn-add" @click="addRental">+ Add Item</button>
      </div>
    </div>

    <!-- Rentals Table -->
    <table class="data-table">
      <thead>
        <tr>
          <th>Item</th>
          <th>Qty</th>
          <th>Unit Cost</th>
          <th>Subtotal</th>
          <th>Vendor</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="rental in rentals" :key="rental.id">
          <td>{{ rental.item }}</td>
          <td>{{ rental.quantity }}</td>
          <td>{{ formatRupiah(rental.unitCost) }}</td>
          <td>{{ formatRupiah(rental.quantity * rental.unitCost) }}</td>
          <td>{{ rental.vendor }}</td>
          <td>
            <span class="status-badge" :class="rental.status.toLowerCase()">{{ rental.status }}</span>
          </td>
          <td>
            <button class="delete-btn" @click="removeRental(rental.id)">🗑</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.status-badge.confirmed {
  background: #E8F5E9;
  color: #2E7D32;
}

.status-badge.quoted {
  background: #E3F2FD;
  color: #1565C0;
}
</style>
