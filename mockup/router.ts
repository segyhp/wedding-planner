import { createRouter, createWebHistory } from 'vue-router'
import BudgetView from './views/BudgetView.vue'
import VenueView from './views/VenueView.vue'
import RentalsView from './views/RentalsView.vue'
import GuestListView from './views/GuestListView.vue'
import SeatingPlanView from './views/SeatingPlanView.vue'
import TodoListView from './views/TodoListView.vue'
import VendorsView from './views/VendorsView.vue'
import FoodBeveragesView from './views/FoodBeveragesView.vue'
import NotesView from './views/NotesView.vue'

const routes = [
  { path: '/', redirect: '/budget' },
  { path: '/budget', name: 'Budget', component: BudgetView },
  { path: '/venue', name: 'Venue', component: VenueView },
  { path: '/rentals', name: 'Rentals', component: RentalsView },
  { path: '/guest-list', name: 'GuestList', component: GuestListView },
  { path: '/seating-plan', name: 'SeatingPlan', component: SeatingPlanView },
  { path: '/todo-list', name: 'TodoList', component: TodoListView },
  { path: '/vendors', name: 'Vendors', component: VendorsView },
  { path: '/food-beverages', name: 'FoodBeverages', component: FoodBeveragesView },
  { path: '/notes', name: 'Notes', component: NotesView },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
