<script setup lang="ts">
import { ref } from 'vue'

const venues = ref([
  {
    id: 1,
    name: 'Hotel Aruss',
    location: 'Jl. Pandanaran No. 56, Semarang',
    capacity: 500,
    cost: 25000000,
    status: 'Shortlisted',
    notes: 'Good catering options, central location',
  },
  {
    id: 2,
    name: 'Padma Hotel Semarang',
    location: 'Jl. Sisingamangaraja No. 16, Semarang',
    capacity: 350,
    cost: 35000000,
    status: 'Visited',
    notes: 'Elegant ballroom, premium packages',
  },
])

const newVenue = ref({ name: '', location: '', capacity: null as number | null, cost: null as number | null })

function addVenue() {
  if (!newVenue.value.name) return
  venues.value.push({
    id: Date.now(),
    name: newVenue.value.name,
    location: newVenue.value.location,
    capacity: newVenue.value.capacity ?? 0,
    cost: newVenue.value.cost ?? 0,
    status: 'Pending',
    notes: '',
  })
  newVenue.value = { name: '', location: '', capacity: null, cost: null }
}

function removeVenue(id: number) {
  venues.value = venues.value.filter((v) => v.id !== id)
}

function formatRupiah(value: number): string {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}
</script>

<template>
  <div>
    <h1 class="page-title">Venue</h1>
    <p class="page-subtitle">Compare and select your wedding venue</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Venues</div>
        <div class="value">{{ venues.length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Shortlisted</div>
        <div class="value green">{{ venues.filter(v => v.status === 'Shortlisted').length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Visited</div>
        <div class="value">{{ venues.filter(v => v.status === 'Visited').length }}</div>
      </div>
    </div>

    <!-- Add Venue Form -->
    <div class="form-section">
      <h3>Add Venue</h3>
      <div class="form-row">
        <input v-model="newVenue.name" class="form-input" placeholder="Venue Name" />
        <input v-model="newVenue.location" class="form-input" placeholder="Location" />
      </div>
      <div class="form-row">
        <input v-model.number="newVenue.capacity" class="form-input" placeholder="Capacity" type="number" />
        <input v-model.number="newVenue.cost" class="form-input" placeholder="Cost (IDR)" type="number" />
        <button class="btn-add" @click="addVenue">+ Add Venue</button>
      </div>
    </div>

    <!-- Venue Cards -->
    <div class="card-grid">
      <div v-for="venue in venues" :key="venue.id" class="vendor-card">
        <button class="delete-btn" @click="removeVenue(venue.id)">🗑</button>
        <div class="vendor-name">{{ venue.name }}</div>
        <div class="vendor-category">{{ venue.location }}</div>
        <div class="vendor-info">👥 Capacity: {{ venue.capacity }} guests</div>
        <div class="vendor-info">💰 {{ formatRupiah(venue.cost) }}</div>
        <div class="vendor-info">📝 {{ venue.notes }}</div>
        <div style="margin-top: 12px;">
          <span class="status-badge" :class="venue.status.toLowerCase()">{{ venue.status }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.status-badge.shortlisted {
  background: #E3F2FD;
  color: #1565C0;
}

.status-badge.visited {
  background: #E8F5E9;
  color: #2E7D32;
}
</style>
