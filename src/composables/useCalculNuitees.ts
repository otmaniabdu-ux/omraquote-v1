import { computed } from 'vue';

export function useCalculNuitees(dateCheckin: string, dateCheckout: string) {
  const nuitees = computed(() => {
    if (!dateCheckin || !dateCheckout) return 0;
    const d1 = new Date(dateCheckin);
    const d2 = new Date(dateCheckout);
    const diff = (d2.getTime() - d1.getTime()) / (1000 * 60 * 60 * 24);
    return Math.max(0, Math.floor(diff));
  });
  return { nuitees };
}