import { computed } from 'vue';

export function useValidationPasseport(dateExpiration: string | undefined, dateRetour: string) {
  const alerte = computed(() => {
    if (!dateExpiration) return false;
    const exp = new Date(dateExpiration);
    const retour = new Date(dateRetour);
    // Ajouter 6 mois
    const seuil = new Date(retour);
    seuil.setMonth(retour.getMonth() + 6);
    return exp < seuil;
  });
  return { alerte };
}