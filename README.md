# tinyTodo

## Vision

App de gestion de taches **ultra minimaliste et ultra fluide**.

## Stack technique

- **Frontend** : [Tauri](https://tauri.app/) (app native legere)
- **Backend** : Rust (coeur applicatif, notifications systeme, performance)
- **IA** : a explorer (suggestions, priorisation intelligente, NLP pour creer des taches ?)

## Fonctionnalites

### Core

- Creer / supprimer / editer une tache
- Marquer une tache comme terminee
- Filtrer par statut (tout, en cours, termine)
- Persistence locale (SQLite ou fichier JSON)

### Rappels et notifications

- **Rappel par defaut : 30 minutes** apres creation
- Choix du delai par **pas de 30 min** (30min, 1h, 1h30, 2h...)
- Possibilite de choisir une date/heure precise
- **Notifications systeme natives** quand le rappel arrive
- **Badge sur l'icone de l'app** quand une tache atteint sa date d'echeance

### Raccourcis clavier

- Raccourci global pour **creer rapidement une tache** (depuis n'importe ou)
- Raccourci global pour **ouvrir l'app** et voir les taches en cours
- Navigation clavier dans la liste de taches

### IA (a explorer)

- Suggestions de priorisation
- Parsing en langage naturel ("rappelle-moi d'appeler le dentiste demain a 10h")
- Categorisation automatique

## Architecture

> A definir — on verra comment ca s'implemente et comment ca s'architecture.

## Notes

-
