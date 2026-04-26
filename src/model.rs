use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: Uuid,
    pub titre: String,
    pub contenu: String,
    pub tags: Vec<String>,
    pub date_creation: DateTime<Utc>,
    pub date_modification: DateTime<Utc>,
    pub epinglee: bool,
}

impl Note {
    pub fn new(titre: String, contenu: String, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            titre,
            contenu,
            tags,
            date_creation: now,
            date_modification: now,
            epinglee: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Backend {
    Json,
    Sqlite,
}

pub struct AppState {
    pub notes: Vec<Note>,
    pub recherche: String,
    pub tag_filtre: Option<String>,
    pub backend_actuel: Backend,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            notes: Vec::new(),
            recherche: String::new(),
            tag_filtre: None,
            backend_actuel: Backend::Json,
        }
    }
}

impl AppState {
    pub fn notes_filtrees(&self) -> Vec<&Note> {
        self.notes
            .iter()
            .filter(|n| {
                let matches_recherche = n.titre.to_lowercase().contains(&self.recherche.to_lowercase())
                    || n.contenu.to_lowercase().contains(&self.recherche.to_lowercase());
                
                let matches_tag = match &self.tag_filtre {
                    Some(tag) => n.tags.contains(tag),
                    None => true,
                };

                matches_recherche && matches_tag
            })
            .collect()
    }

    pub fn tags_uniques(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.notes
            .iter()
            .flat_map(|n| n.tags.clone())
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }
}
