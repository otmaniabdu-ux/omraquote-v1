import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Client, ClientCreate } from '@/types/devis.types';

export const useClientStore = defineStore('clients', () => {
  const clients = ref<Client[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function loadClients() {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Client[]>('list_clients');
      clients.value = result;
    } catch (e) {
      error.value = String(e);
      console.error('Erreur chargement clients:', e);
    } finally {
      loading.value = false;
    }
  }

  async function createClient(data: Omit<ClientCreate, 'code_client'>) {
    loading.value = true;
    error.value = null;
    try {
      // Générer le code client
      const code = await invoke<string>('generate_client_code');
      const fullData: ClientCreate = { ...data, code_client: code };
      const result = await invoke<Client>('create_client', { clientData: fullData });
      clients.value.unshift(result);
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateClient(id: number, data: Partial<Client>) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Client>('update_client', { id, updateData: data });
      const idx = clients.value.findIndex(c => c.id === id);
      if (idx !== -1) clients.value[idx] = result;
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteClient(id: number) {
    loading.value = true;
    error.value = null;
    try {
      await invoke('delete_client', { id });
      clients.value = clients.value.filter(c => c.id !== id);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  return { clients, loading, error, loadClients, createClient, updateClient, deleteClient };
});