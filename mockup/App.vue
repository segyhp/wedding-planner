<script setup lang="ts">
import SidebarNav from './components/SidebarNav.vue'
</script>

<template>
  <div class="app-layout">
    <SidebarNav />
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style>
/* ========================================
   Global Design System
   Based on Figma Mockup Analysis
   Colors: Pink #E91E63, Green #4CAF50, White #FFF
   ======================================== */

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
  color: #333;
}

.app-layout {
  display: flex;
  min-height: 100vh;
}

.main-content {
  flex: 1;
  padding: 32px 40px;
  margin-left: 240px; /* sidebar width */
  background: #fff;
}

/* Shared component styles */
.page-title {
  font-size: 28px;
  font-weight: 700;
  color: #1a1a1a;
  margin-bottom: 4px;
}

.page-subtitle {
  font-size: 14px;
  color: #888;
  margin-bottom: 24px;
}

.stat-cards {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  flex: 1;
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
}

.stat-card .label {
  font-size: 13px;
  color: #888;
  margin-bottom: 4px;
}

.stat-card .value {
  font-size: 28px;
  font-weight: 700;
  color: #1a1a1a;
}

.stat-card .value.green {
  color: #4CAF50;
}

.stat-card .value.pink {
  color: #E91E63;
}

.stat-card .value.orange {
  color: #FF9800;
}

.form-section {
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
}

.form-section h3 {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
}

.form-row {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.form-input {
  flex: 1;
  padding: 10px 14px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.form-input:focus {
  border-color: #E91E63;
}

.form-select {
  flex: 1;
  padding: 10px 14px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
  background: #fff;
}

.btn-add {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 10px 20px;
  background: #E91E63;
  color: #fff;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-add:hover {
  background: #C2185B;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
}

.data-table th {
  text-align: left;
  padding: 12px 16px;
  font-size: 13px;
  font-weight: 600;
  color: #555;
  border-bottom: 1px solid #e0e0e0;
}

.data-table td {
  padding: 12px 16px;
  font-size: 14px;
  border-bottom: 1px solid #f0f0f0;
  vertical-align: middle;
}

.status-badge {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
}

.status-badge.booked,
.status-badge.attending {
  background: #E8F5E9;
  color: #2E7D32;
}

.status-badge.pending {
  background: #FFF9C4;
  color: #F57F17;
}

.status-badge.paid {
  background: #E8F5E9;
  color: #2E7D32;
}

.status-badge.declined {
  background: #FFEBEE;
  color: #C62828;
}

.delete-btn {
  background: none;
  border: none;
  color: #E91E63;
  cursor: pointer;
  font-size: 18px;
  padding: 4px;
}

.delete-btn:hover {
  color: #C2185B;
}

.search-input {
  width: 100%;
  padding: 10px 14px 10px 40px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
  margin-bottom: 16px;
  background: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' fill='%23999' viewBox='0 0 16 16'%3E%3Cpath d='M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z'/%3E%3C/svg%3E") no-repeat 14px center;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.vendor-card {
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
  position: relative;
}

.vendor-card .vendor-name {
  font-size: 16px;
  font-weight: 600;
}

.vendor-card .vendor-category {
  font-size: 13px;
  color: #888;
  margin-bottom: 12px;
}

.vendor-card .vendor-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #555;
  margin-bottom: 6px;
}

.vendor-card .delete-btn {
  position: absolute;
  top: 16px;
  right: 16px;
}

.filter-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.filter-tab {
  padding: 6px 16px;
  border-radius: 20px;
  border: 1px solid #e0e0e0;
  background: #fff;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.filter-tab.active {
  background: #E91E63;
  color: #fff;
  border-color: #E91E63;
}

.category-card {
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
}

.category-card .category-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
}

.category-card .category-total {
  color: #1a1a1a;
}

.category-card .menu-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-top: 1px solid #f0f0f0;
}

.category-card .menu-item .item-name {
  font-size: 14px;
  font-weight: 500;
}

.category-card .menu-item .item-detail {
  font-size: 13px;
  color: #888;
}
</style>
