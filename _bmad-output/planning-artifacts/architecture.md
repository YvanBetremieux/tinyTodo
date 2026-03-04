---
stepsCompleted: ['step-01-init', 'step-02-context', 'step-03-starter']
inputDocuments: ['_bmad-output/planning-artifacts/prd.md', '_bmad-output/planning-artifacts/ux-design-specification.md']
workflowType: 'architecture'
project_name: 'tinyTodo'
user_name: 'Yvan.betremieux'
date: '2026-03-04'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**

23 FRs en 6 groupes architecturaux :
- **Accès rapide (FR1-FR4)** — Raccourci global configurable (toggle/maintenu), ouverture/fermeture peek. Implique : module Rust d'enregistrement de raccourci OS, gestion d'état de la fenêtre, communication IPC Tauri entre backend et frontend.
- **Gestion des tâches (FR5-FR10)** — CRUD texte brut, checkbox, drag & drop, LIFO, tâches cochées disparaissent. Implique : store de tâches en mémoire côté frontend, synchronisation bidirectionnelle avec le fichier JSON, animations de transition.
- **Fenêtre Peek (FR11-FR16)** — Liste scrollable, mode saisie activable, navigation clavier, fenêtre persistante en mode saisie. Implique : gestion d'état de fenêtre (éphémère vs persistante), focus management, event handling clavier.
- **Historique (FR17)** — Consultation des tâches accomplies. Implique : vue séparée ou overlay dans la même fenêtre, filtrage des données.
- **Configuration (FR18-FR20)** — Traybar, lancement au démarrage, config raccourci. Implique : fenêtre séparée, persistance config, API système OS (autostart).
- **Persistance & Update (FR21-FR23)** — JSON local, ordre conservé, auto-update. Implique : sérialisation/désérialisation robuste, écriture atomique, intégration Tauri updater.

**Non-Functional Requirements:**

NFRs qui pilotent les décisions d'architecture :
- **Performance (NFR1, NFR4, NFR5)** — Peek < 200ms, lancement < 1s, opérations < 100ms → la fenêtre doit rester en mémoire (cachée, pas détruite/recréée), le JSON doit être chargé en RAM au démarrage
- **Empreinte (NFR2, NFR3)** — RAM < 50Mo, binaire < 10Mo → Tauri est le bon choix (vs Electron), CSS pur (pas de framework lourd)
- **Fiabilité (NFR9, NFR10)** — Zéro perte de données, JSON lisible → écriture atomique obligatoire, format JSON humainement lisible et simple
- **Compatibilité (NFR7, NFR8)** — 3 OS day 1, raccourci global fiable → abstraction de la couche système, tests par plateforme

**Scale & Complexity:**

- Domaine principal : Desktop natif (Tauri 2 + webview + Rust)
- Niveau de complexité : **Low** — périmètre volontairement minimal
- Composants architecturaux estimés : ~8 (Rust backend: shortcut manager, tray manager, config store, task store, window manager; Frontend: peek view, config view, theme engine)

### Technical Constraints & Dependencies

- **Tauri 2** — framework imposé par le PRD, contraint l'architecture (IPC commands, webview unique ou multiple, plugin system)
- **Rust** — backend obligatoire, gestion mémoire et performance natives
- **Fichier JSON** — pas de base de données, sérialisation manuelle
- **Multi-plateforme day 1** — macOS, Windows, Linux — chaque intégration système (raccourci, tray, autostart) doit être validée sur les 3 OS
- **Wayland** — risque identifié pour le raccourci global, nécessite investigation des APIs disponibles
- **Zéro réseau** — aucune dépendance externe sauf le mécanisme d'auto-update Tauri

### Cross-Cutting Concerns Identified

- **Persistance** — touche gestion des tâches, historique, configuration, ordre personnalisé. Stratégie unique d'écriture atomique nécessaire.
- **Gestion d'état inter-fenêtres** — le peek et la fenêtre config partagent des données (liste de tâches, config raccourci). Pattern de communication IPC à définir.
- **Raccourci global multi-OS** — abstraction nécessaire, comportement variable entre OS (notamment Linux Wayland vs X11)
- **Thèmes** — 6 thèmes configurables affectent toute l'UI, gestion via CSS custom properties
- **Navigation clavier** — transversale à tous les composants du peek (liste, saisie, historique)

## Starter Template Evaluation

### Primary Technology Domain

Desktop natif (Tauri 2 + Rust backend + webview frontend) — imposé par le PRD.

### Starter Options Considered

| Option | Stack | Statut |
|--------|-------|--------|
| `create-tauri-app --template vanilla-ts` | Vite + vanilla TypeScript + CSS | **Sélectionné** |
| `create-tauri-app --template react-ts` | Vite + React + TypeScript | Écarté — framework inutile pour 6 composants CSS pur |
| `create-tauri-app --template svelte-ts` | Vite + Svelte + TypeScript | Écarté — dépendance supplémentaire non justifiée |
| Templates communautaires (dannysmith, etc.) | React + Tailwind + state managers | Écartés — trop lourds, philosophie opposée |

### Selected Starter: create-tauri-app (vanilla-ts)

**Rationale for Selection:**

Le template vanilla TypeScript est le choix naturel pour tinyTodo : zéro framework frontend, CSS pur, 6 composants simples. Ajouter React ou Svelte pour ~200 lignes de CSS et une poignée de composants serait de l'over-engineering. Le template officiel Tauri garantit la compatibilité avec la dernière version stable (2.10.x).

**Initialization Command:**

```bash
npm create tauri-app@latest tinyTodo -- --template vanilla-ts
```

**Architectural Decisions Provided by Starter:**

**Language & Runtime:**
- Frontend : TypeScript (compilé par Vite)
- Backend : Rust (géré par Cargo via Tauri)
- IPC : Tauri commands (Rust) invocables depuis TypeScript via `@tauri-apps/api`

**Styling Solution:**
- CSS pur — fichier `styles.css` dans `src/`
- Pas de preprocesseur, pas de framework CSS
- Design tokens via CSS custom properties (définis dans la spécification UX)

**Build Tooling:**
- Vite pour le frontend (dev server + HMR + build production)
- Cargo pour le backend Rust
- `tauri build` pour le packaging multi-plateforme (`.dmg`, `.msi`, `.AppImage`/`.deb`)

**Testing Framework:**
- Non inclus par le starter — à décider dans les décisions architecturales
- Rust : `cargo test` natif
- Frontend : Vitest (naturel avec Vite) ou test manuel

**Code Organization:**
```
tinyTodo/
├── src/                    # Frontend (TypeScript + CSS)
│   ├── main.ts
│   ├── styles.css
│   └── assets/
├── src-tauri/              # Backend Rust
│   ├── src/
│   ├── Cargo.toml
│   ├── capabilities/
│   ├── icons/
│   └── tauri.conf.json
├── index.html
├── package.json
├── tsconfig.json
└── vite.config.ts
```

**Development Experience:**
- Hot Module Replacement via Vite
- TypeScript strict mode
- `tauri dev` pour lancer l'app en mode développement avec HMR

**Note:** L'initialisation du projet avec cette commande devrait être la première story d'implémentation.
