<script setup lang="ts">
import { ref } from 'vue';
import type { Devis, Passager, SegmentVol, Hebergement, Transfert, PrestationVip } from '@/types/devis.types';

const props = defineProps<{
  devis: Devis | null;
  passagers: Passager[];
  segments: SegmentVol[];
  hebergements: Hebergement[];
  transferts: Transfert[];
  prestations: PrestationVip[];
  totaux: { cout_net_total: string; marge_montant_total: string; prix_vente_total: string } | null;
}>();

const emit = defineEmits(['next', 'generate-pdf']);

// Bascule Vue client / Vue interne
const viewMode = ref<'client' | 'internal'>('client');

function toggleView() {
  viewMode.value = viewMode.value === 'client' ? 'internal' : 'client';
}

/** Formate une chaine decimal en valeur lisible (separateur de milliers, 2 decimales).
 * NE convertit PAS via parseFloat — garde la precision originale du backend. */
function formatMontant(chaine: string): string {
  if (!chaine || chaine === '0') return '0,00';
  const parts = chaine.trim().split('.');
  const entiere = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ' ');
  const decimales = parts.length > 1 ? '.' + parts[1] : '.00';
  return `${entiere}${decimales}`;
}

function generatePdf() {
  emit('generate-pdf');
}
</script>

