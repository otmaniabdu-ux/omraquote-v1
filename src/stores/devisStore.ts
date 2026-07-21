import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Devis, DevisCreate, DevisUpdate, AlerteDevis, AlertePassager } from '@/types/devis.types';

export const useDevisStore = defineStore('devis', () => {
  const devisList = ref<Devis[]>([]);
  const currentDevis = ref<Devis | null>(null);
  const alertes = ref<AlerteDevis[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function loadDevis() {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Devis[]>('list_devis');
      devisList.value = result;
      // Charger aussi les alertes
      await loadAlertes();
    } catch (e) {
      error.value = String(e);
      console.error('Erreur chargement devis:', e);
    } finally {
      loading.value = false;
    }
  }

  async function loadAlertes() {
    try {
      const result = await invoke<AlerteDevis[]>('get_alertes_tous_devis');
      alertes.value = result;
    } catch (e) {
      console.error('Erreur chargement alertes:', e);
    }
  }

  async function getDevisById(id: number) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Devis>('get_devis_by_id', { id });
      currentDevis.value = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function createDevis(data: DevisCreate) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Devis>('create_devis', { devisData: data });
      devisList.value.unshift(result);
      currentDevis.value = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateDevis(id: number, data: DevisUpdate) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Devis>('update_devis', { id, updateData: data });
      const idx = devisList.value.findIndex(d => d.id === id);
      if (idx !== -1) devisList.value[idx] = result;
      if (currentDevis.value?.id === id) currentDevis.value = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteDevis(id: number) {
    loading.value = true;
    error.value = null;
    try {
      await invoke('delete_devis', { id });
      devisList.value = devisList.value.filter(d => d.id !== id);
      if (currentDevis.value?.id === id) currentDevis.value = null;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function recalculerTotaux(id: number) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Devis>('calculate_totals', { devisId: id });
      const idx = devisList.value.findIndex(d => d.id === id);
      if (idx !== -1) devisList.value[idx] = result;
      if (currentDevis.value?.id === id) currentDevis.value = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function getAlertesDevis(devisId: number) {
    try {
      return await invoke<AlertePassager[]>('get_alertes_devis', { devisId });
    } catch (e) {
      console.error('Erreur chargement alertes passagers:', e);
      return [];
    }
  }

  function getAlerteForDevis(devisId: number): boolean {
    return alertes.value.find(a => a.devis_id === devisId)?.alerte || false;
  }

  return {
    devisList,
    currentDevis,
    alertes,
    loading,
    error,
    loadDevis,
    getDevisById,
    createDevis,
    updateDevis,
    deleteDevis,
    recalculerTotaux,
    getAlertesDevis,
    getAlerteForDevis,
    loadAlertes,
  };
});