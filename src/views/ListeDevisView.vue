<template>
  <div class="liste-devis">
    <div class="header">
      <h1>Liste des devis</h1>
      <button class="btn btn-primary" @click="router.push('/devis/nouveau')">
        + Nouveau devis
      </button>
    </div>

    <div v-if="loading" class="loading">Chargement...</div>
    <div v-else-if="devisStore.devisList.length === 0" class="empty">
      Aucun devis pour l'instant. Créez le premier !
    </div>
    <div v-else class="table-wrapper">
      <table class="devis-table">
        <thead>
          <tr>
            <th>N° Devis</th>
            <th>Client</th>
            <th>Départ</th>
            <th>Retour</th>
            <th>Statut</th>
            <th>Prix vente</th>
            <th>Alerte</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="devis in devisStore.devisList" :key="devis.id">
            <td><strong>{{ devis.numero_devis }}</strong></td>
            <td>{{ getClientName(devis.client_id) }}</td>
            <td>{{ formatDate(devis.date_depart) }}</td>
            <td>{{ formatDate(devis.date_retour) }}</td>
            <td><span class="badge" :class="`badge--${devis.statut}`">{{ devis.statut }}</span></td>
            <td>{{ formatPrice(devis.prix_vente_total) }}</td>
            <td>
              <span v-if="devisStore.getAlerteForDevis(devis.id!)" class="alert-badge">⚠️</span>
              <span v-else>✓</span>
            </td>
            <td>
              <button class="btn-sm btn-primary" @click="viewDevis(devis.id!)">Voir</button>
              <button class="btn-sm btn-secondary" @click="editDevis(devis.id!)">Éditer</button>
              <button class="btn-sm btn-danger" @click="confirmDelete(devis.id!)">🗑</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useDevisStore } from '@/stores/devisStore';
import { useClientStore } from '@/stores/clientStore';
import { useRouter } from 'vue-router';

const router = useRouter();

const devisStore = useDevisStore();
const clientStore = useClientStore();
const loading = ref(true);

const getClientName = (id: number) => {
  const client = clientStore.clients.find(c => c.id === id);
  return client?.nom_contact || client?.raison_sociale || `Client #${id}`;
};

const formatDate = (date: string) => {
  if (!date) return '';
  const d = new Date(date);
  return d.toLocaleDateString('fr-FR');
};

const formatPrice = (price?: string) => {
  if (!price) return '—';
  const val = parseFloat(price);
  return val.toFixed(2) + ' DZD';
};

const viewDevis = (id: number) => {
  router.push(`/devis/${id}`);
};

const editDevis = (id: number) => {
  router.push(`/devis/nouveau?edit=${id}`);
};

const confirmDelete = async (id: number) => {
  if (confirm('Supprimer ce devis définitivement ?')) {
    await devisStore.deleteDevis(id);
    await devisStore.loadDevis();
  }
};

onMounted(async () => {
  await Promise.all([devisStore.loadDevis(), clientStore.loadClients()]);
  loading.value = false;
});
</script>

<style scoped>
.liste-devis { padding: 2rem; max-width: 1400px; margin: 0 auto; }
.header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
.header h1 { font-family: 'Playfair Display', serif; color: var(--color-navy); }
.table-wrapper { overflow-x: auto; background: var(--color-white); border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.06); }
.devis-table { width: 100%; border-collapse: collapse; font-size: 0.9rem; }
.devis-table th { text-align: left; padding: 1rem; background: var(--color-cream); color: var(--color-navy); font-weight: 600; }
.devis-table td { padding: 0.75rem 1rem; border-bottom: 1px solid #eee; }
.devis-table tr:hover { background: #fafafa; }
.badge { display: inline-block; padding: 0.2rem 0.6rem; border-radius: 20px; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; }
.badge--brouillon { background: #e0e0e0; color: #666; }
.badge--finalise { background: #c8e6c9; color: #2e7d32; }
.badge--envoye { background: #bbdefb; color: #0d47a1; }
.badge--accepte { background: #a5d6a7; color: #1b5e20; }
.badge--perdu { background: #ffcdd2; color: #b71c1c; }
.alert-badge { font-size: 1.2rem; }
.btn-sm { padding: 0.25rem 0.6rem; margin: 0 0.2rem; border: none; border-radius: 4px; cursor: pointer; font-size: 0.8rem; }
.btn-sm.btn-primary { background: var(--color-navy); color: white; }
.btn-sm.btn-secondary { background: var(--color-gold); color: white; }
.btn-sm.btn-danger { background: var(--color-red); color: white; }
.loading, .empty { text-align: center; padding: 3rem; color: #666; }
</style>