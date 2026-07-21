<template>
  <div class="dashboard">
    <h1 class="dashboard__title">Tableau de bord</h1>
    <div class="dashboard__stats">
      <div class="stat-card">
        <span class="stat-card__label">Devis</span>
        <span class="stat-card__value">{{ devisStore.devisList.length }}</span>
      </div>
      <div class="stat-card">
        <span class="stat-card__label">Clients</span>
        <span class="stat-card__value">{{ clientStore.clients.length }}</span>
      </div>
      <div class="stat-card stat-card--alert" v-if="alertCount > 0">
        <span class="stat-card__label">Alertes passeport</span>
        <span class="stat-card__value">{{ alertCount }}</span>
      </div>
    </div>

    <div class="dashboard__actions">
      <button class="btn btn-primary" @click="router.push('/devis/nouveau')">
        Nouveau devis
      </button>
      <button class="btn btn-secondary" @click="router.push('/devis/liste')">
        Voir tous les devis
      </button>
    </div>

    <div class="dashboard__quick-actions">
      <h3>Accès rapide</h3>
      <div class="quick-grid">
        <router-link to="/clients" class="quick-link">Clients</router-link>
        <router-link to="/catalogue/hotels" class="quick-link">Hôtels</router-link>
        <router-link to="/catalogue/vols" class="quick-link">Compagnies</router-link>
        <router-link to="/parametres" class="quick-link">Paramètres</router-link>
        <router-link to="/marge" class="quick-link quick-link--internal">Dashboard marge</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useDevisStore } from '@/stores/devisStore';
import { useClientStore } from '@/stores/clientStore';
import { computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const devisStore = useDevisStore();
const clientStore = useClientStore();

const alertCount = computed(() => {
  return devisStore.alertes.filter(a => a.alerte).length;
});

onMounted(async () => {
  await devisStore.loadDevis();
  await clientStore.loadClients();
});
</script>

<style scoped>
.dashboard {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}
.dashboard__title {
  font-family: 'Playfair Display', serif;
  color: var(--color-navy);
  font-size: 2rem;
  margin-bottom: 2rem;
}
.dashboard__stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}
.stat-card {
  background: var(--color-white);
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.06);
  border-left: 4px solid var(--color-gold);
}
.stat-card--alert {
  border-left-color: var(--color-red);
}
.stat-card__label {
  display: block;
  font-size: 0.875rem;
  color: #666;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.stat-card__value {
  display: block;
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-navy);
  margin-top: 0.25rem;
}
.dashboard__actions {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}
.quick-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin-top: 0.75rem;
}
.quick-link {
  display: block;
  padding: 0.75rem 1rem;
  background: var(--color-white);
  border-radius: 8px;
  text-align: center;
  text-decoration: none;
  color: var(--color-navy);
  font-weight: 500;
  border: 1px solid #e0e0e0;
  transition: all 0.2s;
}
.quick-link:hover {
  border-color: var(--color-gold);
  background: var(--color-cream);
}
.quick-link--internal {
  border-color: var(--color-red);
  color: var(--color-red);
}
.quick-link--internal:hover {
  background: #fff0f0;
}
</style>