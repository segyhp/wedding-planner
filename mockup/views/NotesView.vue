<script setup lang="ts">
import { ref } from 'vue'

const notes = ref([
  {
    id: 1,
    title: 'Meeting with Dalang WO',
    content: 'Discuss package details for Paket Platinum. Need to confirm decoration style and catering menu. Ask about entertainment options.',
    date: '2026-02-05',
    category: 'Vendor',
  },
  {
    id: 2,
    title: 'Guest list finalization',
    content: 'Need to confirm from bride side: Tante Rina family (3 people), Om Hadi family (5 people). Deadline: March 1.',
    date: '2026-02-03',
    category: 'Guests',
  },
  {
    id: 3,
    title: 'Decoration ideas',
    content: 'Azizah likes: soft pink & gold theme, garden-style flower arrangements, fairy lights. Check Pinterest board for references.',
    date: '2026-01-28',
    category: 'Decoration',
  },
])

const newNote = ref({ title: '', content: '', category: '' })

function addNote() {
  if (!newNote.value.title) return
  notes.value.unshift({
    id: Date.now(),
    title: newNote.value.title,
    content: newNote.value.content,
    date: new Date().toISOString().split('T')[0],
    category: newNote.value.category,
  })
  newNote.value = { title: '', content: '', category: '' }
}

function removeNote(id: number) {
  notes.value = notes.value.filter((n) => n.id !== id)
}
</script>

<template>
  <div>
    <h1 class="page-title">Notes</h1>
    <p class="page-subtitle">Keep track of important wedding planning notes</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Notes</div>
        <div class="value">{{ notes.length }}</div>
      </div>
    </div>

    <!-- Add Note Form -->
    <div class="form-section">
      <h3>Add Note</h3>
      <div class="form-row">
        <input v-model="newNote.title" class="form-input" placeholder="Title" />
        <input v-model="newNote.category" class="form-input" placeholder="Category" />
      </div>
      <div class="form-row">
        <textarea
          v-model="newNote.content"
          class="form-input note-textarea"
          placeholder="Write your note here..."
          rows="3"
        ></textarea>
      </div>
      <button class="btn-add" @click="addNote">+ Add Note</button>
    </div>

    <!-- Notes List -->
    <div class="notes-grid">
      <div v-for="note in notes" :key="note.id" class="note-card">
        <button class="delete-btn" @click="removeNote(note.id)">🗑</button>
        <div class="note-category-badge">{{ note.category }}</div>
        <div class="note-title">{{ note.title }}</div>
        <div class="note-content">{{ note.content }}</div>
        <div class="note-date">{{ note.date }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.note-textarea {
  resize: vertical;
  min-height: 80px;
  font-family: inherit;
}

.notes-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.note-card {
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
  position: relative;
}

.note-card .delete-btn {
  position: absolute;
  top: 16px;
  right: 16px;
}

.note-category-badge {
  display: inline-block;
  padding: 2px 10px;
  background: #FFF0F3;
  color: #E91E63;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
  margin-bottom: 8px;
}

.note-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 8px;
}

.note-content {
  font-size: 13px;
  color: #666;
  line-height: 1.5;
  margin-bottom: 12px;
}

.note-date {
  font-size: 12px;
  color: #999;
}
</style>
