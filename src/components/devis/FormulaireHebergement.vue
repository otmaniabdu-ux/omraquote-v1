<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useCatalogueStore } from '@/stores/catalogueStore';
import { useCalculNuitees } from '@/composables/useCalculNuitees';
import type { Hebergement } from '@/types/devis.types';

const { locale } = useI18n();
const catalogueStore = useCatalogueStore();

const props = defineProps<{
  devis: any;
  hebergements: Hebergement[];
}>();

const emit = defineEmits(['update', 'next']);

// Segments locaux
const localHebergements = ref<Hebergement[]>(props.hebergements.length > 0 ? [...props.hebergements] : []);

// Autocomplete hôtel (filtré par ville)
const hotelQuery = ref('');
const showHotelDropdown = ref(false);
const filteredHotels = computed(() => {
  if (!hotelQuery.value) return catalogueStore.hotels;
  const q = hotelQuery.value.toLowerCase();
  return catalogueStore.hotels.filter(
    h => h.nom_hotel.toLowerCase().includes(q) || (h.ville ?? '').toLowerCase().includes(q)
  );
});

// Nombre de nuitées indicatif (la valeur canonique est calculee cote Rust)
const computeNuitees = (checkin: string, checkout: string): number | null => {
  const { nuitees } = useCalculNuitees(checkin, checkout);
  return nuitees.value > 0 ? nuitees.value : null;
};

onMounted(async () => {
  await catalogueStore.loadHotels(true);
});

function addHebergement(ville: 'Makkah' | 'Medine') {
  localHebergements.value.push({
    id: undefined,
    devis_id: props.devis?.id || 0,
    ville,
    nom_hotel: '',
    type_chambre: 'double',
    formule_repas: undefined,
    vue: undefined,
    date_checkin: props.devis?.date_depart ?? '',
    date_checkout: '',
    nb_nuitees: 0,
    prix_par_nuit: '0',
    devise_prix: props.devis?.devise_achat || 'SAR',
    taxes_incluses: false,
    remarques: '',
  });
  emitUpdate();
}

function removeHebergement(idx: number) {
  localHebergements.value.splice(idx, 1);
  emitUpdate();
}

function updateHotelName(idx: number, name: string) {
  hotelQuery.value = name; // clear query on select
  showHotelDropdown.value = false;
  if (localHebergements.value[idx]) {
    localHebergements.value[idx].nom_hotel = name;
  }
}

function emitUpdate() {
  emit('update', { hebergements: localHebergements.value });
}
</script>

