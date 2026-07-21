<template>
  <div class="marge-dashboard" dir="ltr">
    <!-- En-tete avec avertissement interne -->
    <div class="marge-dashboard__header">
      <h1 class="marge-dashboard__title">{{ $t('marge.title') }}</h1>
      <div class="marge-dashboard__warning">
        {{ $t('marge.internal_only') }}
      </div>
    </div>

    <!-- Filtres -->
    <div class="marge-dashboard__filters">
      <div class="filter-group">
        <label>{{ $t('marge.period') }}</label>
        <div class="date-range">
          <input type="date" v-model="dateDebut" />
          <span aria-hidden="true">&rarr;</span>
          <input type="date" v-model="dateFin" />
        </div>
      </div>
      <div class="filter-group">
        <label>{{ $t('marge.presets') }}</label>
        <select v-model="preset" @change="applyPreset">
          <option value="month">{{ $t('marge.preset_month') }}</option>
          <option value="quarter">{{ $t('marge.preset_quarter') }}</option>
          <option value="year">{{ $t('marge.preset_year') }}</option>
          <option value="custom">{{ $t('marge.preset_custom') }}</option>
        </select>
      </div>
      <button class="btn btn-primary" @click="loadData">{{ $t('marge.refresh') }}</button>
    </div>

    <!-- Chargement / Erreur -->
    <div v-if="loading" class="loading">{{ $t('marge.loading') }}</div>
    <div v-else-if="error" class="error">{{ error }}</div>

    <!-- Cartes de synthese -->
    <div v-else class="marge-dashboard__cards">
      <div class="card card--total">
        <span class="card__label">{{ $t('marge.total_devis') }}</span>
        <span class="card__value">{{ stats.total_devis }}</span>
      </div>
      <div class="card card--cost">
        <span class="card__label">{{ $t('marge.cost_total') }}</span>
        <span class="card__value">{{ formatMontant(stats.total_cout_net) }}</span>
      </div>
      <div class="card card--margin">
        <span class="card__label">{{ $t('marge.margin_total') }}</span>
        <span class="card__value">{{ formatMontant(stats.total_marge) }}</span>
      </div>
      <div class="card card--margin-pct">
        <span class="card__label">{{ $t('margin.average_margin') }}</span>
        <span class="card__value">{{ formatPourcentage(stats.marge_moyenne_pourcentage) }}</span>
      </div>
      <div class="card card--revenue">
        <span class="card__label">{{ $t('marge.revenue') }}</span>
        <span class="card__value">{{ formatMontant(stats.total_prix_vente) }}</span>
      </div>
    </div>

    <!-- Meilleur devis -->
    <div v-if="stats.meilleur_devis" class="marge-dashboard__best">
      <h3>{{ $t('marge.best_devis') }}</h3>
      <div class="best-card">
        <span class="best-number">{{ stats.meilleur_devis.numero_devis }}</span>
        <span class="best-margin">{{ formatMontant(stats.meilleur_devis.marge) }}</span>
        <span class="best-pct">{{ formatPourcentage(stats.meilleur_devis.marge_pourcentage) }}</span>
      </div>
    </div>

    <!-- Graphique des marges par mois -->
    <div class="marge-dashboard__chart">
      <h3>{{ $t('marge.margin_evolution') }}</h3>
      <div class="chart-container">
        <canvas ref="chartCanvas"></canvas>
      </div>
    </div>

    <!-- Top clients -->
    <div class="marge-dashboard__top-clients">
      <h3>{{ $t('marge.top_clients') }}</h3>
      <table class="top-table">
        <thead>
          <tr>
            <th>{{ $t('marge.client') }}</th>
            <th>{{ $t('marge.total_margin') }}</th>
            <th>{{ $t('marge.devis_count') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="client in topClients" :key="client.nom">
            <td>{{ client.nom }}</td>
            <td>{{ formatMontant(client.marge) }}</td>
            <td>{{ client.nb_devis }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Tableau detaille par mois -->
    <div class="marge-dashboard__monthly">
      <h3>{{ $t('marge.monthly_detail') }}</h3>
      <table class="monthly-table">
        <thead>
          <tr>
            <th>{{ $t('marge.month') }}</th>
            <th>{{ $t('marge.devis_nb') }}</th>
            <th>{{ $t('marge.net_cost') }}</th>
            <th>{{ $t('marge.margin') }}</th>
            <th>{{ $t('marge.ca') }}</th>
            <th>{{ $t('marge.margin_pct') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="mois in stats.par_mois" :key="mois.mois">
            <td>{{ mois.mois }}</td>
            <td>{{ mois.nb_devis }}</td>
            <td>{{ formatMontant(mois.cout_net_total) }}</td>
            <td>{{ formatMontant(mois.marge_total) }}</td>
            <td>{{ formatMontant(mois.prix_vente_total) }}</td>
            <td>{{ formatPourcentage(mois.marge_moyenne_pourcentage) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Chart, registerables } from 'chart.js';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

Chart.register(...registerables);

interface StatsMarge {
  total_devis: number;
  total_cout_net: string;
  total_marge: string;
  total_prix_vente: string;
  marge_moyenne_pourcentage: string;
  meilleur_devis: { numero_devis: string; marge: string; marge_pourcentage: string } | null;
  par_mois: Array<{
    mois: string;
    nb_devis: number;
    cout_net_total: string;
    marge_total: string;
    prix_vente_total: string;
    marge_moyenne_pourcentage: string;
  }>;
}

const dateDebut = ref('');
const dateFin = ref('');
const preset = ref('month');
const loading = ref(false);
const error = ref<string | null>(null);
const stats = ref<StatsMarge>({
  total_devis: 0,
  total_cout_net: '0',
  total_marge: '0',
  total_prix_vente: '0',
  marge_moyenne_pourcentage: '0',
  meilleur_devis: null,
  par_mois: [],
});
const topClients = ref<Array<{ nom: string; marge: string; nb_devis: number }>>([]);
const chartCanvas = ref<HTMLCanvasElement | null>(null);
let chartInstance: Chart | null = null;

/**
 * Formate une chaine decimal en valeur lisible avec separateur de milliers.
 * NE convertit PAS via parseFloat — garde la precision originale cote backend.
 */
function formatMontant(chaine: string): string {
  if (!chaine || chaine === '0') return '0,00 DZD';
  const parts = chaine.trim().split('.');
  const entiere = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ' '); // space separator
  const décimales = parts.length > 1 ? '.' + parts[1] : ',00';
  return `${entiere}${décimales} DZD`;
}

/**
 * Formate un pourcentage donne en chaine sans conversion numerique coté frontend.
 */
function formatPourcentage(chaine: string): string {
  if (!chaine || chaine === '0') return '0,00 %';
  const parts = chaine.trim().split('.');
  const entiere = parts[0];
  const décimales = parts.length > 1 ? '.' + parts[1] : '';
  return `${entiere}${décimales} %`;
}

function applyPreset() {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');

  switch (preset.value) {
    case 'month':
      dateDebut.value = `${year}-${month}-01`;
      dateFin.value = `${year}-${month}-${day}`;
      break;
    case 'quarter': {
      const currentMonth = now.getMonth() + 1;
      const quarterStartMonth = Math.floor((currentMonth - 1) / 3) * 3 + 1;
      const qStart = String(quarterStartMonth).padStart(2, '0');
      dateDebut.value = `${year}-${qStart}-01`;
      dateFin.value = `${year}-${month}-${day}`;
      break;
    }
    case 'year':
      dateDebut.value = `${year}-01-01`;
      dateFin.value = `${year}-${month}-${day}`;
      break;
    default:
      break;
  }
}

async function loadData() {
  if (!dateDebut.value || !dateFin.value) {
    error.value = t('marge.select_period');
    return;
  }

  loading.value = true;
  error.value = null;

  try {
    const statsResult = await invoke<StatsMarge>('get_statistiques', {
      dateDebut: dateDebut.value,
      dateFin: dateFin.value,
    });
    stats.value = statsResult;

    const clientsResult = await invoke<Array<[string, string, number]>>('get_top_clients', {
      dateDebut: dateDebut.value,
      dateFin: dateFin.value,
      limit: 10,
    });
    topClients.value = clientsResult.map(([nom, marge, nb]) => ({
      nom,
      marge,
      nb_devis: nb,
    }));

    await nextTick();
    updateChart();
  } catch (e) {
    error.value = String(e);
    console.error('Erreur chargement donnees marge:', e);
  } finally {
    loading.value = false;
  }
}

function updateChart() {
  if (!chartCanvas.value) return;

  if (chartInstance) {
    chartInstance.destroy();
    chartInstance = null;
  }

  const ctx = chartCanvas.value.getContext('2d');
  if (!ctx) return;

  // Chart.js requiert des nombres pour le rendu visuel — acceptable ici car ce n'est
  // PAS de la finance metier, juste de la visualisation graphique.
  const mois = stats.value.par_mois.map(m => m.mois);
  const marges = stats.value.par_mois.map(m => parseFloat(m.marge_total) || 0);
  const ca     = stats.value.par_mois.map(m => parseFloat(m.prix_vente_total) || 0);

  chartInstance = new Chart(ctx, {
    type: 'bar',
    data: {
      labels: mois,
      datasets: [
        {
          label: 'Marge (DZD)',
          data: marges,
          backgroundColor: 'rgba(196, 161, 82, 0.8)',
          borderColor: '#C4A152',
          borderWidth: 2,
          yAxisID: 'y',
        },
        {
          label: "Chiffre d'affaires (DZD)",
          data: ca,
          type: 'line',
          borderColor: '#0A1628',
          backgroundColor: 'rgba(10, 22, 40, 0.1)',
          borderWidth: 2,
          fill: true,
          yAxisID: 'y1',
        },
      ],
    },
    options: {
      responsive: true,
      maintainAspectRatio: true,
      plugins: {
        legend: { position: 'top' },
      },
      scales: {
        y: {
          type: 'linear',
          display: true,
          position: 'left',
        },
        y1: {
          type: 'linear',
          display: true,
          position: 'right',
          grid: { drawOnChartArea: false },
        },
      },
    },
  });
}

watch([dateDebut, dateFin], () => {
  if (dateDebut.value && dateFin.value) {
    loadData();
  }
});

onMounted(() => {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  dateDebut.value = `${year}-${month}-01`;
  dateFin.value = `${year}-${month}-${day}`;
  preset.value = 'month';
  loadData();
});
</script>

<style scoped>
.marge-dashboard {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.marge-dashboard__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 2px solid var(--color-red);
}

.marge-dashboard__title {
  font-family: 'Playfair Display', serif;
  color: var(--color-navy);
}

.marge-dashboard__warning {
  background: var(--color-red);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 600;
  font-size: 0.9rem;
  letter-spacing: 0.5px;
}

.marge-dashboard__filters {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;
  background: var(--color-cream);
  padding: 1rem 1.5rem;
  border-radius: var(--radius-md);
  margin-bottom: 2rem;
  align-items: flex-end;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.filter-group label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--color-navy);
}

.date-range {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.date-range input[type="date"] {
  padding: 0.4rem 0.6rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  background: white;
}

.marge-dashboard__cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.card {
  background: var(--color-white);
  padding: 1.2rem;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  border-top: 4px solid var(--color-gold);
  text-align: center;
}

.card__label {
  display: block;
  font-size: 0.8rem;
  color: #666;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.card__value {
  display: block;
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-navy);
  margin-top: 0.3rem;
}

.card--cost { border-top-color: var(--color-blue-royal); }
.card--margin { border-top-color: var(--color-gold); }
.card--margin-pct { border-top-color: var(--color-red); }
.card--revenue { border-top-color: #2e7d32; }

.marge-dashboard__best {
  background: linear-gradient(135deg, var(--color-cream), #faf8f5);
  padding: 1.5rem;
  border-radius: var(--radius-md);
  margin-bottom: 2rem;
  border: 1px solid var(--color-gold);
}

.best-card {
  display: flex;
  align-items: center;
  gap: 2rem;
  margin-top: 0.5rem;
  flex-wrap: wrap;
}

.best-number {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--color-navy);
}

.best-margin {
  font-size: 1.4rem;
  font-weight: 700;
  color: var(--color-gold);
}

.best-pct {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-red);
}

.marge-dashboard__chart {
  background: var(--color-white);
  padding: 1.5rem;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  margin-bottom: 2rem;
}

.chart-container {
  max-height: 400px;
  margin-top: 1rem;
}

.marge-dashboard__top-clients,
.marge-dashboard__monthly {
  background: var(--color-white);
  padding: 1.5rem;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  margin-bottom: 2rem;
}

.top-table,
.monthly-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1rem;
  font-size: 0.9rem;
}

.top-table th,
.monthly-table th {
  text-align: left;
  padding: 0.6rem 0.8rem;
  background: var(--color-cream);
  color: var(--color-navy);
  font-weight: 600;
}

.top-table td,
.monthly-table td {
  padding: 0.5rem 0.8rem;
  border-bottom: 1px solid #eee;
}

.top-table tr:hover,
.monthly-table tr:hover {
  background: #fafafa;
}

.loading, .error {
  text-align: center;
  padding: 3rem;
  font-size: 1.1rem;
}

.error {
  color: var(--color-red);
}
</style>
