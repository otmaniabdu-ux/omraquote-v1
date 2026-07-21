# Détail génération PDF (Typst) — OmraVIP Quotes

> Complément à la **section 7** du prompt maître (`PROMPT_AGENT_OmraVIP.md`).
> À lire intégralement par le sous-agent `pdf-generator` avant d'attaquer la **Phase 6**,
> et par `qa-financier` comme référence d'audit pour tout contrôle d'étanchéité
> client/interne portant sur un export PDF.

---

## 1. Intégration Rust — `typst-as-lib` plutôt que le trait `World` brut

La crate `typst` brute exige d'implémenter soi-même le trait `World` (résolution des
polices, des fichiers, de la date du jour, etc.) — plusieurs centaines de lignes de
boilerplate pour un besoin qui reste au fond « injecter des données dans un template et
sortir un PDF ».

**Recommandation** : utiliser la crate wrapper **`typst-as-lib`**, conçue précisément pour
cet usage (« utiliser Typst comme moteur de templating »). Elle fournit :
- un `TypstEngineBuilder` où les polices s'ajoutent directement (`.fonts([...])`) ;
- un mécanisme de résolution de fichiers pour les assets (logo, gabarits partagés) ;
- des traits de conversion entre types Rust et types Typst.

Se rabattre sur l'implémentation manuelle du trait `World` uniquement si `typst-as-lib`
présente une limitation bloquante en pratique. Les noms exacts des méthodes du builder
doivent être vérifiés sur `docs.rs/typst-as-lib` au moment de coder — l'API de cette
crate évolue — mais l'architecture décrite dans ce document reste valable quelle que soit
la version rencontrée.

---

## 2. Séparation client/interne — le verrou par absence de données

Le prompt maître impose deux templates séparés (jamais un flag booléen). Ce document
ajoute un cran de sécurité **plus strict qu'une séparation de fichiers** :

> **Règle absolue** : `cout_net_total` et `marge_montant_total` ne doivent jamais être
> injectés dans les données compilées pour la variante client — pas seulement
> « ne pas être affichés ». La donnée absente ne peut pas fuiter, même par erreur de
> template.

```rust
fn construire_inputs_client(devis: &DevisComplet, totaux: &TotauxDevis) -> TypstDict {
    // Ne contient QUE prix_vente_total.
    // cout_net_total et marge_montant_total ne sont PAS présents dans ce dictionnaire,
    // même si l'appelant les a sous la main — l'omission est volontaire et systématique.
}

fn construire_inputs_interne(devis: &DevisComplet, totaux: &TotauxDevis) -> TypstDict {
    // Contient tout : cout_net_total, marge_montant_total, prix_vente_total.
}
```

Deux fonctions Rust distinctes, deux fichiers `.typ` distincts (`devis_client.typ` /
`devis_interne.typ`), qui importent tous deux un `devis_commun.typ` partagé pour
l'en-tête, le pied de page et la charte graphique — mais **jamais** la ligne financière,
qui reste propre à chaque fichier.

C'est ce niveau de séparation que `qa-financier` doit vérifier en priorité : pas
seulement « la marge n'est pas affichée à l'écran », mais « la marge n'existe même pas
dans les données compilées pour cette variante » (voir checklist section 8).

---

## 3. Injection des données

`sys.inputs`, construit programmatiquement (et non via la ligne de commande), accepte un
dictionnaire de valeurs Typst quelconques — pas seulement des chaînes comme le ferait un
`--input` en CLI. On peut donc passer directement une structure imbriquée (passagers,
segments de vol, hébergements...) sans passer par un fichier JSON intermédiaire sur
disque.

```typst
// devis_client.typ
#import "devis_commun.typ": entete, pied_de_page, couleurs

#let devis = sys.inputs.devis
#let totaux = sys.inputs.totaux  // ne contient que prix_vente_total dans cette variante

#entete(devis.numero, devis.client)
#for segment in devis.segments_vol [
  #segment.origine → #segment.destination — #segment.prix_vente
]
```

