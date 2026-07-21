<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { PrestationVip } from '@/types/devis.types';

const { t, locale } = useI18n();

const props = defineProps<{
  devis: any;
  prestations: PrestationVip[];
}>();

const emit = defineEmits(['update', 'next']);

// Types de prestations pre-remplies
const PRESTATION_TYPES = [
  { type: 'ziyarat' as const, label: t('wizard.prestations_vip.types.ziyarat') },
  { type: 'lounge' as const, label: t('wizard.prestations_vip.types.lounge') },
  { type: 'fast_track' as const, label: t('wizard.prestations_vip.types.fast_track') },
  { type: 'bagages' as const, label: t('wizard.prestations_vip.types.bagages') },
  { type: 'zamzam' as const, label: t('wizard.prestations_vip.types.zamzam') },
] as const;

const DEVISES = ['SAR', 'USD', 'EUR', 'DZD'];

// Lignes pre-remplies (option guide/vehicule pour ziyarat)
const localPrestations = ref<PrestationVip[]>([]);
const showOther = ref(false);

function initPreRemplies() {
  if (localPrestations.value.length === 0) {
    localPrestations.value = PRESTATION_TYPES.map(pt => ({
      id: undefined,
      devis_id: props.devis?.id || 0,
      type_prestation: pt.type as PrestationVip['type_prestation'],
      description: pt.label,
      prix_unitaire: '0',
      quantite: 1,
      devise_prix: props.devis?.devise_achat || 'SAR',
      remarques: '',
    }));
  }
}

function addAutre() {
  showOther.value = true;
  if (localPrestations.value.length === 0) {
    initPreRemplies(); // force initialisation si necessaire
  }
}

function removePrestation(idx: number) {
  localPrestations.value.splice(idx, 1);
}

// Valeurs pour la prestation "autre" (personnalisee)
const otherDescription = ref('');
const otherQuantite = ref(1);
const otherPrice = ref('0');
const otherDevise = ref('SAR');
const otherRemarques = ref('');
const optionGuide = ref(false);
const optionVehicule = ref(false);

// Emet la mise a jour quand les prestations changent
watch(
  () => localPrestations.value.map(p => ({ ...p })),
  () => emit('update', { prestations: [...localPrestations.value] }),
  { deep: true }
);
</script>

