<template>
  <div class="catalogue-vols">
    <div class="header">
      <h1>✈️ Catalogue Compagnies</h1>
      <button class="btn btn-primary" @click="openCreateModal">+ Ajouter une compagnie</button>
    </div>

    <div v-if="loading" class="loading">Chargement...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else class="table-wrapper">
      <table class="catalogue-table">
        <thead>
          <tr>
            <th>Code IATA</th>
            <th>Nom</th>
            <th>Pays</th>
            <th>Statut</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="compagnie in compagnies" :key="compagnie.id">
            <td><strong>{{ compagnie.code_iata || '-' }}</strong></td>
            <td>{{ compagnie.nom_compagnie }}</td>
            <td>{{ compagnie.pays || '-' }}</td>
            <td>
              <span class="badge" :class="compagnie.actif ? 'badge--actif' : 'badge--inactif'">
                {{ compagnie.actif ? 'Actif' : 'Inactif' }}
              </span>
            </td>
            <td>
              <button class="btn-sm btn-secondary" @click="openEditModal(compagnie)">✏️</button>
              <button class="btn-sm btn-danger" @click="confirmDelete(compagnie.id!)">🗑</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Modal de création/modification -->
    <div v-if="showModal" class="modal-overlay" @click.self="closeModal">
      <div class="modal-content">
        <h2>{{ editingCompagnie ? 'Modifier la compagnie' : 'Ajouter une compagnie' }}</h2>
        <form @submit.prevent="saveCompagnie">
          <div class="form-group">
            <label>Code IATA</label>
            <input v-model="form.code_iata" type="text" placeholder="Ex: AH" maxlength="3" />
          </div>
          <div class="form-group">
            <label>Nom *</label>
            <input v-model="form.nom_compagnie" type="text" required />
          </div>
          <div class="form-group">
            <label>Pays</label>
            <input v-model="form.pays" type="text" />
          </div>
          <div class="form-group">
            <label>Site web</label>
            <input v-model="form.site_web" type="url" />
          </div>
          <div class="form-group">
            <label>
              <input v-model="form.actif" type="checkbox" />
              Actif
            </label>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="closeModal">Annuler</button>
            <button type="submit" class="btn btn-primary">Enregistrer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useCatalogueStore, type CompagnieCatalogue } from '@/stores/catalogueStore';

const store = useCatalogueStore();
const compagnies = ref<CompagnieCatalogue[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const showModal = ref(false);
const editingCompagnie = ref<CompagnieCatalogue | null>(null);

const form = ref({
  code_iata: '' as string | undefined,
  nom_compagnie: '' as string,
  pays: '' as string | undefined,
  site_web: '' as string | undefined,
  actif: true,
});

async function loadData() {
  loading.value = true;
  error.value = null;
  try {
    await store.loadCompagnies(false);
    compagnies.value = store.compagnies;
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

function openCreateModal() {
  editingCompagnie.value = null;
  form.value = { code_iata: undefined, nom_compagnie: '', pays: undefined, site_web: undefined, actif: true };
  showModal.value = true;
}

function openEditModal(compagnie: CompagnieCatalogue) {
  editingCompagnie.value = compagnie;
  form.value = compagnie as any;
  showModal.value = true;
}

function closeModal() {
  showModal.value = false;
  editingCompagnie.value = null;
}

async function saveCompagnie() {
  try {
    if (editingCompagnie.value) {
      await store.updateCompagnie(editingCompagnie.value.id!, form.value);
    } else {
      const data = { ...form.value, actif: true as boolean | undefined };
      await store.createCompagnie(data as any);
    }
    await loadData();
    closeModal();
  } catch (e) {
    error.value = String(e);
  }
}

async function confirmDelete(id: number) {
  if (confirm('Supprimer cette compagnie définitivement ?')) {
    try {
      await store.deleteCompagnie(id);
      await loadData();
    } catch (e) {
      error.value = String(e);
    }
  }
}

onMounted(loadData);
</script>

<style scoped>
/* Mêmes styles que CatalogueHotelsView */
.catalogue-vols { padding: 2rem; max-width: 1200px; margin: 0 auto; }
.header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
.header h1 { font-family: 'Playfair Display', serif; color: var(--color-navy); }
.table-wrapper { background: var(--color-white); border-radius: 12px; box-shadow: var(--shadow-sm); overflow-x: auto; }
.catalogue-table { width: 100%; border-collapse: collapse; font-size: 0.9rem; }
.catalogue-table th { text-align: left; padding: 0.75rem 1rem; background: var(--color-cream); color: var(--color-navy); font-weight: 600; }
.catalogue-table td { padding: 0.6rem 1rem; border-bottom: 1px solid #eee; }
.badge { display: inline-block; padding: 0.2rem 0.6rem; border-radius: 20px; font-size: 0.75rem; font-weight: 600; }
.badge--actif { background: #c8e6c9; color: #2e7d32; }
.badge--inactif { background: #ffcdd2; color: #b71c1c; }
.btn-sm { padding: 0.25rem 0.6rem; margin: 0 0.2rem; border: none; border-radius: 4px; cursor: pointer; font-size: 0.8rem; }
.btn-sm.btn-secondary { background: var(--color-gold); color: white; }
.btn-sm.btn-danger { background: var(--color-red); color: white; }
.modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal-content { background: white; padding: 2rem; border-radius: 12px; max-width: 500px; width: 100%; max-height: 90vh; overflow-y: auto; }
.modal-content h2 { margin-top: 0; color: var(--color-navy); font-family: 'Playfair Display', serif; }
.form-group { margin-bottom: 1rem; }
.form-group label { display: block; font-weight: 500; font-size: 0.9rem; margin-bottom: 0.2rem; }
.form-group input { width: 100%; padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.modal-actions { display: flex; gap: 1rem; justify-content: flex-end; margin-top: 1.5rem; }
.loading, .error { text-align: center; padding: 3rem; }
.error { color: var(--color-red); }
</style>