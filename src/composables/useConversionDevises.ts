import { ref, watch, type Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

/**
 * Convertit un montant d'une devise vers DZD pour affichage indicatif en appelant le backend Rust.
 * Retourne une ref réactive contenant la chaîne décimale calculée par le backend.
 */
export function useConversionDevises(
  montant: Ref<string>,
  deviseSource: Ref<string>,
  tauxSarDzd: Ref<string>,
  tauxUsdDzd: Ref<string>,
  tauxEurDzd: Ref<string>
) {
  const montantDzd = ref('0.00');

  const updateConversion = async () => {
    try {
      const res = await invoke<string>('calculer_conversion_backend', {
        montant: montant.value || '0',
        deviseSource: deviseSource.value,
        tauxSarDzd: tauxSarDzd.value || '1.0',
        tauxUsdDzd: tauxUsdDzd.value || '1.0',
        tauxEurDzd: tauxEurDzd.value || '1.0',
      });
      montantDzd.value = Number(res).toFixed(2);
    } catch (e) {
      console.error('Erreur de conversion backend:', e);
      montantDzd.value = '0.00';
    }
  };

  watch(
    [montant, deviseSource, tauxSarDzd, tauxUsdDzd, tauxEurDzd],
    () => {
      updateConversion();
    },
    { immediate: true }
  );

  return { montantDzd };
}
