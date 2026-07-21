<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useCatalogueStore } from '@/stores/catalogueStore';
import type { SegmentVol } from '@/types/devis.types';

const { locale } = useI18n();
const catalogueStore = useCatalogueStore();

const props = defineProps<{
  devis: any;
  segments: SegmentVol[];
}>();

const emit = defineEmits(['update', 'next']);

// Segments locaux (copie de la ref parent) — le store/parent est source de vérité.
const localSegments = ref<SegmentVol[]>(props.segments.length > 0 ? [...props.segments] : []);

// Autocomplete compagnies
const compagnieQuery = ref('');
const showCompagnieDropdown = ref(false);

onMounted(async () => {
  await catalogueStore.loadCompagnies(true);
});

watch(() => catalogueStore.compagnies, () => {
  // rien à faire de spécial — le v-for dans le template lit directement le store
}, { deep: true });

// Filtrer les compagnies selon la saisie
const filteredCompagnies = computed(() => {
  if (!compagnieQuery.value) return catalogueStore.compagnies;
  const q = compagnieQuery.value.toLowerCase();
  return catalogueStore.compagnies.filter(
    c => c.nom_compagnie.toLowerCase().includes(q) || (c.code_iata?.toLowerCase() ?? '').includes(q)
  );
});

// Ajouter / supprimer / dupliquer un segment
function addSegment() {
  const nextOrdre = localSegments.value.length > 0
    ? Math.max(...localSegments.value.map(s => s.ordre)) + 1
    : 1;
  const isOutbound = nextOrdre % 2 === 1; // segments impairs = aller, pairs = retour
  localSegments.value.push({
    id: undefined,
    devis_id: props.devis?.id || 0,
    ordre: nextOrdre,
    compagnie: '',
    numero_vol: '',
    classe: 'economique',
    date_vol: isOutbound ? (props.devis?.date_depart ?? '') : (props.devis?.date_retour ?? ''),
    aeroport_depart: isOutbound ? '' : (props.devis?.aeroport_arrivee ?? ''),
    aeroport_arrivee: isOutbound ? (props.devis?.aeroport_arrivee ?? '') : '',
    heure_depart: '',
    heure_arrivee: '',
    prix_adulte: '0',
    prix_enfant: '0',
    prix_bebe: '0',
    devise_prix: props.devis?.devise_achat || 'SAR',
    remarques: '',
  });
  emitUpdate();
}

function duplicateSegment(idx: number) {
  const copy = { ...localSegments.value[idx] };
  const nextOrdre = localSegments.value.length > 0
    ? Math.max(...localSegments.value.map(s => s.ordre)) + 1
    : 2;
  copy.id = undefined;
  // Basculer la classe (aller->retour: inverse les aéroports)
  const tempDepart = copy.aeroport_depart;
  copy.aeroport_depart = copy.aeroport_arrivee;
  copy.aeroport_arrivee = tempDepart;
  copy.ordre = nextOrdre;
  localSegments.value.splice(idx + 1, 0, copy);
  // Re-number all segments
  renumberSegments();
  emitUpdate();
}

function removeSegment(idx: number) {
  localSegments.value.splice(idx, 1);
  renumberSegments();
  emitUpdate();
}

function renumberSegments() {
  localSegments.value.forEach((s, i) => { s.ordre = i + 1; });
}

// Réordonner (boutons haut/bas)
function moveSegment(idx: number, dir: -1 | 1) {
  const newIdx = idx + dir;
  if (newIdx < 0 || newIdx >= localSegments.value.length) return;
  [localSegments.value[idx], localSegments.value[newIdx]] = [localSegments.value[newIdx], localSegments.value[idx]];
  renumberSegments();
  emitUpdate();
}

function isOutboundSegment(idx: number): boolean {
  // Convention métier: segment impair (1-indexed) = aller, pair = retour
  return (idx + 1) % 2 === 1;
}

function emitUpdate() {
  emit('update', { segments: localSegments.value });
}
</script>