`DevisComplet` et `TotauxDevis` dérivent déjà `Serialize` pour le pont IPC Tauri (voir
`BACKEND_RUST_DETAIL.md`). La conversion vers un `Dict` Typst doit réutiliser cette même
sérialisation plutôt que dupliquer la logique de mapping ailleurs — une seule source de
vérité pour la forme des données, à ceci près que `construire_inputs_client` retire
explicitement les champs financiers sensibles avant construction du dictionnaire (voir
section 2).

---

## 4. Polices embarquées

```rust
.fonts([
    include_bytes!("../../assets/fonts/PlayfairDisplay-Regular.ttf").as_slice(),
    include_bytes!("../../assets/fonts/PlayfairDisplay-Bold.ttf").as_slice(),
    include_bytes!("../../assets/fonts/Lato-Regular.ttf").as_slice(),
    include_bytes!("../../assets/fonts/Amiri-Regular.ttf").as_slice(),
    include_bytes!("../../assets/fonts/Amiri-Bold.ttf").as_slice(),
])
```

`include_bytes!` embarque les polices dans le binaire à la compilation — aucune
dépendance aux polices installées sur le poste utilisateur, cohérent avec une application
100% locale. Même logique pour le logo : SVG converti en bytes embarqués plutôt que lu
depuis le disque à l'exécution.

---

## 5. Mélange RTL/LTR dans le même document

```typst
#set text(font: "Lato", lang: "fr")

= Devis Omra VIP

#set text(dir: rtl, font: "Amiri", lang: "ar")
[contenu arabe ici]

#set text(dir: ltr, font: "Lato", lang: "fr")
Suite du contenu en français...
```

La direction se fixe **par bloc**, jamais globalement. Un `#set text(dir: rtl)` en tête
de document RTL-ifierait aussi les tableaux de prix en français qui suivent — erreur
classique à surveiller en revue de template.

---

## 6. Performance — construire le moteur une seule fois

Les polices ne changent jamais entre deux générations de devis. Le moteur Typst (polices
+ résolveur de fichiers) doit être initialisé **une seule fois** pour toute la durée de
vie de l'application (ex. via `std::sync::OnceLock` ou `once_cell::sync::Lazy`) et
réutilisé à chaque appel de `generer_pdf_devis` — jamais reconstruit à chaque clic sur
« Générer le PDF ».

---

## 7. Date de génération

Pour l'horodatage affiché sur le PDF (« généré le... »), utiliser `chrono::Local::now()`
côté Rust et l'injecter explicitement dans les inputs — pas une date résolue en interne
par Typst — pour rester cohérent avec le fuseau horaire de l'agence et avec le reste de
l'application.

---

## 8. Checklist de test (à couvrir avant de clore la Phase 6)

- [ ] Compiler la variante client avec un `DevisComplet` contenant volontairement des
      montants de marge non nuls, puis **extraire le texte du PDF généré** (pas une
      simple inspection visuelle) et vérifier qu'aucune occurrence de `cout_net` ou
      `marge` n'apparaît, y compris dans les métadonnées du document
- [ ] Un devis avec segments de vol multiples (ex. Alger→Médine / Djeddah→Alger) pour
      vérifier que le gabarit gère une liste de longueur variable sans débordement de
      page
- [ ] Un nom d'hôtel en français adjacent à une mention en arabe, pour valider
      visuellement l'absence d'inversion de glyphes à la frontière RTL/LTR
- [ ] Le moteur Typst n'est initialisé qu'une seule fois sur une session de génération de
      plusieurs devis consécutifs (vérification de performance, pas seulement de
      correction)
- [ ] La variante interne affiche bien coût net, marge et prix de vente sur un devis de
      test, avec les 3 valeurs cohérentes entre elles (`prix_vente = cout_net + marge`)