<template>
  <div class="formulaire-prestations-vip" :dir="locale === 'ar' ? 'rtl' : 'ltr'">
    <h2>{{ $t('wizard.prestations_vip.title') }}</h2>

    <!-- Lignes pre-remplies -->
    <div v-for="(p, idx) in localPrestations" :key="'p-' + idx" class="prestation-item">
      <div class="prestation-header">
        <span class="prestation-type">{{ p.description }}</span>
        <button class="btn-sm btn-remove" @click="removePrestation(idx)">&#10005;</button>
      </div>

      <div class="prestation-fields">
        <!-- Type -->
        <div class="field">
          <label>{{ $t('wizard.prestations_vip.type') }}</label>
          <select v-model="p.type_prestation" @change="p.description = PRESTATION_TYPES.find(pt => pt.type === p.type_prestation)?.label || ''">
            <option v-for="pt in PRESTATION_TYPES" :key="pt.type" :value="pt.type">{{ pt.label }}</option>
            <option value="autre">{{ $t('wizard.prestations_vip.types.autre') }}</option>
          </select>
        </div>

        <!-- Description (modifiable) -->
        <div class="field field--full">
          <label>{{ $t('wizard.prestations_vip.description') }}</label>
          <input type="text" v-model="p.description" :placeholder="$t('wizard.prestations_vip.description')" />
        </div>

        <!-- Option guide/vehicule pour ziyarat -->
        <div v-if="p.type_prestation === 'ziyarat'" class="field--full ziyarat-options">
          <span>{{ $t('wizard.vols.class') }}</span>
          <div class="checkbox-group">
            <label><input type="checkbox" v-model="optionGuide" /> Guide</label>
            <label><input type="checkbox" v-model="optionVehicule" /> Vehicule</label>
          </div>
        </div>

        <!-- Quantite -->
        <div class="field">
          <label>{{ $t('wizard.prestations_vip.quantity') }}</label>
          <input type="number" min="1" v-model.number="p.quantite" />
        </div>

        <!-- Prix unitaire (chaine decimale — pas de number!) -->
        <div class="field">
          <label>{{ $t('wizard.prestations_vip.unit_price') }}</label>
          <input type="text" v-model="p.prix_unitaire" placeholder="0" />
        </div>

        <!-- Devise -->
        <div class="field">
          <label>{{ $t('wizard.vols.currency') }}</label>
          <select v-model="p.devise_prix">
            <option v-for="d in DEVISES" :key="d" :value="d">{{ d }}</option>
          </select>
        </div>

        <!-- Remarques -->
        <div class="field field--full">
          <label>{{ $t('devis.notes_internes') || 'Remarques' }}</label>
          <input type="text" v-model="p.remarques" />
        </div>
      </div>
    </div>

    <!-- Autre (personnalise) -->
    <div v-if="showOther" class="prestation-item prestation-autre">
      <div class="prestation-header">
        <span>{{ $t('wizard.prestations_vip.other') }}</span>
        <button class="btn-sm btn-remove" @click="showOther = false">&#10005;</button>
      </div>

      <div class="prestation-fields">
        <div class="field--full">
          <label>{{ $t('wizard.prestations_vip.description') }}</label>
          <input type="text" v-model="otherDescription" :placeholder="$t('wizard.prestations_vip.description')" />
        </div>

        <div class="field">
          <label>{{ $t('wizard.prestations_vip.quantity') }}</label>
          <input type="number" min="1" v-model.number="otherQuantite" />
        </div>

        <div class="field">
          <label>{{ $t('wizard.prestations_vip.unit_price') }}</label>
          <input type="text" v-model="otherPrice" placeholder="0" />
        </div>

        <div class="field">
          <label>{{ $t('wizard.vols.currency') }}</label>
          <select v-model="otherDevise">
            <option v-for="d in DEVISES" :key="d" :value="d">{{ d }}</option>
          </select>
        </div>

        <div class="field field--full">
          <label>{{ $t('devis.notes_internes') || 'Remarques' }}</label>
          <input type="text" v-model="otherRemarques" />
        </div>
      </div>
    </div>

    <button class="btn btn-secondary" @click="addAutre">
      {{ $t('wizard.prestations_vip.add_prestation') }}
    </button>

    <!-- Ajouter une prestation standard -->
    <div class="step-actions">
      <button class="btn btn-primary" @click="$emit('next')">
        {{ $t('devis.next') }} &rarr;
      </button>
    </div>
  </div>
</template>

<style scoped>
.formulaire-prestations-vip { max-width: 900px; margin: 0 auto; }
.prestation-item { border: 1px solid #e0e0e0; border-radius: 6px; padding: 0.75rem; margin-bottom: 0.75rem; background: white; }
.prestation-autre { border-style: dashed; border-color: var(--color-gold); }
.prestation-header { display: flex; justify-content: space-between; align-items: center; font-weight: 600; font-size: 0.9rem; margin-bottom: 0.5rem; }
.prestation-type { color: var(--color-navy); }
.btn-sm { border: 1px solid #ccc; border-radius: 4px; padding: 0.2rem 0.4rem; cursor: pointer; background: none; color: var(--color-red); }
.prestation-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 0.75rem; }
.field--full { grid-column: 1 / -1; }
.field { display: flex; flex-direction: column; gap: 0.25rem; }
.field label { font-size: 0.8rem; font-weight: 500; color: #555; }
.field input, .field select { padding: 0.4rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.ziyarat-options { background: var(--color-cream); padding: 0.5rem; border-radius: 4px; }
.checkbox-group { display: flex; gap: 1rem; }
.btn-secondary { background-color: var(--color-gold); color: white; border: none; border-radius: 4px; padding: 0.5rem 1rem; cursor: pointer; margin-bottom: 1rem; }
.step-actions { display: flex; justify-content: flex-end; margin-top: 1.5rem; }
</style>
