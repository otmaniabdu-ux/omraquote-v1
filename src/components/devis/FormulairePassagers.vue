<template>
  <div class="formulaire-passagers" :dir="locale === 'ar' ? 'rtl' : 'ltr'">
    <h2>{{ $t('wizard.passagers.title') }}</h2>
    <p class="hint">{{ $t('wizard.passagers.hint') }}</p>

    <div class="passager-list">
      <div v-for="(p, idx) in passagers" :key="'pass-' + idx" class="passager-item">
        <div class="passager-header">
          <span class="passager-number">#{{ idx + 1 }}</span>
          <span class="passager-categorie">{{ p.categorie }}</span>
          <button class="btn-remove" @click="removePassager(idx)">&#10005;</button>
        </div>
        <div class="passager-fields">
          <div class="field">
            <label>{{ $t('wizard.passagers.full_name') }}</label>
            <input v-model="p.nom_complet" type="text" :placeholder="$t('wizard.passagers.full_name')" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.passagers.birth_date') }}</label>
            <input v-model="p.date_naissance" type="date" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.passagers.category') }}</label>
            <select v-model="p.categorie">
              <option value="adulte">{{ $t('wizard.passagers.categories.adulte') }}</option>
              <option value="enfant_avec_lit">{{ $t('wizard.passagers.categories.enfant_avec_lit') }}</option>
              <option value="enfant_sans_lit">{{ $t('wizard.passagers.categories.enfant_sans_lit') }}</option>
              <option value="bebe">{{ $t('wizard.passagers.categories.bebe') }}</option>
            </select>
          </div>
          <div class="field">
            <label>{{ $t('wizard.passagers.nationality') }}</label>
            <input v-model="p.nationalite" type="text" :placeholder="$t('wizard.passagers.nationality')" />
          </div>
          <div class="field">
            <label>{{ $t('wizard.passagers.passport_number') }}</label>
            <input v-model="p.numero_passeport" type="text" :placeholder="$t('wizard.passagers.passport_number')" />
          </div>
          <div class="field" :class="{ 'field--alert': alertePasseport(p) }">
            <label>{{ $t('wizard.passagers.passport_expiry') }}</label>
            <input v-model="p.date_expiration_passeport" type="date" />
            <span v-if="alertePasseport(p)" class="alert-msg">{{ $t('alert.passeport_text') }}</span>
          </div>
        </div>
      </div>
    </div>

    <button class="btn btn-secondary" @click="addPassager">
      {{ $t('wizard.passagers.add') }}
    </button>

    <div class="step-actions">
      <button class="btn btn-primary" @click="$emit('next')">
        {{ $t('devis.next') }} &rarr;
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Passager } from '@/types/devis.types';
import { useValidationPasseport } from '@/composables/useValidationPasseport';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();

const props = defineProps<{
  devis: any;
  passagers: Passager[];
}>();

const emit = defineEmits(['update', 'next']);

const localPassagers = ref<Passager[]>([...props.passagers]);

function addPassager() {
  localPassagers.value.push({
    devis_id: props.devis?.id || 0,
    categorie: 'adulte',
    nom_complet: '',
    date_naissance: new Date().toISOString().split('T')[0],
    nationalite: '',
    numero_passeport: '',
    date_expiration_passeport: '',
    lieu_delivrance: '',
    remarques: '',
  });
  emitUpdate();
}

function removePassager(idx: number) {
  localPassagers.value.splice(idx, 1);
  emitUpdate();
}

function alertePasseport(p: Passager): boolean {
  if (!p.date_expiration_passeport || !props.devis?.date_retour) return false;
  const { alerte } = useValidationPasseport(p.date_expiration_passeport, props.devis.date_retour);
  return alerte.value;
}

function emitUpdate() {
  emit('update', { passagers: localPassagers.value });
}

watch(localPassagers, () => emitUpdate(), { deep: true });
</script>

<style scoped>
.formulaire-passagers { max-width: 800px; margin: 0 auto; }
.hint { color: #666; font-size: 0.9rem; margin-bottom: 1rem; }
.passager-list { display: flex; flex-direction: column; gap: 1.5rem; margin: 1.5rem 0; }
.passager-item { border: 1px solid #e0e0e0; border-radius: 8px; padding: 1rem; background: #f9f9f9; }
.passager-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem; }
.passager-number { font-weight: 700; color: var(--color-navy); }
.passager-categorie { font-size: 0.8rem; padding: 0.2rem 0.6rem; background: var(--color-cream); border-radius: 12px; }
.btn-remove { background: none; border: none; color: var(--color-red); font-size: 1.2rem; cursor: pointer; }
.passager-fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 0.75rem; }
.field { display: flex; flex-direction: column; gap: 0.25rem; }
.field label { font-size: 0.8rem; font-weight: 500; color: #555; }
.field input, .field select { padding: 0.4rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; font-size: 0.9rem; }
.field--alert { border-left: 3px solid var(--color-red); padding-left: 0.5rem; }
.alert-msg { font-size: 0.75rem; color: var(--color-red); margin-top: 0.2rem; }
.step-actions { display: flex; justify-content: flex-end; margin-top: 1.5rem; }
</style>
