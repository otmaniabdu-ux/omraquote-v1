import { computed, type Ref } from 'vue';

/**
 * Convertit un montant d'une devise vers DZD pour affichage indicatif uniquement.
 * Retourne toujours une chaine decimale — jamais de number.
 * Le montant CANONIQUE est calcule cote Backend au moment de la sauvegarde.
 */
export function useConversionDevises(
  montant: Ref<string>,
  deviseSource: Ref<string>,
  tauxSarDzd: Ref<string>,
  tauxUsdDzd: Ref<string>,
  tauxEurDzd: Ref<string>
) {
  const montantDzd = computed(() => {
    // On parse le montant et les taux SEULEMENT pour un affichage indicatif.
    // Les valeurs canoniques (montant_dzd) sont calculees cote Rust.
    const val = Number(montant.value) || 0;
    let taux = 1;
    switch (deviseSource.value) {
      case 'SAR': taux = Number(tauxSarDzd.value) || 1; break;
      case 'USD': taux = Number(tauxUsdDzd.value) || 1; break;
      case 'EUR': taux = Number(tauxEurDzd.value) || 1; break;
      case 'DZD': taux = 1; break;
      default:    taux = 1;
    }
    return (val * taux).toFixed(2);
  });

  return { montantDzd };
}
