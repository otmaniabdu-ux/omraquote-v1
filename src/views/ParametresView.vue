<script setup lang="ts">
import { ref } from 'vue';

const form = ref({
  nom_agence: 'El Mouhssinouen Tours',
  adresse: '',
  telephone: '',
  email: '',
  devise_defaut: 'DZD',
  marge_defaut_type: 'pourcentage' as 'pourcentage' | 'montant_fixe',
  marge_defaut_valeur: '15',
});

function save() {
  // TODO: appel Tauri pour persister les parametres
  alert('Parametres sauvegardes');
}
</script>

<template>
  <div class="parametres-view">
    <h1>Parametres</h1>
    <form @submit.prevent="save" class="parametres-form">
      <div class="form-group">
        <label>Nom agence</label>
        <input v-model="form.nom_agence" type="text" />
      </div>
      <div class="form-group"><label>Adresse</label><input v-model="form.adresse" type="text" /></div>
      <div class="form-group"><label>Tel</label><input v-model="form.telephone" type="text" /></div>
      <div class="form-group"><label>Email</label><input v-model="form.email" type="email" /></div>
      <div class="form-group">
        <label>Devise par defaut</label>
        <select v-model="form.devise_defaut">
          <option value="DZD">DZD</option>
          <option value="SAR">SAR</option>
          <option value="USD">USD</option>
          <option value="EUR">EUR</option>
        </select>
      </div>
      <div class="form-group">
        <label>Type de marge par defaut</label>
        <select v-model="form.marge_defaut_type">
          <option value="pourcentage">Pourcentage (%)</option>
          <option value="montant_fixe">Montant fixe</option>
        </select>
      </div>
      <div class="form-group"><label>Valeur marge par defaut</label><input v-model="form.marge_defaut_valeur" type="text" /></div>
      <button class="btn btn-primary" type="submit">Enregistrer</button>
    </form>
  </div>
</template>

<style scoped>
.parametres-view { padding: 2rem; max-width: 700px; margin: 0 auto; }
.parametres-view h1 { font-family: 'Playfair Display', serif; color: var(--color-navy); }
.parametres-form { background: white; padding: 2rem; border-radius: 12px; box-shadow: var(--shadow-sm); margin-top: 1.5rem; }
.form-group { margin-bottom: 1rem; }
.form-group label { display: block; font-weight: 500; font-size: 0.9rem; margin-bottom: 0.2rem; }
.form-group input, .form-group select { width: 100%; padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
</style>
