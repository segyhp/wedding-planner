<script setup lang="ts">
import { ref, computed } from 'vue'

const guests = ref([
  { id: 1, name: 'Pak Budi & Ibu Sari', email: 'budi@example.com', phone: '0812-1234-5678', rsvp: 'Attending', plusOne: true },
  { id: 2, name: 'Mbak Dina', email: 'dina@example.com', phone: '0813-2345-6789', rsvp: 'Pending', plusOne: false },
  { id: 3, name: 'Mas Andi & Keluarga', email: 'andi@example.com', phone: '0815-3456-7890', rsvp: 'Attending', plusOne: true },
  { id: 4, name: 'Tante Rina', email: 'rina@example.com', phone: '0816-4567-8901', rsvp: 'Declined', plusOne: false },
])

const searchQuery = ref('')
const newGuest = ref({ name: '', email: '', phone: '' })

const filteredGuests = computed(() => {
  if (!searchQuery.value) return guests.value
  const q = searchQuery.value.toLowerCase()
  return guests.value.filter(
    (g) => g.name.toLowerCase().includes(q) || g.email.toLowerCase().includes(q)
  )
})

const attending = computed(() => guests.value.filter((g) => g.rsvp === 'Attending').length)
const pending = computed(() => guests.value.filter((g) => g.rsvp === 'Pending').length)
const declined = computed(() => guests.value.filter((g) => g.rsvp === 'Declined').length)
const plusOnes = computed(() => guests.value.filter((g) => g.plusOne).length)

function addGuest() {
  if (!newGuest.value.name) return
  guests.value.push({
    id: Date.now(),
    name: newGuest.value.name,
    email: newGuest.value.email,
    phone: newGuest.value.phone,
    rsvp: 'Pending',
    plusOne: false,
  })
  newGuest.value = { name: '', email: '', phone: '' }
}

function removeGuest(id: number) {
  guests.value = guests.value.filter((g) => g.id !== id)
}
</script>

<template>
  <div>
    <h1 class="page-title">Guest List</h1>
    <p class="page-subtitle">Manage your wedding guest list and RSVPs</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Guests</div>
        <div class="value">{{ guests.length }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Attending</div>
        <div class="value green">{{ attending }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Pending</div>
        <div class="value orange">{{ pending }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Declined</div>
        <div class="value pink">{{ declined }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Plus Ones</div>
        <div class="value">{{ plusOnes }}</div>
      </div>
    </div>

    <!-- Add Guest Form -->
    <div class="form-section">
      <h3>Add Guest</h3>
      <div class="form-row">
        <input v-model="newGuest.name" class="form-input" placeholder="Name" @keyup.enter="addGuest" />
        <input v-model="newGuest.email" class="form-input" placeholder="Email" @keyup.enter="addGuest" />
        <input v-model="newGuest.phone" class="form-input" placeholder="Phone" @keyup.enter="addGuest" />
        <button class="btn-add" @click="addGuest">+ Add</button>
      </div>
    </div>

    <!-- Search -->
    <input v-model="searchQuery" class="search-input" placeholder="Search guests..." />

    <!-- Guest Table -->
    <table class="data-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Email</th>
          <th>Phone</th>
          <th>RSVP Status</th>
          <th>Plus One</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="guest in filteredGuests" :key="guest.id">
          <td>{{ guest.name }}</td>
          <td>{{ guest.email }}</td>
          <td>{{ guest.phone }}</td>
          <td>
            <select v-model="guest.rsvp" class="status-select" :class="guest.rsvp.toLowerCase()">
              <option>Attending</option>
              <option>Pending</option>
              <option>Declined</option>
            </select>
          </td>
          <td>
            <input type="checkbox" v-model="guest.plusOne" class="checkbox" />
          </td>
          <td>
            <button class="delete-btn" @click="removeGuest(guest.id)">🗑</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.status-select {
  padding: 4px 8px;
  border-radius: 6px;
  border: none;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
}

.status-select.attending {
  background: #E8F5E9;
  color: #2E7D32;
}

.status-select.pending {
  background: #FFF9C4;
  color: #F57F17;
}

.status-select.declined {
  background: #FFEBEE;
  color: #C62828;
}

.checkbox {
  width: 18px;
  height: 18px;
  accent-color: #E91E63;
  cursor: pointer;
}
</style>
