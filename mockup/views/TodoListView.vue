<script setup lang="ts">
import { ref, computed } from 'vue'

const todos = ref([
  { id: 1, task: 'Book photographer', category: 'Vendors', dueDate: '2026-03-01', completed: true },
  { id: 2, task: 'Send invitations', category: 'Guests', dueDate: '2026-04-15', completed: false },
  { id: 3, task: 'Order wedding cake', category: 'Catering', dueDate: '2026-05-01', completed: false },
  { id: 4, task: 'Finalize venue contract', category: 'Venue', dueDate: '2026-03-15', completed: false },
  { id: 5, task: 'Book honeymoon', category: 'Personal', dueDate: '2026-06-01', completed: false },
])

const activeFilter = ref<'all' | 'active' | 'completed'>('all')
const newTodo = ref({ task: '', category: '', dueDate: '' })

const totalTasks = computed(() => todos.value.length)
const completedCount = computed(() => todos.value.filter((t) => t.completed).length)
const remainingCount = computed(() => todos.value.filter((t) => !t.completed).length)

const filteredTodos = computed(() => {
  if (activeFilter.value === 'active') return todos.value.filter((t) => !t.completed)
  if (activeFilter.value === 'completed') return todos.value.filter((t) => t.completed)
  return todos.value
})

function addTodo() {
  if (!newTodo.value.task) return
  todos.value.push({
    id: Date.now(),
    task: newTodo.value.task,
    category: newTodo.value.category,
    dueDate: newTodo.value.dueDate,
    completed: false,
  })
  newTodo.value = { task: '', category: '', dueDate: '' }
}

function removeTodo(id: number) {
  todos.value = todos.value.filter((t) => t.id !== id)
}
</script>

<template>
  <div>
    <h1 class="page-title">To-Do List</h1>
    <p class="page-subtitle">Track your wedding planning tasks</p>

    <!-- Stat Cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="label">Total Tasks</div>
        <div class="value">{{ totalTasks }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Completed</div>
        <div class="value green">{{ completedCount }}</div>
      </div>
      <div class="stat-card">
        <div class="label">Remaining</div>
        <div class="value orange">{{ remainingCount }}</div>
      </div>
    </div>

    <!-- Add Task Form -->
    <div class="form-section">
      <h3>Add Task</h3>
      <div class="form-row">
        <input v-model="newTodo.task" class="form-input" placeholder="Task" @keyup.enter="addTodo" />
        <input v-model="newTodo.category" class="form-input" placeholder="Category" />
        <input v-model="newTodo.dueDate" class="form-input" type="date" />
      </div>
      <button class="btn-add" @click="addTodo">+ Add Task</button>
    </div>

    <!-- Filter Tabs -->
    <div class="filter-tabs">
      <button
        class="filter-tab"
        :class="{ active: activeFilter === 'all' }"
        @click="activeFilter = 'all'"
      >All</button>
      <button
        class="filter-tab"
        :class="{ active: activeFilter === 'active' }"
        @click="activeFilter = 'active'"
      >Active</button>
      <button
        class="filter-tab"
        :class="{ active: activeFilter === 'completed' }"
        @click="activeFilter = 'completed'"
      >Completed</button>
    </div>

    <!-- Todo Items -->
    <div class="todo-list">
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        class="todo-item"
        :class="{ completed: todo.completed }"
      >
        <div class="todo-left">
          <button
            class="todo-check"
            :class="{ checked: todo.completed }"
            @click="todo.completed = !todo.completed"
          >
            <span v-if="todo.completed">✓</span>
          </button>
          <div>
            <div class="todo-task">{{ todo.task }}</div>
            <div class="todo-meta">{{ todo.category }} &nbsp; Due: {{ todo.dueDate }}</div>
          </div>
        </div>
        <button class="delete-btn" @click="removeTodo(todo.id)">🗑</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.todo-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  background: #f0f0f0;
  border-radius: 12px;
  overflow: hidden;
}

.todo-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: #fff;
}

.todo-item.completed .todo-task {
  text-decoration: line-through;
  color: #999;
}

.todo-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.todo-check {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  border: 2px solid #e0e0e0;
  background: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  font-size: 14px;
  color: #fff;
  transition: all 0.2s;
}

.todo-check.checked {
  background: #4CAF50;
  border-color: #4CAF50;
}

.todo-task {
  font-size: 14px;
  font-weight: 500;
}

.todo-meta {
  font-size: 12px;
  color: #999;
  margin-top: 2px;
}
</style>
