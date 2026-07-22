// Types correspondant aux modèles Rust (sérialisés en JSON)
export interface Client {
  id?: number;
  code_client: string;
  raison_sociale?: string;
  nom_contact?: string;
  telephone?: string;
  email?: string;
  adresse?: string;
  pays?: string;
  type_client: 'particulier' | 'agence';
  remarques?: string;
  created_at: string; // date ISO
  updated_at: string;
}

export interface Devis {
  id?: number;
  numero_devis: string;
  client_id: number;
  date_creation: string; // ISO date
  date_depart: string;
  date_retour: string;
  type_visa: 'omra_standard' | 'touristique' | 'hadj';
  assurance_medicale: boolean;
  devise_achat: string;
  taux_sar_dzd: string; // Decimal en string
  taux_usd_dzd: string;
  taux_eur_dzd: string;
  marge_type: 'pourcentage' | 'montant_fixe';
  marge_valeur: string;
  cout_net_total?: string;
  montant_marge?: string;
  prix_vente_total?: string;
  statut: 'brouillon' | 'finalise' | 'envoye' | 'accepte' | 'perdu';
  remise?: string;
  notes_internes?: string;
  updated_at?: string;
}

export interface Passager {
  id?: number;
  devis_id: number;
  categorie: 'adulte' | 'enfant_avec_lit' | 'enfant_sans_lit' | 'bebe';
  nom_complet: string;
  date_naissance: string;
  nationalite?: string;
  numero_passeport?: string;
  date_expiration_passeport?: string;
  lieu_delivrance?: string;
  remarques?: string;
}

export interface SegmentVol {
  id?: number;
  devis_id: number;
  ordre: number;
  compagnie: string;
  numero_vol?: string;
  classe: 'economique' | 'affaires' | 'premiere';
  date_vol: string;
  aeroport_depart: string;
  aeroport_arrivee: string;
  heure_depart?: string;
  heure_arrivee?: string;
  prix_adulte: string;
  prix_enfant: string;
  prix_bebe: string;
  devise_prix: string;
  remarques?: string;
}

export interface Hebergement {
  id?: number;
  devis_id: number;
  ville: 'Makkah' | 'Medine';
  nom_hotel: string;
  type_chambre: 'single' | 'double' | 'triple' | 'quadruple';
  formule_repas?: 'petit_dejeuner' | 'demi_pension' | 'pension_complete';
  vue?: 'Kaaba' | 'Haram' | 'City';
  date_checkin: string;
  date_checkout: string;
  nb_nuitees?: number;
  prix_par_nuit: string;
  devise_prix: string;
  taxes_incluses: boolean;
  remarques?: string;
}

export interface Transfert {
  id?: number;
  devis_id: number;
  type_transfert: 'obligatoire' | 'optionnel';
  trajet: string;
  type_vehicule: 'GMC_Yukon' | 'Mercedes_Classe_E' | 'Bus_VIP_prive';
  date_transfert?: string;
  heure_transfert?: string;
  nombre_vehicules: number;
  prix_unitaire: string;
  devise_prix: string;
  remarques?: string;
}

export interface PrestationVip {
  id?: number;
  devis_id: number;
  type_prestation: 'ziyarat' | 'lounge' | 'fast_track' | 'bagages' | 'zamzam' | 'autre';
  description: string;
  prix_unitaire: string;
  quantite: number;
  devise_prix: string;
  remarques?: string;
}

export interface AlerteDevis {
  devis_id: number;
  numero_devis: string;
  alerte: boolean;
}

export interface AlertePassager {
  passager_id: number;
  nom: string;
  date_expiration?: string;
  alerte: boolean;
}

// ---------- Créations / mises à jour ----------

export interface DevisCreate {
  client_id: number;
  date_depart: string; // YYYY-MM-DD
  date_retour: string; // YYYY-MM-DD
  type_visa: 'omra_standard' | 'touristique' | 'hadj';
  assurance_medicale: boolean;
  devise_achat: string; // SAR, USD, EUR, DZD
  taux_sar_dzd: string;
  taux_usd_dzd: string;
  taux_eur_dzd: string;
  marge_type: 'pourcentage' | 'montant_fixe';
  marge_valeur: string; // chaîne décimale
  statut: 'brouillon';
  remise?: string;
  notes_internes?: string;
}

export interface DevisUpdate {
  date_depart?: string;
  date_retour?: string;
  type_visa?: 'omra_standard' | 'touristique' | 'hadj';
  assurance_medicale?: boolean;
  devise_achat?: string;
  taux_sar_dzd?: string;
  taux_usd_dzd?: string;
  taux_eur_dzd?: string;
  marge_type?: 'pourcentage' | 'montant_fixe';
  marge_valeur?: string;
  statut?: 'brouillon' | 'finalise' | 'envoye' | 'accepte' | 'perdu';
  remise?: string;
  notes_internes?: string;
}

export interface ClientCreate {
  code_client: string;
  raison_sociale?: string;
  nom_contact: string;
  telephone: string;
  email?: string;
  adresse?: string;
  pays?: string;
  type_client: 'particulier' | 'agence';
  remarques?: string;
}

// ---------- Totaux renvoyés par le backend ----------

export interface TotauxDevis {
  cout_net_total: string;
  marge_montant_total: string;
  prix_vente_total: string;
}

// ---------- Statistiques marge (dashboard interne) ----------

export interface StatsMarge {
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