import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '../views/DashboardView.vue'
import MargeDashboardView from '../views/MargeDashboardView.vue';
import CatalogueHotelsView from '../views/CatalogueHotelsView.vue';
import CatalogueVolsView from '../views/CatalogueVolsView.vue';

const routes = [
  { path: '/', component: DashboardView },
  { path: '/devis/nouveau', component: () => import('../views/NouveauDevisView.vue') },
  { path: '/devis/liste', component: () => import('../views/ListeDevisView.vue') },
  { path: '/clients', component: () => import(/* webpackChunkName: "clients" */ '../views/ClientsView.vue') },
  { path: '/catalogue/hotels', component: CatalogueHotelsView },
  { path: '/catalogue/vols', component: CatalogueVolsView },
  { path: '/parametres', component: () => import(/* webpackChunkName: "parametres" */ '../views/ParametresView.vue') },
  { path: '/marge', component: MargeDashboardView },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
