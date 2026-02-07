<script setup lang="ts">
import { ref, computed } from 'vue'

const vendors = ref([
  {
    id: 1,
    name: 'Dalang Wedding Organizer',
    category: 'Wedding Organizer',
    phone: '0812-1111-2222',
    email: 'info@dalangwo.com',
    cost: 25000000,
    status: 'Booked',
  },
  {
    id: 2,
    name: 'Studio Foto Semarang',
    category: 'Photography',
    phone: '0813-3333-4444',
    email: 'hello@studiofoto.com',
    cost: 15000000,
    status: 'Pending',
  },
  {
    id: 3,
    name: 'Bunda Catering',
    category: 'Catering',
    phone: '0815-5555-6666',
    email: 'order@bundacatering.com',
    cost: 35000000,
    status: 'Paid',
  },
])

const newVendor = ref({ name: '', category: '', cost: null as number | null, phone: '', email: '' })

const totalCost = computed(() => vendors.value.reduce((sum, v) => sum + v.cost, 0))
const totalPaid = computed(() =>
  vendors.value.filter((v) => v.status === 'Paid').reduce((sum, v) => sum + v.cost, 0)
)

function addVendor() {
  if (!newVendor.value.name) return
  vendors.value.push({
    id: Date.now(),
    name: newVendor.value.name,
    category: newVendor.value.category,
    phone: newVendor.value.phone,
    email: newVendor.value.email,
    cost: newVendor.value.cost ?? 0,
    status: 'Pending',
  })
  newVendor.value = { name: '', category: '', cost: null, phone: '', email: '' }
}

function removeVendor(id: number) {
  vendors.value = vendors.value.filter((v) => v.id !== id)
}

function formatRupiah(value: number): string {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}
</script>

<template>
  <div>
    <h1 class="page-title">Vendors</h1>
    <p class="page-subtitle">Manage your wedding vendors and service providers</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Vendors</div>
        <div class="value">{{ vendors.length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Total Cost</div>
        <div class="value">{{ formatRupiah(totalCost) }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Paid</div>
        <div class="value green">{{ formatRupiah(totalPaid) }}</div>
      </div>
    </div>

    <!-- Add Vendor Form -->
    <div class="form-section">
      <h3>Add Vendor</h3>
      <div class="form-row">
        <input v-model="newVendor.name" class="form-input" placeholder="Vendor Name" />
        <input v-model="newVendor.category" class="form-input" placeholder="Category" />
        <input v-model.number="newVendor.cost" class="form-input" placeholder="Cost (IDR)" type="number" />
      </div>
      <div class="form-row">
        <input v-model="newVendor.phone" class="form-input" placeholder="Phone" />
        <input v-model="newVendor.email" class="form-input" placeholder="Email" />
      </div>
      <button class="btn-add" @click="addVendor">+ Add Vendor</button>
    </div>

    <!-- Vendor Cards -->
    <div class="card-grid">
      <div v-for="vendor in vendors" :key="vendor.id" class="vendor-card">
        <button class="delete-btn" @click="removeVendor(vendor.id)">🗑</button>
        <div class="vendor-name">{{ vendor.name }}</div>
        <div class="vendor-category">{{ vendor.category }}</div>
        <div class="vendor-info">📞 {{ vendor.phone }}</div>
        <div class="vendor-info">✉️ {{ vendor.email }}</div>
        <div class="vendor-info">💰 {{ formatRupiah(vendor.cost) }}</div>
        <div style="margin-top: 12px;">
          <label class="status-label">Status</label>
          <select v-model="vendor.status" class="status-select" :class="vendor.status.toLowerCase()">
            <option>Pending</option>
            <option>Booked</option>
            <option>Paid</option>
          </select>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.status-label {
  font-size: 12px;
  color: #888;
  display: block;
  margin-bottom: 4px;
}

.status-select {
  width: 100%;
  padding: 8px 12px;
  border-radius: 8px;
  border: none;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
}

.status-select.booked {
  background: #E8F5E9;
  color: #2E7D32;
}

.status-select.pending {
  background: #FFF9C4;
  color: #F57F17;
}

.status-select.paid {
  background: #E8F5E9;
  color: #2E7D32;
}
</style>
