<script setup lang="ts">
import { ref, computed } from 'vue'

const tables = ref([
  { id: 1, name: 'Table 1 - Family Groom', capacity: 10, guests: ['Pak Budi & Ibu Sari', 'Mas Andi & Keluarga', 'Om Hadi', 'Tante Lina'] },
  { id: 2, name: 'Table 2 - Family Bride', capacity: 10, guests: ['Pak Ahmad', 'Ibu Dewi', 'Mbak Fia', 'Mas Reza'] },
  { id: 3, name: 'Table 3 - Friends', capacity: 10, guests: ['Dina', 'Rangga', 'Sinta'] },
  { id: 4, name: 'Table 4 - Colleagues', capacity: 10, guests: [] },
])

const newTable = ref({ name: '', capacity: 10 as number })

const totalSeats = computed(() => tables.value.reduce((sum, t) => sum + t.capacity, 0))
const totalSeated = computed(() => tables.value.reduce((sum, t) => sum + t.guests.length, 0))
const availableSeats = computed(() => totalSeats.value - totalSeated.value)

function addTable() {
  if (!newTable.value.name) return
  tables.value.push({
    id: Date.now(),
    name: newTable.value.name,
    capacity: newTable.value.capacity,
    guests: [],
  })
  newTable.value = { name: '', capacity: 10 }
}

function removeTable(id: number) {
  tables.value = tables.value.filter((t) => t.id !== id)
}
</script>

<template>
  <div>
    <h1 class="page-title">Seating Plan</h1>
    <p class="page-subtitle">Organize table assignments for your guests</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Tables</div>
        <div class="value">{{ tables.length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Total Seats</div>
        <div class="value">{{ totalSeats }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Seated</div>
        <div class="value green">{{ totalSeated }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Available</div>
        <div class="value orange">{{ availableSeats }}</div>
      </div>
    </div>

    <!-- Add Table Form -->
    <div class="form-section">
      <h3>Add Table</h3>
      <div class="form-row">
        <input v-model="newTable.name" class="form-input" placeholder="Table Name" @keyup.enter="addTable" />
        <input v-model.number="newTable.capacity" class="form-input" placeholder="Capacity" type="number" @keyup.enter="addTable" />
        <button class="btn-add" @click="addTable">+ Add Table</button>
      </div>
    </div>

    <!-- Table Cards Grid -->
    <div class="card-grid">
      <div v-for="table in tables" :key="table.id" class="vendor-card">
        <button class="delete-btn" @click="removeTable(table.id)">🗑</button>
        <div class="vendor-name">{{ table.name }}</div>
        <div class="vendor-category">{{ table.guests.length }} / {{ table.capacity }} seats filled</div>

        <div class="seat-progress">
          <div class="seat-bar">
            <div
              class="seat-fill"
              :style="{ width: `${(table.guests.length / table.capacity) * 100}%` }"
            ></div>
          </div>
        </div>

        <div class="guest-list">
          <div v-for="guest in table.guests" :key="guest" class="guest-chip">
            {{ guest }}
          </div>
          <div v-if="table.guests.length === 0" class="empty-text">No guests assigned yet</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.seat-progress {
  margin: 12px 0;
}

.seat-bar {
  width: 100%;
  height: 6px;
  background: #f0f0f0;
  border-radius: 3px;
  overflow: hidden;
}

.seat-fill {
  height: 100%;
  background: #E91E63;
  border-radius: 3px;
  transition: width 0.3s;
}

.guest-list {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
}

.guest-chip {
  padding: 4px 10px;
  background: #FFF0F3;
  color: #E91E63;
  border-radius: 12px;
  font-size: 12px;
}

.empty-text {
  font-size: 13px;
  color: #999;
  font-style: italic;
}
</style>
