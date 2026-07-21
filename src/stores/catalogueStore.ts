import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface HotelCatalogue {
  id?: number;
  nom_hotel: string;
  ville: 'Makkah' | 'Medine';
  categorie?: string;
  adresse?: string;
  contact?: string;
  site_web?: string;
  remarques?: string;
  actif: boolean;
}

export interface CompagnieCatalogue {
  id?: number;
  code_iata?: string;
  nom_compagnie: string;
  pays?: string;
  site_web?: string;
  actif: boolean;
}

export const useCatalogueStore = defineStore('catalogue', () => {
  const hotels = ref<HotelCatalogue[]>([]);
  const compagnies = ref<CompagnieCatalogue[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // --- Hôtels ---
  async function loadHotels(actifSeulement: boolean = true) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<HotelCatalogue[]>('list_hotels', { actifSeulement });
      hotels.value = result;
    } catch (e) {
      error.value = String(e);
      console.error('Erreur chargement hôtels:', e);
    } finally {
      loading.value = false;
    }
  }

  async function createHotel(data: Omit<HotelCatalogue, 'id' | 'actif'> & { actif?: boolean }) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<HotelCatalogue>('create_hotel', { hotelData: data });
      hotels.value.unshift(result);
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateHotel(id: number, data: Partial<HotelCatalogue>) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<HotelCatalogue>('update_hotel', { id, updateData: data });
      const idx = hotels.value.findIndex(h => h.id === id);
      if (idx !== -1) hotels.value[idx] = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteHotel(id: number) {
    loading.value = true;
    error.value = null;
    try {
      await invoke('delete_hotel', { id });
      hotels.value = hotels.value.filter(h => h.id !== id);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // --- Compagnies ---
  async function loadCompagnies(actifSeulement: boolean = true) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<CompagnieCatalogue[]>('list_compagnies', { actifSeulement });
      compagnies.value = result;
    } catch (e) {
      error.value = String(e);
      console.error('Erreur chargement compagnies:', e);
    } finally {
      loading.value = false;
    }
  }

  async function createCompagnie(data: Omit<CompagnieCatalogue, 'id' | 'actif'> & { actif?: boolean }) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<CompagnieCatalogue>('create_compagnie', { compagnieData: data });
      compagnies.value.unshift(result);
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateCompagnie(id: number, data: Partial<CompagnieCatalogue>) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<CompagnieCatalogue>('update_compagnie', { id, updateData: data });
      const idx = compagnies.value.findIndex(c => c.id === id);
      if (idx !== -1) compagnies.value[idx] = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteCompagnie(id: number) {
    loading.value = true;
    error.value = null;
    try {
      await invoke('delete_compagnie', { id });
      compagnies.value = compagnies.value.filter(c => c.id !== id);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  return {
    hotels,
    compagnies,
    loading,
    error,
    loadHotels,
    createHotel,
    updateHotel,
    deleteHotel,
    loadCompagnies,
    createCompagnie,
    updateCompagnie,
    deleteCompagnie,
  };
});