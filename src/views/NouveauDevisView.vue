<script setup lang="ts">
import { ref, computed, onMounted, type Ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useI18n } from 'vue-i18n';
import { useDevisStore } from '@/stores/devisStore';
import type { Devis, Passager, SegmentVol, Hebergement, Transfert, PrestationVip, DevisCreate } from '@/types/devis.types';

// Composants d'etapes
import FormulairePassagers from '@/components/devis/FormulairePassagers.vue';
import FormulaireVols from '@/components/devis/FormulaireVols.vue';
import FormulaireHebergement from '@/components/devis/FormulaireHebergement.vue';
import FormulaireTransferts from '@/components/devis/FormulaireTransferts.vue';
import FormulairePrestationsVIP from '@/components/devis/FormulairePrestationsVIP.vue';
import FormulaireFinancier from '@/components/devis/FormulaireFinancier.vue';
import RecapitulatifDevis from '@/components/devis/RecapitulatifDevis.vue';

const { t, locale } = useI18n();
const route = useRoute();
const router = useRouter();
const devisStore = useDevisStore();

// --- Etapes du wizard ---
interface Etape {
  labelKey: string;
  component: string;
  completed: boolean;
}

const etapes: Ref<Etape[]> = ref([
  { labelKey: 'wizard.passagers.title', component: 'FormulairePassagers', completed: false },
  { labelKey: 'wizard.vols.title', component: 'FormulaireVols', completed: false },
  { labelKey: 'wizard.hebergement.title_makkah', component: 'FormulaireHebergement', completed: false },
  { labelKey: 'wizard.transferts.title', component: 'FormulaireTransferts', completed: false },
  { labelKey: 'wizard.prestations_vip.title', component: 'FormulairePrestationsVIP', completed: false },
  { labelKey: 'wizard.financier.title', component: 'FormulaireFinancier', completed: false },
  { labelKey: 'wizard.recapitulatif.title', component: 'RecapitulatifDevis', completed: false },
]);

const currentStep = ref(0);
const currentEtape = computed(() => etapes.value[currentStep.value]);

const etapeLabels = computed(() => etapes.value.map(e => t(e.labelKey)));

function goToStep(idx: number) {
  if (idx < currentStep.value) {
    currentStep.value = idx;
  }
}

function nextStep() {
  if (currentStep.value < etapes.value.length - 1) {
    etapes.value[currentStep.value].completed = true;
    currentStep.value++;
  }
}

function prevStep() {
  if (currentStep.value > 0) {
    currentStep.value--;
  }
}

// --- Donnees line items (refs synchronisees avec le parent) ---
const passagers = ref<Passager[]>([]);
const segments = ref<SegmentVol[]>([]);
const hebergements = ref<Hebergement[]>([]);
const transfertsData = ref<{ transferts: Transfert[]; trainHaramain: any }>({
  transferts: [],
  trainHaramain: {},
});
const prestationsData = ref<{ prestations: PrestationVip[] }>({ prestations: [] });

// --- Devis courant ---
const isEdit = computed(() => !!route.query.edit);
const currentDevis = ref<Devis | null>(null);

function getComponentForStep(key: string) {
  switch (key) {
    case 'FormulairePassagers': return FormulairePassagers;
    case 'FormulaireVols': return FormulaireVols;
    case 'FormulaireHebergement': return FormulaireHebergement;
    case 'FormulaireTransferts': return FormulaireTransferts;
    case 'FormulairePrestationsVIP': return FormulairePrestationsVIP;
    case 'FormulaireFinancier': return FormulaireFinancier;
    case 'RecapitulatifDevis': return RecapitulatifDevis;
    default: return null;
  }
}

// --- Validation par etape ---
function validateEtape(idx: number): { valid: boolean; messages?: string[] } {
  const msgs: string[] = [];
  switch (idx) {
    case 0: // Passagers
      if (passagers.value.length === 0) msgs.push(t('wizard.passagers.hint'));
      for (const p of passagers.value) {
        if (!p.nom_complet.trim()) msgs.push(`${t('common.please_fill_name')} (${p.categorie})`);
        if (!p.date_naissance) msgs.push(`${t('common.please_fill_birthdate')} (${p.categorie})`);
      }
      break;
    case 1: // Vols
      if (segments.value.length === 0) msgs.push(t('wizard.vols.hint'));
      for (const s of segments.value) {
        if (!s.compagnie.trim()) msgs.push(`Compagnie requise`);
        if (!s.date_vol) msgs.push(`Date requise`);
        if (!s.aeroport_depart.trim()) msgs.push(`Aeroport depart requis`);
        if (!s.aeroport_arrivee.trim()) msgs.push(`Aeroport arrivee requis`);
      }
      break;
    case 2: // Hebergement
      for (const h of hebergements.value) {
        if (!h.nom_hotel.trim()) msgs.push('Hotel requis');
        if (!h.date_checkin) msgs.push('Date checkin requise');
        if (!h.date_checkout || (h.date_checkin && h.date_checkout <= h.date_checkin))
          msgs.push('Date checkout doit etre apres checkin');
      }
      break;
    case 3: // Transferts — pas de validation stricte necessaire
      break;
    case 4: // Prestations VIP — pas de validation stricte necessaire
      break;
    case 5: // Financier — taux et marge obligatoires
      break;
    default:
      break;
  }
  return { valid: msgs.length === 0, messages: msgs.length > 0 ? msgs : undefined };
}

