use crate::model::Note;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use rusqlite::{params, Connection};

pub trait Dao {
    fn lire_tout(&self) -> Result<Vec<Note>, Box<dyn Error>>;
    fn sauvegarder(&self, note: &Note) -> Result<(), Box<dyn Error>>;
    fn mettre_a_jour(&self, note: &Note) -> Result<(), Box<dyn Error>>;
    fn supprimer(&self, id: Uuid) -> Result<(), Box<dyn Error>>;
}

// --- JSON DAO ---

pub struct JsonDao {
    chemin: PathBuf,
}

impl JsonDao {
    pub fn new(chemin: &str) -> Self {
        Self {
            chemin: PathBuf::from(chemin),
        }
    }

    fn ecrire_fichier(&self, notes: &Vec<Note>) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(notes)?;
        fs::write(&self.chemin, json)?;
        Ok(())
    }
}

impl Dao for JsonDao {
    fn lire_tout(&self) -> Result<Vec<Note>, Box<dyn Error>> {
        if !self.chemin.exists() {
            return Ok(Vec::new());
        }
        let contenu = fs::read_to_string(&self.chemin)?;
        let notes = serde_json::from_str(&contenu)?;
        Ok(notes)
    }

    fn sauvegarder(&self, note: &Note) -> Result<(), Box<dyn Error>> {
        let mut notes = self.lire_tout()?;
        notes.push(note.clone());
        self.ecrire_fichier(&notes)
    }

    fn mettre_a_jour(&self, note: &Note) -> Result<(), Box<dyn Error>> {
        let mut notes = self.lire_tout()?;
        if let Some(pos) = notes.iter().position(|n| n.id == note.id) {
            notes[pos] = note.clone();
            self.ecrire_fichier(&notes)?;
        }
        Ok(())
    }

    fn supprimer(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut notes = self.lire_tout()?;
        notes.retain(|n| n.id != id);
        self.ecrire_fichier(&notes)
    }
}

// --- SQLITE DAO ---

pub struct SqliteDao {
    chemin: PathBuf,
}

impl SqliteDao {
    pub fn new(chemin: &str) -> Result<Self, Box<dyn Error>> {
        let dao = Self {
            chemin: PathBuf::from(chemin),
        };
        dao.initialiser_schema()?;
        Ok(dao)
    }

    fn connecter(&self) -> Result<Connection, Box<dyn Error>> {
        let conn = Connection::open(&self.chemin)?;
        Ok(conn)
    }

    fn initialiser_schema(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.connecter()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                titre TEXT NOT NULL,
                contenu TEXT NOT NULL,
                tags TEXT NOT NULL,
                date_creation TEXT NOT NULL,
                date_modification TEXT NOT NULL,
                epinglee INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}

impl Dao for SqliteDao {
    fn lire_tout(&self) -> Result<Vec<Note>, Box<dyn Error>> {
        let conn = self.connecter()?;
        let mut stmt = conn.prepare("SELECT id, titre, contenu, tags, date_creation, date_modification, epinglee FROM notes")?;
        
        let notes_iter = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            let tags_str: String = row.get(3)?;
            let date_creation_str: String = row.get(4)?;
            let date_mod_str: String = row.get(5)?;

            Ok(Note {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                titre: row.get(1)?,
                contenu: row.get(2)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                date_creation: chrono::DateTime::parse_from_rfc3339(&date_creation_str).unwrap_or_default().with_timezone(&chrono::Utc),
                date_modification: chrono::DateTime::parse_from_rfc3339(&date_mod_str).unwrap_or_default().with_timezone(&chrono::Utc),
                epinglee: row.get::<_, i32>(6)? != 0,
            })
        })?;

        let mut notes = Vec::new();
        for note in notes_iter {
            notes.push(note?);
        }
        Ok(notes)
    }

    fn sauvegarder(&self, note: &Note) -> Result<(), Box<dyn Error>> {
        let conn = self.connecter()?;
        conn.execute(
            "INSERT INTO notes (id, titre, contenu, tags, date_creation, date_modification, epinglee) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                note.id.to_string(),
                note.titre,
                note.contenu,
                serde_json::to_string(&note.tags)?,
                note.date_creation.to_rfc3339(),
                note.date_modification.to_rfc3339(),
                if note.epinglee { 1 } else { 0 }
            ],
        )?;
        Ok(())
    }

    fn mettre_a_jour(&self, note: &Note) -> Result<(), Box<dyn Error>> {
        let conn = self.connecter()?;
        conn.execute(
            "UPDATE notes SET titre = ?1, contenu = ?2, tags = ?3, date_modification = ?4, epinglee = ?5 WHERE id = ?6",
            params![
                note.titre,
                note.contenu,
                serde_json::to_string(&note.tags)?,
                note.date_modification.to_rfc3339(),
                if note.epinglee { 1 } else { 0 },
                note.id.to_string()
            ],
        )?;
        Ok(())
    }

    fn supprimer(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let conn = self.connecter()?;
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
