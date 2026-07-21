<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Transfert } from '@/types/devis.types';

const { t, locale } = useI18n();

const props = defineProps<{
  devis: any;
  transferts: Transfert[];
}>();

const emit = defineEmits(['update', 'next']);

// Types de vehicules disponibles
const VEHICULES = [
  { value: 'GMC_Yukon', label: t('wizard.transferts.vehicules.GMC_Yukon') },
  { value: 'Mercedes_Classe_E', label: t('wizard.transferts.vehicules.Mercedes_Classe_E') },
  { value: 'Bus_VIP_prive', label: t('wizard.transferts.vehicules.Bus_VIP_prive') },
] as const;

const DEVISES = ['SAR', 'USD', 'EUR', 'DZD'];

// Transferts obligatoires pre-remplis
const obligatoireTrajets = [
  { trajet: t('wizard.transferts.trajets.aeroport_makkah_hotel'), ville: 'Makkah' },
  { trajet: t('wizard.transferts.trajets.makkah_hotel_aeroport') as string, ville: 'Makkah' },
  { trajet: t('wizard.transferts.trajets.aeroport_medine_hotel'), ville: 'Medine' },
  { trajet: t('wizard.transferts.trajets.medine_hotel_aeroport'), ville: 'Medine' },
] as const;

const localTransferts = ref<Transfert[]>([]);

// Initialiser les transferts obligatoires si vides
function initObligatoires() {
  if (localTransferts.value.length === 0) {
    localTransferts.value = obligatoireTrajets.map(tj => ({
      id: undefined,
      devis_id: props.devis?.id || 0,
      type_transfert: 'obligatoire',
      trajet: tj.trajet,
      type_vehicule: 'GMC_Yukon' as const,
      date_transfert: '',
      heure_transfert: '',
      nombre_vehicules: 1,
      prix_unitaire: '0',
      devise_prix: props.devis?.devise_achat || 'SAR',
      remarques: '',
    }));
  }
}

// Transferts optionnels (ajoutables par l'utilisateur, max 2)
const nbOptionnels = ref(0);
const localOptionnels = ref<Transfert[]>([]);

function addOptionnel() {
  if (nbOptionnels.value >= 2) return;
  const nextOrdre = Math.max(...localTransferts.value.map(tr => tr.id ?? 0), ...localOptionnels.value.map(tr => tr.id ?? 0)) + 1;
  localOptionnels.value.push({
    id: nextOrdre > 0 ? undefined : nextOrdre,
    devis_id: props.devis?.id || 0,
    type_transfert: 'optionnel',
    trajet: '',
    type_vehicule: 'GMC_Yukon',
    date_transfert: '',
    heure_transfert: '',
    nombre_vehicules: 1,
    prix_unitaire: '0',
    devise_prix: props.devis?.devise_achat || 'SAR',
    remarques: '',
  });
  nbOptionnels.value++;
}

function removeOptionnel(idx: number) {
  localOptionnels.value.splice(idx, 1);
  nbOptionnels.value = localOptionnels.value.length;
}

// Train Haramain
const trainHaramain = ref({
  trajet: 'Makkah-Medine',
  classe: 'economique' as 'economique' | 'business',
  nb_passagers: 1,
  prix_par_passager: '0',
});

// Devise train (saisie locale, pas de v-model sur props)
const trainDevise = ref(props.devis?.devise_achat || 'SAR');

// Fusionner les transferts pour emit
const allTransferts = computed(() => [
  ...localTransferts.value,
  ...localOptionnels.value,
]);

watch([allTransferts, trainDevise], () => {
  emit('update', {
    transferts: allTransferts.value,
    trainHaramain: { ...trainHaramain.value },
    trainDevise: trainDevise.value,
  });
}, { deep: true });

// Sync initial depuis props
watch(() => props.transferts, (newVal) => {
  if (newVal && newVal.length > 0) {
    initObligatoires(); // force initialisation si nécessaire
  }
}, { immediate: true });
</script>