// --- Save devis (Task 1) ---
async function saveDevis() {
  // 1. Collecter tous les line items depuis les refs
  const allTransferts = transfertsData.value.transferts || [];
  const allPrestations = prestationsData.value.prestations || [];

  // 2. Valider chaque etape
  for (let i = 0; i < currentStep.value; i++) {
    const result = validateEtape(i);
    if (!result.valid) {
      alert(result.messages?.join('\n') || 'Donnees incomplete');
      currentStep.value = i;
      return;
    }
  }

  // 3. Valider l'etape courante
  const finalValidation = validateEtape(currentStep.value);
  if (!finalValidation.valid) {
    alert(finalValidation.messages?.join('\n') || 'Donnees incomplete');
    return;
  }

  // 4. Créer le devis principal via devisStore.createDevis()
  const devisData: DevisCreate = {
    client_id: currentDevis.value?.client_id || 0,
    date_depart: currentDevis.value?.date_depart || '',
    date_retour: currentDevis.value?.date_retour || '',
    type_visa: 'omra_standard',
    assurance_medicale: true,
    devise_achat: currentDevis.value?.devise_achat || 'SAR',
    taux_sar_dzd: currentDevis.value?.taux_sar_dzd || '1.0000',
    taux_usd_dzd: currentDevis.value?.taux_usd_dzd || '1.0000',
    taux_eur_dzd: currentDevis.value?.taux_eur_dzd || '1.0000',
    marge_type: 'pourcentage',
    marge_valeur: '15',
    statut: 'brouillon',
  };

  let createdDevis: Devis | null = null;
  try {
    if (isEdit.value && currentDevis.value?.id) {
      createdDevis = await devisStore.updateDevis(currentDevis.value.id, devisData);
    } else {
      createdDevis = await devisStore.createDevis(devisData);
    }
  } catch (e) {
    alert(`Erreur creation du devis: ${String(e)}`);
    return;
  }

  if (!createdDevis?.id) {
    alert('Impossible de sauvegarder le devis.');
    return;
  }

  // 5. Pour chaque line item, appeler la Tauri command correspondante
  try {
    // Passagers
    for (const p of passagers.value) {
      await invoke('create_passager', {
        passagerData: { ...p, devis_id: createdDevis.id },
      });
    }

    // Segments vols
    for (const s of segments.value) {
      await invoke('create_segment_vol', {
        segmentData: { ...s, devis_id: createdDevis.id },
      });
    }

    // Hebergements
    for (const h of hebergements.value) {
      await invoke('create_hebergement', {
        hebergementData: { ...h, devis_id: createdDevis.id },
      });
    }

    // Transferts
    for (const tr of allTransferts) {
      await invoke('create_transfert', {
        transfertData: { ...tr, devis_id: createdDevis.id },
      });
    }

    // Prestations VIP
    for (const pr of allPrestations) {
      await invoke('create_prestation_vip', {
        prestationData: { ...pr, devis_id: createdDevis.id },
      });
    }
  } catch (e) {
    console.error('Erreur creation line items:', e);
    alert(`Erreur sauvegarde des elements secondaires: ${String(e)}`);
    return;
  }

  // 6. Recalculer les totaux cote backend
  try {
    await devisStore.recalculerTotaux(createdDevis.id!);
  } catch (e) {
    console.error('Erreur recalcul totaux:', e);
  }

  // 7. Marquer toutes les etapes comme completees
  etapes.value.forEach(e => { e.completed = true; });

  // 8. Rediriger vers /devis/liste?edit=${devis.id}
  router.push(`/devis/liste?edit=${createdDevis.id}`);
}

// --- Reception des mises a jour depuis les composants enfants ---
function handleUpdate(data: Record<string, any>) {
  for (const [key, value] of Object.entries(data)) {
    switch (key) {
      case 'passagers': passagers.value = value as Passager[]; break;
      case 'segments': segments.value = value as SegmentVol[]; break;
      case 'hebergements': hebergements.value = value as Hebergement[]; break;
      case 'transferts':
        transfertsData.value.transferts = (value as Transfert[]) || [];
        break;
      case 'prestations': prestationsData.value.prestations = (value as PrestationVip[]) || [];
        break;
      default: break;
    }
  }
}

// --- Generation PDF ---
async function generatePdf() {
  try {
    await invoke('generer_devis_pdf', {
      devisId: currentDevis.value?.id,
    });
  } catch (e) {
    alert(`Erreur generation PDF: ${String(e)}`);
  }
}