<template>
  <div class="recapitulatif-devis" dir="ltr">
    <h2>{{ $t('wizard.recapitulatif.title') }}</h2>

    <!-- Bascule Vue client / Vue interne -->
    <div class="view-toggle">
      <button
        :class="{ active: viewMode === 'client' }"
        @click="toggleView"
      >
        {{ $t('devis.view_client') }}
      </button>
      <button
        :class="{ active: viewMode === 'internal' }"
        @click="toggleView"
      >
        {{ $t('devis.view_internal') }}
      </button>

      <!-- Avertissement visuel pour mode interne -->
      <div v-if="viewMode === 'internal'" class="internal-warning">
        Usage interne — ne pas inclure dans le PDF client
      </div>
    </div>

    <!-- Info devis principal -->
    <div class="devis-info" :class="{ 'is-internal': viewMode === 'internal' }">
      <div class="info-row">
        <strong>{{ $t('devis.number') }}:</strong>
        <span>{{ devis?.numero_devis || '—' }}</span>
      </div>
      <div class="info-row">
        <strong>{{ $t('devis.status') }}:</strong>
        <span>{{ devis?.statut || 'brouillon' }}</span>
      </div>
      <div class="info-row">
        <strong>{{ $t('wizard.financier.title') }}:</strong>
        <span>{{ devis?.date_depart ? `${devis.date_depart} → ${devis.date_retour}` : '—' }}</span>
      </div>
    </div>

    <!-- Vue client: uniquement prix_vente_total -->
    <template v-if="viewMode === 'client'">
      <div class="total-section total--sale">
        <span class="total-label">{{ $t('devis.total') }}</span>
        <span class="total-value">{{ totaux?.prix_vente_total ? formatMontant(totaux.prix_vente_total) : '0,00' }} {{ devis?.devise_achat || 'DZD' }}</span>
      </div>

      <!-- Passagers -->
      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.passagers') }} ({{ passagers.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Nom</th><th>Categorie</th><th>Date naissance</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in passagers" :key="i">
              <td>{{ p.nom_complet }}</td>
              <td>{{ p.categorie }}</td>
              <td>{{ p.date_naissance || '—' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Vols -->
      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.vols') }} ({{ segments.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Compagnie</th><th>Vol</th><th>Dates</th><th>A/R</th><th>Prix adulte</th></tr></thead>
          <tbody>
            <tr v-for="(s, i) in segments" :key="i">
              <td>{{ s.compagnie }}</td>
              <td>{{ s.numero_vol || '—' }}</td>
              <td>{{ s.date_vol }}</td>
              <td>{{ s.aeroport_depart }} → {{ s.aeroport_arrivee }}</td>
              <td>{{ formatMontant(s.prix_adulte) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Hebergements -->
      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.hebergements') }} ({{ hebergements.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Hôtel</th><th>Ville</th><th>Type chambre</th><th>Nuitées</th><th>Prix/nuit</th></tr></thead>
          <tbody>
            <tr v-for="(h, i) in hebergements" :key="i">
              <td>{{ h.nom_hotel }}</td>
              <td>{{ h.ville }}</td>
              <td>{{ h.type_chambre }}</td>
              <td>{{ h.nb_nuitees ?? '—' }}</td>
              <td>{{ formatMontant(h.prix_par_nuit) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Transferts -->
      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.transferts') }} ({{ transferts.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Trajet</th><th>Véhicule</th><th>Prix unitaire</th></tr></thead>
          <tbody>
            <tr v-for="(t, i) in transferts" :key="i">
              <td>{{ t.trajet }}</td>
              <td>{{ t.type_vehicule }}</td>
              <td>{{ formatMontant(t.prix_unitaire) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Prestations VIP -->
      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.prestations') }} ({{ prestations.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Type</th><th>Description</th><th>Qté</th><th>Prix unitaire</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in prestations" :key="i">
              <td>{{ p.type_prestation }}</td>
              <td>{{ p.description }}</td>
              <td>{{ p.quantite }}</td>
              <td>{{ formatMontant(p.prix_unitaire) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>

    <!-- Vue interne: cout_net_total, marge_montant_total, prix_vente_total -->
    <template v-else-if="viewMode === 'internal' && totaux">
      <!-- Totals internes (UNIQUEMENT valeurs du backend) -->
      <div class="internal-totals">
        <div class="total-card total--cost">
          <span class="card-label">{{ $t('marge.cost_total') }}</span>
          <span class="card-value">{{ formatMontant(totaux.cout_net_total) }} DZD</span>
        </div>
        <div class="total-card total--margin">
          <span class="card-label">{{ $t('marge.margin_total') }}</span>
          <span class="card-value">{{ formatMontant(totaux.marge_montant_total) }} DZD</span>
        </div>
        <div class="total-card total--sale">
          <span class="card-label">{{ $t('devis.total') }}</span>
          <span class="card-value">{{ formatMontant(totaux.prix_vente_total) }} {{ devis?.devise_achat || 'DZD' }}</span>
        </div>
      </div>

      <!-- Sections identiques mais sans masquage -->
      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.passagers') }} ({{ passagers.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Nom</th><th>Categorie</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in passagers" :key="i">
              <td>{{ p.nom_complet }}</td>
              <td>{{ p.categorie }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.vols') }} ({{ segments.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Compagnie</th><th>Date</th><th>A/R</th></tr></thead>
          <tbody>
            <tr v-for="(s, i) in segments" :key="'seg-'+i">
              <td>{{ s.compagnie }}</td>
              <td>{{ s.date_vol }}</td>
              <td>{{ s.aeroport_depart }} → {{ s.aeroport_arrivee }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.hebergements') }} ({{ hebergements.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Hôtel</th><th>Ville</th></tr></thead>
          <tbody>
            <tr v-for="(h, i) in hebergements" :key="'heb-'+i">
              <td>{{ h.nom_hotel }}</td>
              <td>{{ h.ville }} ({{ h.type_chambre }})</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.transferts') }} ({{ transferts.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Trajet</th><th>Véhicule</th></tr></thead>
          <tbody>
            <tr v-for="(t, i) in transferts" :key="'tra-'+i">
              <td>{{ t.trajet }}</td>
              <td>{{ t.type_vehicule }} (x{{ t.nombre_vehicules }})</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="recap-section">
        <h3>{{ $t('wizard.recapitulatif.prestations') }} ({{ prestations.length }})</h3>
        <table class="recap-table">
          <thead><tr><th>Type</th><th>Description</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in prestations" :key="'pre-'+i">
              <td>{{ p.type_prestation }}</td>
              <td>{{ p.description }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>

    <!-- Actions -->
    <div class="actions">
      <button class="btn btn-primary" @click="generatePdf">
        {{ $t('devis.generate_pdf') }}
      </button>
      <button class="btn btn-success" @click="$emit('next')">
        {{ $t('devis.validate') }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.recapitulatif-devis { max-width: 900px; margin: 0 auto; }
h2 { font-family: 'Playfair Display', serif; color: var(--color-navy); }
.view-toggle { display: flex; gap: 0.5rem; margin-bottom: 1rem; }
.view-toggle button { padding: 0.4rem 1rem; border: 2px solid #ccc; border-radius: 20px; background: white; cursor: pointer; font-weight: 600; }
.view-toggle button.active { border-color: var(--color-gold); background: var(--color-cream); color: var(--color-navy); }
.internal-warning { margin-top: 0.5rem; padding: 0.3rem 0.8rem; background: var(--color-red); color: white; border-radius: 4px; font-size: 0.8rem; text-align: center; }
.devis-info { background: var(--color-cream); padding: 1rem; border-radius: 8px; margin-bottom: 1rem; }
.info-row { display: flex; gap: 0.5rem; margin-bottom: 0.3rem; font-size: 0.9rem; }
.info-row strong { min-width: 120px; color: var(--color-navy); }
.internal-totals { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; margin-bottom: 1.5rem; }
.total-card { padding: 1rem; border-radius: 8px; text-align: center; background: white; box-shadow: var(--shadow-sm); }
.card-label { display: block; font-size: 0.8rem; color: #666; margin-bottom: 0.3rem; }
.card-value { display: block; font-size: 1.4rem; font-weight: 700; color: var(--color-navy); }
.total--cost { border-top: 3px solid var(--color-blue-royal); }
.total--margin { border-top: 3px solid var(--color-gold); }
.total--sale { border-top: 3px solid #2e7d32; }
.recap-section { margin-bottom: 1rem; padding: 0.75rem; background: white; border-radius: 8px; box-shadow: var(--shadow-sm); }
.recap-section h3 { margin-top: 0; font-size: 1rem; color: var(--color-navy); }
.recap-table { width: 100%; border-collapse: collapse; font-size: 0.85rem; }
.recap-table th { text-align: left; padding: 0.4rem 0.6rem; background: var(--color-cream); color: var(--color-navy); font-weight: 600; }
.recap-table td { padding: 0.35rem 0.6rem; border-bottom: 1px solid #eee; }
.actions { display: flex; gap: 1rem; justify-content: center; margin-top: 2rem; }
</style>