<template>
  <div class="formulaire-vols" :dir="locale === 'ar' ? 'rtl' : 'ltr'">
    <h2>{{ $t('wizard.vols.title') }}</h2>

    <div class="segment-list">
      <div
        v-for="(seg, idx) in localSegments"
        :key="idx"
        class="segment-item"
        :class="{ 'segment--outbound': isOutboundSegment(idx), 'segment--return': !isOutboundSegment(idx) }"
      >
        <div class="segment-header">
          <span class="segment-label">
            {{ isOutboundSegment(idx) ? `${$t('wizard.vols.outbound')} #${idx + 1}` : `${$t('wizard.vols.return')} #${idx + 1}` }}
          </span>
          <div class="segment-actions">
            <button v-if="idx > 0" class="btn-sm" @click="moveSegment(idx, -1)" title="Monter">&#9650;</button>
            <button v-if="idx < localSegments.length - 1" class="btn-sm" @click="moveSegment(idx, 1)" title="Descendre">&#9660;</button>
            <button class="btn-sm btn-duplicate" @click="duplicateSegment(idx)" :title="$t('wizard.vols.duplicate_segment')">
              {{ $t('devis.duplicate') || 'Dupliquer' }}
            </button>
            <button class="btn-sm btn-remove" @click="removeSegment(idx)">&#10005;</button>
          </div>
        </div>

        <div class="segment-fields">
          <!-- Compagnie (autocomplete) -->
          <div class="field field--autocomplete">
            <label>{{ $t('wizard.vols.company') }}</label>
            <div class="autocomplete-wrapper">
              <input
                :value="compagnieQuery"
                @input="compagnieQuery = ($event.target as HTMLInputElement).value"
                @focus="showCompagnieDropdown = true"
                @blur="() => { setTimeout(() => { showCompagnieDropdown = false; compagnieQuery = ''; }, 200); }"
                type="text"
                :placeholder="$t('wizard.vols.company')"
                list="compagnies-list"
              />
              <datalist id="compagnies-list">
                <option v-for="c in filteredCompagnies" :key="c.id" :value="c.nom_compagnie" />
              </datalist>
            </div>
          </div>

          <!-- Classe -->
          <div class="field">
            <label>{{ $t('wizard.vols.class') }}</label>
            <select v-model="seg.classe">
              <option value="economique">{{ $t('wizard.vols.classes.economique') }}</option>
              <option value="affaires">{{ $t('wizard.vols.classes.affaires') }}</option>
              <option value="premiere">{{ $t('wizard.vols.classes.premiere') }}</option>
            </select>
          </div>

          <!-- Date -->
          <div class="field">
            <label>{{ $t('wizard.vols.flight_date') }}</label>
            <input type="date" v-model="seg.date_vol" />
          </div>

          <!-- Aéroports -->
          <div class="field">
            <label>{{ $t('wizard.vols.departure_airport') }}</label>
            <input type="text" v-model="seg.aeroport_depart" :placeholder="$t('wizard.vols.departure_airport')" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.vols.arrival_airport') }}</label>
            <input type="text" v-model="seg.aeroport_arrivee" :placeholder="$t('wizard.vols.arrival_airport')" />
          </div>

          <!-- Numéros de vol -->
          <div class="field">
            <label>{{ $t('wizard.vols.flight_number') }}</label>
            <input type="text" v-model="seg.numero_vol" :placeholder="$t('wizard.vols.flight_number')" />
          </div>

          <!-- Horaires -->
          <div class="field">
            <label>{{ $t('wizard.vols.departure_time') }}</label>
            <input type="time" v-model="seg.heure_depart" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.vols.arrival_time') }}</label>
            <input type="time" v-model="seg.heure_arrivee" />
          </div>

          <!-- Prix (3 champs monétaires — chaînes décimales) -->
          <div class="field">
            <label>{{ $t('wizard.vols.prices.adulte') }}</label>
            <input type="text" v-model="seg.prix_adulte" placeholder="0" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.vols.prices.enfant') }}</label>
            <input type="text" v-model="seg.prix_enfant" placeholder="0" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.vols.prices.bebe') }}</label>
            <input type="text" v-model="seg.prix_bebe" placeholder="0" />
          </div>

          <!-- Devise -->
          <div class="field">
            <label>{{ $t('wizard.vols.currency') }}</label>
            <select v-model="seg.devise_prix">
              <option value="SAR">SAR</option>
              <option value="USD">USD</option>
              <option value="EUR">EUR</option>
              <option value="DZD">DZD</option>
            </select>
          </div>

          <!-- Remarques -->
          <div class="field field--full">
            <label>{{ $t('devis.notes_internes') || 'Remarques' }}</label>
            <input type="text" v-model="seg.remarques" />
          </div>
        </div>
      </div>
    </div>

    <button class="btn btn-secondary" @click="addSegment">
      {{ $t('wizard.vols.add_segment') }}
    </button>

    <div class="step-actions">
      <button class="btn btn-primary" @click="$emit('next')">
        {{ $t('devis.next') }} &rarr;
      </button>
    </div>
  </div>
</template>

<style scoped>
.formulaire-vols { max-width: 900px; margin: 0 auto; }
.segment-list { display: flex; flex-direction: column; gap: 1rem; margin: 1.5rem 0; }
.segment-item { border: 1px solid #e0e0e0; border-radius: 8px; padding: 1rem; background: var(--color-cream); position: relative; }
.segment--outbound { border-left: 4px solid #2e7d32; }
.segment--return { border-left: 4px solid var(--color-blue-royal); }
[dir="rtl"] .segment--outbound { border-left: none; border-right: 4px solid #2e7d32; }
[dir="rtl"] .segment--return { border-left: none; border-right: 4px solid var(--color-blue-royal); }
.segment-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem; }
.segment-label { font-weight: 700; color: var(--color-navy); }
.segment-actions { display: flex; gap: 0.25rem; }
.btn-sm { background: none; border: 1px solid #ccc; border-radius: 4px; padding: 0.2rem 0.4rem; cursor: pointer; font-size: 0.8rem; }
.btn-sm:hover { background: var(--color-gold); color: white; }
.btn-duplicate { color: var(--color-blue-royal); border-color: var(--color-blue-royal); }
.btn-remove { color: var(--color-red); border-color: var(--color-red); }
.segment-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 0.75rem; }
.field--full { grid-column: 1 / -1; }
.field { display: flex; flex-direction: column; gap: 0.25rem; }
.field label { font-size: 0.8rem; font-weight: 500; color: #555; }
.field input, .field select { padding: 0.4rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.btn-secondary { background-color: var(--color-gold); color: white; border: none; border-radius: 4px; padding: 0.5rem 1rem; cursor: pointer; margin-bottom: 1rem; }
.step-actions { display: flex; justify-content: flex-end; }
</style>
