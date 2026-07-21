<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useClientStore } from '@/stores/clientStore';

const store = useClientStore();
const loading = ref(false);
const error = ref<string | null>(null);
const showModal = ref(false);
const form = ref({
  nom_contact: '',
  telephone: '',
  email: '',
  adresse: '',
  pays: '',
  type_client: 'particulier' as 'particulier' | 'agence',
  remarques: '',
});

async function loadData() {
  loading.value = true;
  error.value = null;
  try {
    await store.loadClients();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

async function saveClient() {
  try {
    if (form.value.nom_contact.trim()) {
      await store.createClient(form.value as any);
      showModal.value = false;
      form.value = { nom_contact: '', telephone: '', email: '', adresse: '', pays: '', type_client: 'particulier', remarques: '' };
      await loadData();
    }
  } catch (e) {
    error.value = String(e);
  }
}

onMounted(loadData);
</script>

<template>
  <div class="clients-view">
    <div class="header">
      <h1>Clients</h1>
      <button class="btn btn-primary" @click="showModal = true">+ Nouveau client</button>
    </div>

    <div v-if="loading" class="loading">Chargement...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else class="table-wrapper">
      <table class="clients-table">
        <thead>
          <tr><th>Code</th><th>Contact</th><th>Tel</th><th>Email</th><th>Type</th><th>Actions</th></tr>
        </thead>
        <tbody>
          <tr v-for="c in store.clients" :key="c.id">
            <td>{{ c.code_client }}</td>
            <td>{{ c.nom_contact || c.raison_sociale || '—' }}</td>
            <td>{{ c.telephone || '—' }}</td>
            <td>{{ c.email || '—' }}</td>
            <td>{{ c.type_client }}</td>
            <td>
              <button class="btn-sm">Voir</button>
              <button class="btn-sm btn-danger" @click="store.deleteClient(c.id!)">Suppr.</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Modal creation client -->
    <div v-if="showModal" class="modal-overlay" @click.self="showModal = false">
      <div class="modal-content">
        <h2>Nouveau client</h2>
        <form @submit.prevent="saveClient">
          <div class="form-group">
            <label>Contact *</label>
            <input v-model="form.nom_contact" type="text" required />
          </div>
          <div class="form-group"><label>Tel</label><input v-model="form.telephone" type="text" /></div>
          <div class="form-group"><label>Email</label><input v-model="form.email" type="email" /></div>
          <div class="form-group">
            <label>Type</label>
            <select v-model="form.type_client">
              <option value="particulier">Particulier</option>
              <option value="agence">Agence</option>
            </select>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showModal = false">Annuler</button>
            <button type="submit" class="btn btn-primary">Enregistrer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.clients-view { padding: 2rem; max-width: 1200px; margin: 0 auto; }
.header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
.header h1 { font-family: 'Playfair Display', serif; color: var(--color-navy); }
.table-wrapper { background: var(--color-white); border-radius: 12px; box-shadow: var(--shadow-sm); overflow-x: auto; }
.clients-table { width: 100%; border-collapse: collapse; font-size: 0.9rem; }
.clients-table th { text-align: left; padding: 0.75rem 1rem; background: var(--color-cream); color: var(--color-navy); font-weight: 600; }
.clients-table td { padding: 0.5rem 1rem; border-bottom: 1px solid #eee; }
.btn-sm { padding: 0.25rem 0.6rem; margin: 0 0.2rem; border: none; border-radius: 4px; cursor: pointer; font-size: 0.8rem; background: var(--color-gold); color: white; }
.btn-sm.btn-danger { background: var(--color-red); }
.modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal-content { background: white; padding: 2rem; border-radius: 12px; max-width: 500px; width: 90%; }
.form-group { margin-bottom: 1rem; }
.form-group label { display: block; font-weight: 500; font-size: 0.9rem; margin-bottom: 0.2rem; }
.form-group input, .form-group select { width: 100%; padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.modal-actions { display: flex; gap: 1rem; justify-content: flex-end; margin-top: 1.5rem; }
.loading, .error { text-align: center; padding: 3rem; }
.error { color: var(--color-red); }
</style>
