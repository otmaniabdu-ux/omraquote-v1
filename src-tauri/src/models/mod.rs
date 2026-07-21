pub mod devis;
pub mod client;
pub mod passager;
pub mod segment_vol;
pub mod hebergement;
pub mod transfert;
pub mod prestation_vip;
pub mod devise;
pub mod parametres;
pub mod catalogue;

// Réexportations pour faciliter l'usage
pub use devis::Devis;
pub use client::Client;
pub use passager::Passager;
pub use segment_vol::SegmentVol;
pub use hebergement::{Hebergement, HebergementCreate};
pub use transfert::{Transfert, TransfertCreate, TransfertUpdate};
pub use prestation_vip::{PrestationVip, PrestationVipCreate, PrestationVipUpdate};
pub use devise::Devise;
pub use parametres::ParametresAgence;
pub use catalogue::{HotelCatalogue, HotelCatalogueCreate, HotelCatalogueUpdate,
                    CompagnieCatalogue, CompagnieCatalogueCreate, CompagnieCatalogueUpdate};