<template>
  <div class="formulaire-hebergement" :dir="locale === 'ar' ? 'rtl' : 'ltr'">
    <h2>{{ $t('wizard.hebergement.title') }}</h2>

    <!-- Makkah -->
    <div class="ville-block">
      <div class="ville-title">
        {{ $t('wizard.hebergement.title_makkah') }}
        <button class="btn-add" @click="addHebergement('Makkah')">{{ $t('wizard.hebergement.add_hébergement') }}</button>
      </div>

      <div v-for="(h, idx) in localHebergements.filter(h => h.ville === 'Makkah')" :key="'m-' + idx" class="hebergement-item">
        <div class="hebergement-header">
          <span class="hebergement-label">{{ $t('wizard.hebergement.title_makkah') }} #{{ idx + 1 }}</span>
          <button class="btn-sm btn-remove" @click="removeHebergement(localHebergements.indexOf(h))">&#10005;</button>
        </div>

        <div class="hebergement-fields">
          <!-- Autocomplete Hotel -->
          <div class="field--autocomplete">
            <label>{{ $t('wizard.hebergement.hotel') }}</label>
            <input
              :value="hotelQuery"
              @input="hotelQuery = ($event.target as HTMLInputElement).value; updateHotelName(localHebergements.indexOf(h), hotelQuery)"
              @focus="showHotelDropdown = true"
              @blur="() => { setTimeout(() => { showHotelDropdown = false; hotelQuery = ''; }, 200) }"
              type="text"
              :placeholder="$t('wizard.hebergement.hotel')"
            />
            <div v-if="showHotelDropdown && filteredHotels.length > 0" class="autocomplete-dropdown">
              <div
                v-for="hotel in filteredHotels.filter(ht => ht.ville === 'Makkah' || !h.nom_hotel)"
                :key="hotel.id"
                class="autocomplete-item"
                @click="h.nom_hotel = hotel.nom_hotel; hotelQuery = ''; showHotelDropdown = false; emitUpdate()"
              >
                {{ hotel.nom_hotel }} - {{ hotel.ville }}
              </div>
            </div>
          </div>

          <!-- Dates -->
          <div class="field">
            <label>{{ $t('wizard.hebergement.checkin') }}</label>
            <input type="date" v-model="h.date_checkin" @change="emitUpdate()" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.hebergement.checkout') }}</label>
            <input type="date" v-model="h.date_checkout" :min="h.date_checkin" @change="emitUpdate()" />
          </div>

          <!-- Nuitées indicatif -->
          <div class="field field--info">
            <label>{{ $t('wizard.hebergement.nuites') }}</label>
            <span class="nuites-value">{{ computeNuitees(h.date_checkin, h.date_checkout) ?? '--' }}</span>
          </div>

          <!-- Type chambre -->
          <div class="field">
            <label>{{ $t('wizard.hebergement.room_type') }}</label>
            <select v-model="h.type_chambre">
              <option value="single">{{ $t('wizard.hebergement.room_types.single') }}</option>
              <option value="double">{{ $t('wizard.hebergement.room_types.double') }}</option>
              <option value="triple">{{ $t('wizard.hebergement.room_types.triple') }}</option>
              <option value="quadruple">{{ $t('wizard.hebergement.room_types.quadruple') }}</option>
            </select>
          </div>

          <!-- Formule repas -->
          <div class="field">
            <label>{{ $t('wizard.hebergement.meal_plan') }}</label>
            <select v-model="h.formule_repas">
              <option :value="undefined">{{ $t('common.empty') || '—' }}</option>
              <option value="petit_dejeuner">{{ $t('wizard.hebergement.meal_plans.petit_dejeuner') }}</option>
              <option value="demi_pension">{{ $t('wizard.hebergement.meal_plans.demi_pension') }}</option>
              <option value="pension_complete">{{ $t('wizard.hebergement.meal_plans.pension_complete') }}</option>
            </select>
          </div>

          <!-- Vue -->
          <div class="field">
            <label>{{ $t('wizard.hebergement.view') }}</label>
            <select v-model="h.vue">
              <option :value="undefined">{{ $t('common.empty') || '—' }}</option>
              <option value="Kaaba">{{ $t('wizard.hebergement.views.kaaba') }}</option>
              <option value="Haram">{{ $t('wizard.hebergement.views.haram') }}</option>
              <option value="City">{{ $t('wizard.hebergement.views.city') }}</option>
            </select>
          </div>

          <!-- Prix -->
          <div class="field">
            <label>Prix par nuit</label>
            <input type="text" v-model="h.prix_par_nuit" placeholder="0" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.vols.currency') }}</label>
            <select v-model="h.devise_prix">
              <option value="SAR">SAR</option>
              <option value="USD">USD</option>
              <option value="EUR">EUR</option>
              <option value="DZD">DZD</option>
            </select>
          </div>

          <!-- Taxes -->
          <div class="field field--checkbox">
            <label>
              <input type="checkbox" v-model="h.taxes_incluses" />
              {{ $t('wizard.hebergement.taxes_included') }}
            </label>
          </div>

          <!-- Remarques -->
          <div class="field--full">
            <label>{{ $t('wizard.hebergement.remarques') }}</label>
            <input type="text" v-model="h.remarques" />
          </div>
        </div>
      </div>
    </div>

    <!-- Medine -->
    <div class="ville-block">
      <div class="ville-title">
        {{ $t('wizard.hebergement.title_medine') }}
        <button class="btn-add" @click="addHebergement('Medine')">{{ $t('wizard.hebergement.add_hébergement') }}</button>
      </div>

      <div v-for="(h, idx) in localHebergements.filter(h => h.ville === 'Medine')" :key="'m-' + idx" class="hebergement-item">
        <div class="hebergement-header">
          <span class="hebergement-label">{{ $t('wizard.hebergement.title_medine') }} #{{ idx + 1 }}</span>
          <button class="btn-sm btn-remove" @click="removeHebergement(localHebergements.indexOf(h))">&#10005;</button>
        </div>

        <div class="hebergement-fields">
          <!-- Autocomplete Hotel -->
          <div class="field--autocomplete">
            <label>{{ $t('wizard.hebergement.hotel') }}</label>
            <input
              :value="hotelQuery"
              @input="hotelQuery = ($event.target as HTMLInputElement).value"
              @focus="showHotelDropdown = true"
              @blur="() => { setTimeout(() => { showHotelDropdown = false; hotelQuery = ''; }, 200) }"
              type="text"
              :placeholder="$t('wizard.hebergement.hotel')"
            />
            <div v-if="showHotelDropdown && filteredHotels.length > 0" class="autocomplete-dropdown">
              <div
                v-for="hotel in filteredHotels.filter(ht => ht.ville === 'Medine' || !h.nom_hotel)"
                :key="hotel.id"
                class="autocomplete-item"
                @click="h.nom_hotel = hotel.nom_hotel; hotelQuery = ''; showHotelDropdown = false; emitUpdate()"
              >
                {{ hotel.nom_hotel }} - {{ hotel.ville }}
              </div>
            </div>
          </div>

          <div class="field">
            <label>{{ $t('wizard.hebergement.checkin') }}</label>
            <input type="date" v-model="h.date_checkin" @change="emitUpdate()" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.hebergement.checkout') }}</label>
            <input type="date" v-model="h.date_checkout" :min="h.date_checkin" @change="emitUpdate()" />
          </div>

          <div class="field field--info">
            <label>{{ $t('wizard.hebergement.nuites') }}</label>
            <span class="nuites-value">{{ computeNuitees(h.date_checkin, h.date_checkout) ?? '--' }}</span>
          </div>

          <div class="field">
            <label>{{ $t('wizard.hebergement.room_type') }}</label>
            <select v-model="h.type_chambre">
              <option value="single">{{ $t('wizard.hebergement.room_types.single') }}</option>
              <option value="double">{{ $t('wizard.hebergement.room_types.double') }}</option>
              <option value="triple">{{ $t('wizard.hebergement.room_types.triple') }}</option>
              <option value="quadruple">{{ $t('wizard.hebergement.room_types.quadruple') }}</option>
            </select>
          </div>

          <div class="field">
            <label>{{ $t('wizard.hebergement.meal_plan') }}</label>
            <select v-model="h.formule_repas">
              <option :value="undefined">{{ $t('common.empty') || '—' }}</option>
              <option value="petit_dejeuner">{{ $t('wizard.hebergement.meal_plans.petit_dejeuner') }}</option>
              <option value="demi_pension">{{ $t('wizard.hebergement.meal_plans.demi_pension') }}</option>
              <option value="pension_complete">{{ $t('wizard.hebergement.meal_plans.pension_complete') }}</option>
            </select>
          </div>

          <div class="field">
            <label>{{ $t('wizard.hebergement.view') }}</label>
            <select v-model="h.vue">
              <option :value="undefined">{{ $t('common.empty') || '—' }}</option>
              <option value="Kaaba">{{ $t('wizard.hebergement.views.kaaba') }}</option>
              <option value="Haram">{{ $t('wizard.hebergement.views.haram') }}</option>
              <option value="City">{{ $t('wizard.hebergement.views.city') }}</option>
            </select>
          </div>

          <div class="field">
            <label>Prix par nuit</label>
            <input type="text" v-model="h.prix_par_nuit" placeholder="0" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.vols.currency') }}</label>
            <select v-model="h.devise_prix">
              <option value="SAR">SAR</option>
              <option value="USD">USD</option>
              <option value="EUR">EUR</option>
              <option value="DZD">DZD</option>
            </select>
          </div>

          <div class="field field--checkbox">
            <label>
              <input type="checkbox" v-model="h.taxes_incluses" />
              {{ $t('wizard.hebergement.taxes_included') }}
            </label>
          </div>

          <div class="field--full">
            <label>{{ $t('wizard.hebergement.remarques') }}</label>
            <input type="text" v-model="h.remarques" />
          </div>
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
.formulaire-hebergement { max-width: 900px; margin: 0 auto; }
.ville-block { margin-bottom: 2rem; padding: 1rem; border: 1px solid var(--color-gold); border-radius: 8px; background: var(--color-cream); }
.ville-title { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem; font-weight: 700; color: var(--color-navy); font-size: 1.1rem; }
.btn-add { background: var(--color-gold); color: white; border: none; border-radius: 4px; padding: 0.3rem 0.8rem; cursor: pointer; font-size: 0.85rem; }
.hebergement-item { border: 1px solid #e0e0e0; border-radius: 6px; padding: 0.75rem; margin-bottom: 0.75rem; background: white; }
.hebergement-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; font-weight: 600; font-size: 0.9rem; }
.btn-sm { border: 1px solid #ccc; border-radius: 4px; padding: 0.2rem 0.4rem; cursor: pointer; background: none; color: var(--color-red); }
.hebergement-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 0.75rem; position: relative; }
.field, .field--autocomplete, .field--full, .field--checkbox { display: flex; flex-direction: column; gap: 0.25rem; }
.field label, .field--autocomplete label { font-size: 0.8rem; font-weight: 500; color: #555; }
.field input, .field select, .field--autocomplete input { padding: 0.4rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.field--info { justify-content: center; }
.nuites-value { font-weight: 700; color: var(--color-gold); font-size: 1.2rem; text-align: center; }
.autocomplete-dropdown { position: absolute; top: 100%; left: 0; right: 0; background: white; border: 1px solid #ccc; border-radius: 4px; max-height: 200px; overflow-y: auto; z-index: 10; }
.autocomplete-item { padding: 0.4rem 0.6rem; cursor: pointer; font-size: 0.85rem; }
.autocomplete-item:hover { background: var(--color-cream); }
</style>
