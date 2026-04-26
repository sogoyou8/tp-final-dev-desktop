# ⚡ COSMIC_NOTES OS v1.0

> **Gestionnaire de Notes Desktop haute-performance développé en Rust.**

## 🚀 Présentation
Ce projet est une application de bureau complète permettant la gestion de notes avec une architecture multi-modules robuste. Elle combine une interface utilisateur immédiate (`egui`), une persistance hybride (JSON/SQLite) et une synchronisation réseau asynchrone.

## ✨ Fonctionnalités
- **Interface HUD Premium** : Design sombre avec accents bleu néon (Electric Blue) et animations fluides.
- **CRUD Complet** : Création, édition, recherche et suppression de notes.
- **Moteur de Données Hybride** : Basculez entre une persistance **JSON** et **SQLite** en temps réel sans redémarrage.
- **Synchronisation Asynchrone** : Importation de données depuis un thread séparé (ne bloque pas l'UI).
- **Import Thématique** : Flux de données exclusif sur le sacre du PSG en Ligue des Champions.
- **Outils Avancés** :
  - 📊 Tableau de bord de statistiques en temps réel.
  - 📤 Exportation de la base de données en format JSON horodaté.
  - ⌨️ Raccourcis clavier : `Ctrl+N` (Nouvelle), `Ctrl+S` (Sauvegarder), `Echap` (Fermer).

## 🏗 Architecture Technique
Le projet est découpé en modules pour une maintenabilité maximale :
- `src/model.rs` : Définition des structures de données (`Note`, `AppState`).
- `src/dao.rs` : Abstraction de la persistance via le Trait `Dao`.
- `src/api.rs` : Client REST asynchrone utilisant `reqwest` et des canaux `mpsc`.
- `src/app.rs` : Logique de l'interface utilisateur et gestion du cycle de vie `eframe`.

## 🛠 Installation et Lancement

### Prérequis
- Rust & Cargo installés (version ≥ 1.75)

### Commandes
1. Cloner le projet.
2. Installer les dépendances et lancer l'application :
```bash
cargo run
```

## 🎮 Utilisation
1. **Création** : Cliquez sur `+ NOUVELLE NOTE` ou utilisez `Ctrl+N`.
2. **Sauvegarde** : Saisissez vos données et cliquez sur le large bouton bleu `💾 VALIDER ET ENREGISTRER`.
3. **Importation** : Utilisez le bouton `📥 IMPORTER` pour récupérer les dernières nouvelles du PSG.
4. **Suppression** : Cliquez sur une note puis sur le bouton `🗑 SUPPRIMER` dans l'éditeur.

---
*Projet réalisé dans le cadre de la formation Desktop Dev - Ynov B3.*