<template>
  <div class="formulaire-transferts" :dir="locale === 'ar' ? 'rtl' : 'ltr'">
    <h2>{{ $t('wizard.transferts.title') }}</h2>

    <!-- Obligatoires -->
    <div class="section obligatory">
      <h3>{{ $t('wizard.transferts.required_transfers') }}</h3>
      <div v-for="(t, idx) in localTransferts" :key="'o-' + idx" class="transfert-item">
        <div class="transfert-fields">
          <!-- Trajet (read-only pour obligatoires) -->
          <div class="field--full">
            <label>{{ $t('wizard.transferts.title') }}</label>
            <input type="text" :value="t.trajet" readonly class="readonly-field" />
          </div>

          <!-- Vehicule -->
          <div class="field">
            <label>{{ $t('wizard.transferts.type_vehicule') }}</label>
            <select v-model="t.type_vehicule">
              <option v-for="v in VEHICULES" :key="v.value" :value="v.value">{{ v.label }}</option>
            </select>
          </div>

          <!-- Date / Heure -->
          <div class="field">
            <label>{{ $t('wizard.transferts.date') }}</label>
            <input type="date" v-model="t.date_transfert" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.transferts.time') }}</label>
            <input type="time" v-model="t.heure_transfert" />
          </div>

          <!-- Nombre vehicules -->
          <div class="field">
            <label>{{ $t('wizard.transferts.nb_vehicles') }}</label>
            <input type="number" min="1" v-model.number="t.nombre_vehicules" />
          </div>

          <!-- Prix unitaire (devise du devis) -->
          <div class="field">
            <label>{{ $t('wizard.transferts.unit_price') }}</label>
            <input type="text" v-model="t.prix_unitaire" placeholder="0" />
          </div>

          <!-- Devise -->
          <div class="field">
            <label>{{ $t('wizard.vols.currency') }}</label>
            <select v-model="t.devise_prix">
              <option v-for="d in DEVISES" :key="d" :value="d">{{ d }}</option>
            </select>
          </div>

          <!-- Remarques -->
          <div class="field--full">
            <label>{{ $t('devis.notes_internes') || 'Remarques' }}</label>
            <input type="text" v-model="t.remarques" />
          </div>
        </div>
      </div>
    </div>

    <!-- Optionnels -->
    <div class="section optionals">
      <h3>{{ $t('wizard.transferts.optional_transfers') }}</h3>
      <button class="btn-add" @click="addOptionnel" :disabled="nbOptionnels >= 2">
        {{ $t('wizard.transferts.add_transfer') }} ({{ nbOptionnels }}/2)
      </button>

      <div v-for="(t, idx) in localOptionnels" :key="'x-' + idx" class="transfert-item transfert-optionnel">
        <div class="transfert-header">
          <span>{{ $t('wizard.transferts.optional_transfers') }} #{{ idx + 1 }}</span>
          <button class="btn-sm btn-remove" @click="removeOptionnel(idx)">&#10005;</button>
        </div>

        <div class="transfert-fields">
          <div class="field--full">
            <label>{{ $t('wizard.transferts.title') }}</label>
            <input type="text" v-model="t.trajet" :placeholder="$t('wizard.transferts.title')" />
          </div>

          <div class="field">
            <label>{{ $t('wizard.transferts.type_vehicule') }}</label>
            <select v-model="t.type_vehicule">
              <option v-for="v in VEHICULES" :key="v.value" :value="v.value">{{ v.label }}</option>
            </select>
          </div>

          <div class="field">
            <label>{{ $t('wizard.transferts.date') }}</label>
            <input type="date" v-model="t.date_transfert" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.transferts.time') }}</label>
            <input type="time" v-model="t.heure_transfert" />
          </div>

          <div class="field">
            <label>{{ $t('wizard.transferts.nb_vehicles') }}</label>
            <input type="number" min="1" v-model.number="t.nombre_vehicules" />
          </div>

          <div class="field">
            <label>{{ $t('wizard.transferts.unit_price') }}</label>
            <input type="text" v-model="t.prix_unitaire" placeholder="0" />
          </div>

          <div class="field">
            <label>{{ $t('wizard.vols.currency') }}</label>
            <select v-model="t.devise_prix">
              <option v-for="d in DEVISES" :key="d" :value="d">{{ d }}</option>
            </select>
          </div>

          <div class="field--full">
            <label>{{ $t('devis.notes_internes') || 'Remarques' }}</label>
            <input type="text" v-model="t.remarques" />
          </div>
        </div>
      </div>
    </div>

    <!-- Train Haramain -->
    <div class="section train">
      <h3>{{ $t('wizard.transferts.train_section') }}</h3>
      <div class="train-fields">
        <div class="field">
          <label>{{ $t('wizard.transferts.train_trajet') }}</label>
          <select v-model="trainHaramain.trajet">
            <option value="Makkah-Medine">Makkah -> Medine</option>
            <option value="Medine-Makkah">Medine -> Makkah</option>
          </select>
        </div>

        <div class="field">
          <label>{{ $t('wizard.transferts.train_classe') }}</label>
          <select v-model="trainHaramain.classe">
            <option value="economique">Economique</option>
            <option value="business">Business</option>
          </select>
        </div>

        <div class="field">
          <label>{{ $t('wizard.transferts.train_nb_passagers') }}</label>
          <input type="number" min="1" v-model.number="trainHaramain.nb_passagers" />
        </div>

        <div class="field">
          <label>{{ $t('wizard.transferts.train_price_per_passenger') }}</label>
          <input type="text" v-model="trainHaramain.prix_par_passager" placeholder="0" />
        </div>

        <div class="field">
          <label>{{ $t('wizard.vols.currency') }}</label>
          <select v-model="trainDevise">
            <option v-for="d in DEVISES" :key="d" :value="d">{{ d }}</option>
          </select>
        </div>
      </div>
    </div>

    <div class="step-actions">
      <button class="btn btn-primary" @click="$emit('next')">
        {{ $t('devis.next') }} &rarr;
      </button>
    </div>
  </div>
