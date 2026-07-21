// templates/devis_pdf.typ
// Template de devis bilingue pour OmraVIP Quotes

// ----- Paramètres -----
#let data = json("input.json")  // injecté depuis Rust
#let devis = data.devis
#let client = data.client
#let passagers = data.passagers
#let segments = data.segments
#let hebergements = data.hebergements
#let transferts = data.transferts
#let prestations = data.prestations
#let totaux = data.totaux

// ----- Polices (embarquées) -----
#let font-fr = ("Playfair Display", "Lato")
#let font-ar = "Amiri"

// ----- Couleurs -----
#let rouge = rgb("#CC1A1A")
#let nuit = rgb("#0A1628")
#let or = rgb("#C4A152")
#let blanc = rgb("#FFFFFF")

// ----- Mise en page -----
#set page(
  paper: "a4",
  margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm),
  fill: blanc,
)

#set text(font: font-fr, size: 11pt, lang: "fr")

// ----- En-tête (bilingue) -----
#block(
  width: 100%,
  fill: nuit,
  inset: 0.8cm,
)
[
  #align(center)[
    #text(fill: blanc, size: 18pt, weight: "bold")[#devis.nom_agence_fr]
    #text(fill: or, size: 14pt, font: font-ar, dir: rtl)[#devis.nom_agence_ar]
    #v(0.3cm)
    #text(fill: or, size: 12pt)[#devis.numero_devis]
  ]
]

#v(1cm)

// ----- Titre du document -----
#set text(size: 16pt, weight: "bold", fill: nuit)
#align(center)[
  #text(font: font-ar, dir: rtl, fill: nuit)[عرض سعر]
  #text(fill: or)[ | ]
  #text[Devis]
]

#v(0.5cm)

// ----- Informations client et séjour -----
#grid(
  columns: (1fr, 1fr),
  gutter: 1cm,
  [
    #set text(size: 10pt)
    #set text(weight: "bold")[Client : ]
    #set text(weight: "regular")[#client.nom_contact]
    #linebreak()
    #set text(weight: "bold")[Code : ]
    #set text(weight: "regular")[#client.code_client]
  ],
  [
    #set text(size: 10pt)
    #set text(weight: "bold")[Départ : ]
    #set text(weight: "regular")[#devis.date_depart]
    #linebreak()
    #set text(weight: "bold")[Retour : ]
    #set text(weight: "regular")[#devis.date_retour]
    #linebreak()
    #set text(weight: "bold")[Visa : ]
    #set text(weight: "regular")[#devis.type_visa]
  ]
)

#v(0.8cm)

// ----- Tableau des passagers -----
#set text(size: 10pt)
#table(
  columns: (auto, 1fr, auto, auto),
  fill: (_, row) => if row == 0 { nuit },
  stroke: 0.5pt,
  [#text(fill: blanc, weight: "bold")[N°]],
  [#text(fill: blanc, weight: "bold")[Nom]],
  [#text(fill: blanc, weight: "bold")[Catégorie]],
  [#text(fill: blanc, weight: "bold")[Passeport]],
  ..passagers.enumerate().map(((i, p)) => {
    (
      [#p.num],
      [#p.nom_complet],
      [#p.categorie],
      [#p.numero_passeport],
    )
  }).flatten()
)

#v(0.8cm)

// ----- Détails prestations (synthèse) -----
#heading(level: 2, fill: or)[Détail des prestations]
#set text(size: 9pt)

// Vols
#if segments.len() > 0 {
  heading(level: 3)[Vols]
  for s in segments {
    grid(
      columns: (auto, 1fr, auto),
      [
        #s.date_vol
      ],
      [
        #s.aeroport_depart ➔ #s.aeroport_arrivee
        (#s.compagnie)
      ],
      [
        #s.prix_adulte #s.devise_prix
      ],
    )
  }
}

// Hébergements
#if hebergements.len() > 0 {
  heading(level: 3)[Hébergements]
  for h in hebergements {
    grid(
      columns: (auto, 1fr, auto),
      [
        #h.ville
      ],
      [
        #h.nom_hotel (#h.type_chambre)
        #h.date_checkin → #h.date_checkout (#h.nb_nuitees nuits)
      ],
      [
        #h.prix_par_nuit #h.devise_prix
      ],
    )
  }
}

// Transferts
#if transferts.len() > 0 {
  heading(level: 3)[Transferts]
  for t in transferts {
    grid(
      columns: (auto, 1fr, auto),
      [
        #t.type_transfert
      ],
      [
        #t.trajet (#t.type_vehicule) × #t.nombre_vehicules
      ],
      [
        #t.prix_unitaire #t.devise_prix
      ],
    )
  }
}

// Prestations VIP
#if prestations.len() > 0 {
  heading(level: 3)[Prestations VIP]
  for p in prestations {
    grid(
      columns: (auto, 1fr, auto),
      [
        #p.type_prestation
      ],
      [
        #p.description
      ],
      [
        #p.prix_unitaire #p.devise_prix × #p.quantite
      ],
    )
  }
}

#v(1cm)

// ----- Récapitulatif financier -----
#line(length: 100%, stroke: 0.5pt + or)
#align(right)[
  #set text(size: 11pt)
  #grid(
    columns: (auto, auto),
    gap: 0.5cm,
    [
      #set text(weight: "bold")[Coût net :]
    ],
    [
      #totaux.cout_net_total DZD
    ],
    [
      #set text(weight: "bold")[Marge :]
    ],
    [
      #totaux.montant_marge DZD
    ],
    [
      #set text(weight: "bold", fill: nuit, size: 14pt)[Prix de vente :]
    ],
    [
      #set text(weight: "bold", fill: rouge, size: 14pt)[#totaux.prix_vente_total DZD]
    ],
  )
]

// ----- Pied de page -----
#v(2cm)
#set text(size: 8pt, fill: gray)
#align(center)[
  #devis.nom_agence_fr — #devis.nom_agence_ar
  #linebreak()
  #devis.adresse · #devis.telephone · #devis.email
]