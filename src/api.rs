use crate::model::Note;
use std::sync::mpsc;
use std::thread;

pub fn lancer_fetch_notes(tx: mpsc::Sender<Result<Vec<Note>, String>>) {
    thread::spawn(move || {
        // Simulation d'un délai réseau pour l'effet "Jarvis"
        thread::sleep(std::time::Duration::from_millis(800));

        // Données épiques sur le PSG
        let psg_notes = vec![
            ("🏆 Le Sacre Historique", "Le Paris Saint-Germain remporte enfin la Ligue des Champions après une finale épique. Paris est magique !"),
            ("🥇 Mbappé au Sommet", "Auteur d'un triplé légendaire en finale, Kylian Mbappé offre le trophée tant attendu à la capitale."),
            ("🏟️ Liesse au Parc", "Des scènes de joie incroyables au Parc des Princes et sur les Champs-Élysées. Paris ne dort plus."),
            ("🧠 Le Génie de Luis Enrique", "Une maîtrise tactique totale en finale contre le Real Madrid. Le plan s'est déroulé sans accroc."),
            ("⭐ La Première Étoile", "Le PSG entre dans le cercle très fermé des vainqueurs de la C1. L'histoire est en marche."),
            ("🧤 Donnarumma Infranchissable", "Le portier italien a repoussé toutes les tentatives adverses, dont un penalty à la 90ème minute."),
            ("🇧🇷 Marquinhos Capitaine", "Le capitaine brésilien soulève la coupe aux grandes oreilles, les larmes aux yeux."),
            ("⚽ Le But de la Délivrance", "À la 118ème minute, une frappe surpuissante de Vitinha vient délivrer tout un peuple."),
            ("📈 Statistiques de Match", "Possession : 62%, Tirs : 24. Domination totale des Parisiens sur le toit de l'Europe."),
            ("📅 Date Historique", "Ce 26 avril restera gravé comme le jour où Paris est devenu la capitale du football européen.")
        ];

        let notes: Vec<Note> = psg_notes.into_iter().map(|(titre, contenu)| {
            let note = Note::new(titre.to_string(), contenu.to_string(), vec!["PSG".to_string(), "LDC".to_string(), "IMPORT".to_string()]);
            note
        }).collect();

        let _ = tx.send(Ok(notes));
    });
}