// --- Chargement initial ---
onMounted(async () => {
  await devisStore.loadDevis();
  const editId = route.query.edit;
  if (editId) {
    currentDevis.value = await devisStore.getDevisById(Number(editId));
    // Charger les donnees associees depuis le backend
    try {
      passagers.value = await invoke<Passager[]>('get_passagers_par_devis', { devis_id: Number(editId) });
      segments.value = await invoke<SegmentVol[]>('get_segments_par_devis', { devis_id: Number(editId) });
      hebergements.value = await invoke<Hebergement[]>('get_hebergements_par_devis', { devis_id: Number(editId) });
    } catch (e) {
      console.error('Erreur chargement donnees:', e);
    }
  } else {
    // Nouveau devis : valeurs par defaut
    currentDevis.value = {
      id: undefined,
      numero_devis: '...',
      client_id: 0,
      date_depart: new Date().toISOString().split('T')[0],
      date_retour: new Date(Date.now() + 7 * 86400000).toISOString().split('T')[0],
      type_visa: 'omra_standard',
      assurance_medicale: true,
      devise_achat: 'SAR',
      taux_sar_dzd: '1.0000',
      taux_usd_dzd: '1.0000',
      taux_eur_dzd: '1.0000',
      marge_type: 'pourcentage',
      marge_valeur: '15',
      statut: 'brouillon',
      date_creation: new Date().toISOString().split('T')[0],
    } as Devis;
  }
});
</script>

<template>
  <div class="nouveau-devis" dir="ltr">
    <!-- Barre d'etapes -->
    <div class="steps">
      <button
        v-for="(step, idx) in etapes"
        :key="idx"
        class="step-btn"
        :class="{ active: currentStep === idx, completed: step.completed }"
        @click="goToStep(idx)"
      >
        <span class="step-number">{{ idx + 1 }}</span>
        <span class="step-label">{{ etapeLabels[idx] }}</span>
      </button>
    </div>

    <!-- Contenu de l'etape -->
    <div class="step-content">
      <component
        :is="getComponentForStep(currentEtape.component)"
        :devis="currentDevis"
        :passagers="passagers"
        :segments="segments"
        :hebergements="hebergements"
        :transferts="transfertsData.transferts"
        :prestations="prestationsData.prestations"
        @update="handleUpdate"
        @next="nextStep"
        @prev="prevStep"
        @save="saveDevis"
        @generate-pdf="generatePdf"
      />
    </div>

    <!-- Navigation -->
    <div class="step-nav">
      <button v-if="currentStep > 0" class="btn btn-secondary" @click="prevStep">
        {{ $t('devis.previous') }}
      </button>
      <button v-if="currentStep < etapes.length - 1" class="btn btn-primary" @click="nextStep">
        {{ $t('devis.next') }} &rarr;
      </button>
      <button
        v-else
        class="btn btn-success"
        @click="saveDevis"
      >
        {{ isEdit ? $t('devis.update') : $t('devis.create') }}
      </button>
    </div>

    <!-- Selecteur de langue (layout global) -->
    <div class="lang-switcher">
      <button :class="{ active: locale === 'fr' }" @click="locale = 'fr'">FR</button>
      <button :class="{ active: locale === 'ar' }" @click="locale = 'ar'">AR</button>
    </div>
  </div>
</template>

<style scoped>
.nouveau-devis { padding: 2rem; max-width: 1200px; margin: 0 auto; position: relative; }
.nouveau-devis__title { font-family: 'Playfair Display', serif; color: var(--color-navy); margin-bottom: 2rem; }
.steps { display: flex; gap: 0.5rem; margin-bottom: 2rem; flex-wrap: wrap; }
.step-btn { display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; border: 2px solid #ddd; border-radius: 30px; background: white; cursor: pointer; transition: all 0.2s; }
.step-btn.active { border-color: var(--color-gold); background: var(--color-cream); }
.step-btn.completed { border-color: var(--color-navy); background: var(--color-navy); color: white; }
.step-number { display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 24px; border-radius: 50%; background: #eee; font-weight: 700; font-size: 0.8rem; }
.step-btn.active .step-number { background: var(--color-gold); color: white; }
.step-btn.completed .step-number { background: white; color: var(--color-navy); }
.step-content { background: var(--color-white); padding: 2rem; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.06); min-height: 400px; margin-bottom: 1.5rem; }
.step-nav { display: flex; justify-content: space-between; gap: 1rem; margin-top: 1rem; }

/* Language switcher */
.lang-switcher { position: fixed; top: 1rem; right: 1rem; z-index: 100; display: flex; gap: 0.25rem; }
.lang-switcher button { padding: 0.3rem 0.6rem; border: 1px solid #ccc; border-radius: 4px; background: white; cursor: pointer; font-weight: 600; font-size: 0.85rem; }
.lang-switcher button.active { border-color: var(--color-gold); background: var(--color-cream); color: var(--color-navy); }
</style>
