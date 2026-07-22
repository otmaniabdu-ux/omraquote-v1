<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Devis } from '@/types/devis.types';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  devis: Devis | null;
}>();

const emit = defineEmits(['update', 'next']);

// Saisie des taux du jour
const tauxSarDzd = ref(props.devis?.taux_sar_dzd || '1.0');
const tauxUsdDzd = ref(props.devis?.taux_usd_dzd || '1.0');
const tauxEurDzd = ref(props.devis?.taux_eur_dzd || '1.0');

// Saisie de la marge
const margeType = ref<'pourcentage' | 'montant_fixe'>(props.devis?.marge_type || 'pourcentage');
const margeValeur = ref<string>(props.devis?.marge_valeur || '15');
const remise = ref<string>(props.devis?.remise || '0');

// Saisie des notes internes
const notesInternes = ref<string | undefined>(props.devis?.notes_internes);

// Calcul indicatif (affichage uniquement — valeur canonique cote Rust)
const margeIndicative = ref('0.00');

const updateMargeIndicative = async () => {
  try {
    const res = await invoke<string>('calculer_marge_indicative_backend', {
      coutNet: '100000',
      margeType: margeType.value,
      margeValeur: margeValeur.value || '0',
    });
    margeIndicative.value = Number(res).toFixed(2);
  } catch (e) {
    console.error('Erreur marge backend:', e);
    margeIndicative.value = '0.00';
  }
};

watch([margeType, margeValeur], updateMargeIndicative, { immediate: true });

function emitUpdate() {
  emit('update', {
    taux_sar_dzd: tauxSarDzd.value,
    taux_usd_dzd: tauxUsdDzd.value,
    taux_eur_dzd: tauxEurDzd.value,
    marge_type: margeType.value,
    marge_valeur: margeValeur.value,
    remise: remise.value,
    notes_internes: notesInternes.value,
  });
}

// Emettre la mise a jour quand les valeurs changent
watch([tauxSarDzd, tauxUsdDzd, tauxEurDzd, margeType, margeValeur, remise, notesInternes], emitUpdate, { deep: true });
</script>

<template>
  <div class="formulaire-financier" dir="ltr">
    <!-- Avertissement visuel: strictement interne -->
    <div class="internal-badge">Usage interne uniquement</div>

    <h2>{{ $t('wizard.financier.title') }}</h2>

    <!-- Taux de change -->
    <div class="section taux-section">
      <h3>Taux de change du jour (-> DZD)</h3>
      <div class="taux-fields">
        <div class="field">
          <label>SAR -> DZD *</label>
          <input type="text" v-model="tauxSarDzd" placeholder="1.0000" />
        </div>
        <div class="field">
          <label>USD -> DZD *</label>
          <input type="text" v-model="tauxUsdDzd" placeholder="1.0000" />
        </div>
        <div class="field">
          <label>EUR -> DZD *</label>
          <input type="text" v-model="tauxEurDzd" placeholder="1.0000" />
        </div>
      </div>
    </div>

    <!-- Marge -->
    <div class="section marge-section">
      <h3>Marge agence</h3>
      <div class="marge-fields">
        <div class="field">
          <label>{{ $t('wizard.financier.marge_type') }}</label>
          <select v-model="margeType">
            <option value="pourcentage">{{ $t('wizard.financier.margin_types.pourcentage') }}</option>
            <option value="montant_fixe">{{ $t('wizard.financier.margin_types.montant_fixe') }}</option>
          </select>
        </div>

        <div class="field">
          <label>{{ $t('wizard.financier.marge_valeur') }}</label>
          <input type="text" v-model="margeValeur" :placeholder="margeType === 'pourcentage' ? '15' : '0'" />
          <span v-if="margeType === 'pourcentage'" class="unit">%</span>
        </div>
      </div>

      <!-- Indicatif -->
      <div class="indicatif">
        Indicatif (pour 100 000 DZD de coût net) : + {{ margeIndicative }} DZD
      </div>
    </div>

    <!-- Remise -->
    <div class="section remise-section">
      <h3>Remise</h3>
      <div class="remise-fields">
        <div class="field">
          <label>{{ $t('wizard.financier.remise') }}</label>
          <input type="text" v-model="remise" placeholder="0" />
        </div>
      </div>
    </div>

    <!-- Notes internes -->
    <div class="section notes-section">
      <h3>{{ $t('wizard.financier.notes_internes') }}</h3>
      <textarea v-model="notesInternes" rows="3" placeholder="Notes internes (visible uniquement par l'agence)"></textarea>
    </div>

    <div class="step-actions">
      <button class="btn btn-primary" @click="$emit('next')">
        {{ $t('devis.next') }} &rarr;
      </button>
    </div>
  </div>
</template>

<style scoped>
.formulaire-financier { max-width: 700px; margin: 0 auto; }
.internal-badge { display: inline-block; background: var(--color-red); color: white; padding: 0.3rem 0.8rem; border-radius: 4px; font-size: 0.8rem; font-weight: 600; margin-bottom: 1rem; }
h2 { font-family: 'Playfair Display', serif; color: var(--color-navy); }
.section { background: white; padding: 1rem; border-radius: 8px; box-shadow: var(--shadow-sm); margin-bottom: 1rem; }
.taux-section { border-top: 3px solid var(--color-gold); }
.marge-section { border-top: 3px solid var(--color-blue-royal); }
.remise-section { border-top: 3px solid #2e7d32; }
.notes-section textarea { width: 100%; padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; resize: vertical; }
.section h3 { margin-top: 0; color: var(--color-navy); font-size: 1rem; }
.taux-fields, .marge-fields, .remise-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 0.75rem; }
.field { display: flex; flex-direction: column; gap: 0.25rem; position: relative; }
.field label { font-size: 0.8rem; font-weight: 500; color: #555; }
.field input, .field select { padding: 0.4rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.unit { position: absolute; right: 8px; top: 32px; color: var(--color-gray-500); font-size: 0.85rem; }
.indicatif { margin-top: 1rem; padding: 0.75rem; background: var(--color-cream); border-radius: 4px; font-size: 0.85rem; color: #555; }
.step-actions { display: flex; justify-content: flex-end; margin-top: 1.5rem; }
</style>