</template>

<style scoped>
.formulaire-transferts { max-width: 900px; margin: 0 auto; }
.section { margin-bottom: 1.5rem; padding: 1rem; border-radius: 8px; }
.section.obligatory { background: var(--color-cream); border: 1px solid var(--color-gold); }
.section.optionals { background: #f0f0f0; border: 1px dashed #ccc; }
.section.train { background: #e8f4fd; border: 1px solid var(--color-blue-royal); }
.transfert-item { border: 1px solid #e0e0e0; border-radius: 6px; padding: 0.75rem; margin-bottom: 0.75rem; background: white; }
.transfert-optionnel { border-style: dashed; }
.transfert-header { display: flex; justify-content: space-between; align-items: center; font-weight: 600; font-size: 0.9rem; margin-bottom: 0.5rem; }
.btn-add { background: var(--color-gold); color: white; border: none; border-radius: 4px; padding: 0.3rem 0.8rem; cursor: pointer; margin-bottom: 0.75rem; }
.btn-sm { border: 1px solid #ccc; border-radius: 4px; padding: 0.2rem 0.4rem; cursor: pointer; background: none; color: var(--color-red); }
.btn-remove { color: var(--color-red); }
.section h3 { margin-top: 0; color: var(--color-navy); font-size: 1rem; }
.transfert-fields, .train-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 0.75rem; }
.field--full { grid-column: 1 / -1; }
.field { display: flex; flex-direction: column; gap: 0.25rem; }
.field label { font-size: 0.8rem; font-weight: 500; color: #555; }
.field input, .field select { padding: 0.4rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.readonly-field { background: #f5f5f5; cursor: default; }
.step-actions { display: flex; justify-content: flex-end; margin-top: 1.5rem; }
</style>
