---
stepsCompleted: ['step-01-init', 'step-02-discovery', 'step-02b-vision', 'step-02c-executive-summary', 'step-03-success', 'step-04-journeys', 'step-05-domain', 'step-06-innovation', 'step-07-project-type', 'step-08-scoping', 'step-09-functional', 'step-10-nonfunctional', 'step-11-polish', 'step-12-complete']
workflowCompleted: true
completedAt: '2026-03-04'
classification:
  projectType: desktop_app
  domain: general
  complexity: low
  projectContext: greenfield
inputDocuments: ['_bmad-output/brainstorming/brainstorming-session-2026-03-04-110000.md', 'README.md']
workflowType: 'prd'
documentCounts:
  briefs: 0
  research: 0
  brainstorming: 1
  projectDocs: 0
---

# Product Requirements Document - tinyTodo

**Author:** Yvan.betremieux
**Date:** 2026-03-04

## Executive Summary

tinyTodo est une application de bureau ultra-minimaliste de gestion de tâches personnelles. L'interaction principale se fait via un raccourci clavier global qui affiche une fenêtre éphémère centrée (type Spotlight) — l'utilisateur voit ses tâches, en coche, en crée, puis ferme. Pas de fenêtre principale, pas de rappels, pas de cloud, pas d'IA. L'app est un réflexe, pas un outil à ouvrir.

Le produit cible les développeurs et utilisateurs avancés qui veulent noter et gérer des tâches avec un minimum absolu de friction — ceux pour qui ouvrir une app, naviguer dans une interface et configurer des rappels est déjà trop. tinyTodo résout le problème des todo apps qui finissent abandonnées parce qu'elles en font trop.

### Ce qui rend tinyTodo spécial

Le différenciateur de tinyTodo est le **refus radical**. Là où chaque todo app ajoute des fonctionnalités pour se vendre (rappels, tags, catégories, sync, IA), tinyTodo élimine tout sauf le geste essentiel : noter et cocher. L'insight fondamental est que la meilleure todo app est celle qui disparaît — qui ne demande ni temps, ni attention, ni engagement. Un seul raccourci, une fenêtre éphémère, un fichier JSON local. L'app est open source, 100% locale, et si légère qu'on oublie qu'elle est installée.

## Classification du projet

- **Type :** Application de bureau (Tauri 2 + Rust)
- **Domaine :** Productivité personnelle
- **Complexité :** Low — pas de backend, pas de réseau, pas d'authentification. Complexité technique concentrée sur l'intégration système (raccourci global, traybar, fenêtre flottante)
- **Contexte :** Greenfield — nouveau projet, aucun code existant
- **Distribution :** Open source

## Critères de succès

### Succès utilisateur

- L'utilisateur prend le réflexe du raccourci en moins d'une semaine d'utilisation
- Créer une tâche prend moins de 5 secondes du raccourci à la fermeture
- Cocher une tâche prend moins de 2 secondes
- Zéro configuration nécessaire au premier lancement — installer, raccourci, c'est parti

### Succès business (communauté open source)

- **3 mois :** L'auteur utilise tinyTodo quotidiennement, le produit est stable et publié sur GitHub
- **6 mois :** 100+ stars GitHub, premiers retours utilisateurs, premières issues ouvertes par des utilisateurs externes
- **12 mois :** 500+ stars, au moins 3 contributeurs externes ayant mergé une PR, communauté active (issues, discussions)

### Résultats mesurables

- Rétention personnelle : utilisation quotidienne sur 30 jours consécutifs
- Performance : benchmarks automatisés sur le temps d'affichage du peek (voir NFRs pour les seuils)
- Adoption : métriques GitHub (stars, forks, clones, issues)

## Périmètre produit

### MVP (Phase 1)

**Approche :** MVP d'expérience — le minimum pour que le geste "raccourci → peek → action → fermer" soit fluide et fiable.
**Ressources :** Solo developer (l'auteur), open source dès le jour 1.

**Parcours utilisateur supportés :**
- Marc — le déclic (création rapide, peek, cochage)
- Marc — cas limite (scroll, cocher en masse, réordonner)

**Capacités essentielles :**
- Raccourci global configurable (toggle/maintenu) — macOS, Windows, Linux day 1
- Fenêtre peek centrée type Spotlight
- Liste des tâches à faire avec checkbox (cocher au clic ou flèches + Entrée)
- Barre d'espace ou "+" pour activer la saisie, fenêtre persistante
- Raccourci ou Échap pour fermer
- Création de tâche par texte brut
- Édition in-place d'une tâche existante
- Drag & drop pour réordonner, LIFO par défaut
- Persistance fichier JSON local (répertoire OS standard)
- Traybar avec fenêtre de configuration (raccourci, lancement au démarrage)
- Auto-update via Tauri updater

### Post-MVP (Phase 2 — Growth)

- Micro-feedback de complétion (animation/son)
- Historique des tâches accomplies + bouton historique dans le peek
- Thèmes visuels (clair/sombre)
- CONTRIBUTING.md et process de contribution documenté

### Phase 3 — Vision

- IA locale invisible pour parsing de saisie rapide (si besoin émerge)
- Extensibilité communautaire (plugins, API locale)
- Autres plateformes (mobile ?)

## Parcours utilisateurs

### Parcours 1 : Marc, développeur débordé — le déclic

**Scène d'ouverture :** Marc est en plein sprint. Il a un bug à fixer, un call dans 30 minutes, il doit répondre à un email important, et il vient de se rappeler qu'il doit appeler sa banque. Son cerveau déborde. Il a essayé Todoist, Notion, les Notes Apple — à chaque fois, il a arrêté au bout de 3 jours parce que ouvrir l'app et naviguer dans l'interface lui prenait plus de temps que de simplement essayer de tout retenir.

**Action montante :** Marc installe tinyTodo. Pas de compte à créer, pas de tutoriel. Il appuie sur le raccourci global — une fenêtre apparaît, centrée, propre. Il tape "fixer bug auth", Entrée. La fenêtre se ferme. 3 secondes. Il réappuie, tape "appeler banque", Entrée. Il continue à coder.

**Climax :** Entre deux commits, Marc maintient le raccourci. Ses 5 tâches sont là, claires. Il clique sur "répondre email" — coché, disparu. Le geste est devenu un réflexe. Il ne pense plus à ses tâches, il les décharge dans tinyTodo en 3 secondes et son cerveau est libre.

**Résolution :** Marc utilise tinyTodo tous les jours pendant 3 semaines intenses. Le sprint se termine, la charge retombe. Il n'ouvre plus tinyTodo pendant un mois — et c'est très bien. Le jour où ça repart, le raccourci est toujours là, ses anciennes tâches cochées ne polluent pas la vue. Il recommence, zéro friction.

### Parcours 2 : Marc — le cas limite, trop de tâches

**Scène d'ouverture :** Marc a accumulé 25 tâches en une semaine sans en cocher une seule. Le peek est devenu une longue liste qui scrolle.

**Action montante :** Il ouvre le peek. C'est intimidant. Mais il commence à cocher ce qui est fait ou abandonné — clic, clic, clic. Les tâches disparaissent une par une. Il réordonne les 8 restantes par drag & drop, les plus urgentes en haut.

**Climax :** En 2 minutes, sa liste est passée de 25 à 8 tâches priorisées. L'outil n'a rien jugé, rien signalé, rien rappelé. C'est Marc qui a décidé de faire le tri quand il était prêt.

**Résolution :** Marc reprend le rythme — noter, cocher, avancer. L'app ne l'a pas culpabilisé, elle a juste attendu.

### Parcours 3 : Léa, contributrice open source

**Scène d'ouverture :** Léa découvre tinyTodo sur GitHub. Elle l'installe par curiosité — le README promet un outil radicalement minimaliste. Elle l'utilise pendant une semaine et adore le concept.

**Action montante :** Elle remarque que le drag & drop a un petit bug visuel sur Linux. Elle ouvre une issue sur GitHub. Le code est propre, bien structuré — Tauri + Rust, elle connaît. Elle clone le repo, lit le code, comprend l'architecture en 30 minutes.

**Climax :** Léa soumet une PR qui fixe le bug. Le code est simple, le scope est clair. Pas de framework complexe à comprendre, pas de 47 fichiers de configuration. Elle a contribué en une soirée.

**Résolution :** Léa continue à utiliser tinyTodo et contribue occasionnellement. La simplicité du code rend chaque contribution rapide et satisfaisante.

### Résumé des capacités révélées par les parcours

| Parcours | Capacités requises |
|----------|-------------------|
| Marc — déclic | Raccourci global, saisie rapide, peek instantané, fermeture immédiate |
| Marc — cas limite | Scroll dans le peek, cocher en masse, drag & drop, pas de jugement/rappel |
| Léa — contributrice | Code propre et lisible, architecture simple, README clair, process de contribution documenté |

## Exigences spécifiques Desktop App

### Architecture technique

- **Framework :** Tauri 2 (webview natif + backend Rust)
- **Plateformes cibles (MVP) :** macOS, Windows, Linux
- **Persistance :** Fichier JSON local dans le répertoire de données utilisateur de l'OS (`~/Library/Application Support/` sur macOS, `%APPDATA%` sur Windows, `~/.local/share/` sur Linux)
- **Réseau :** Aucun — sauf pour l'auto-update
- **Auto-update :** Système de mise à jour intégré Tauri (seule connexion réseau de l'app)

### Intégration système

- **Raccourci global :** Enregistrement d'un raccourci clavier système intercepté même quand l'app n'est pas au focus. Configurable par l'utilisateur (toggle ou maintenu).
- **Traybar :** Icône dans la zone de notification système, accès à la fenêtre de configuration
- **Fenêtre flottante :** Fenêtre sans bordure, centrée, toujours au-dessus, type overlay. Apparaît/disparaît via le raccourci global.
- **Lancement au démarrage :** Option configurable pour démarrer avec l'OS

### Stratégie de mitigation des risques

**Risque technique — raccourci global cross-platform :**
- Risque principal identifié : comportement variable du raccourci global entre OS (notamment Linux Wayland vs X11)
- Stratégie : analyse des blocages potentiels en amont, solution day 1 pour chaque OS

**Risque marché :**
- Validation par l'usage personnel de l'auteur d'abord
- Publication GitHub pour feedback communautaire rapide

**Risque ressources :**
- Projet solo — le scope MVP est volontairement minuscule
- Si blocage technique, le périmètre fonctionnel est déjà au minimum

## Exigences fonctionnelles

### Accès rapide et raccourci global

- **FR1 :** L'utilisateur peut afficher la fenêtre peek via un raccourci clavier global depuis n'importe quelle application
- **FR2 :** L'utilisateur peut fermer la fenêtre peek via le même raccourci ou la touche Échap
- **FR3 :** L'utilisateur peut configurer le comportement du raccourci (toggle ou maintenu)
- **FR4 :** L'utilisateur peut configurer la combinaison de touches du raccourci

### Gestion des tâches

- **FR5 :** L'utilisateur peut créer une tâche en saisissant du texte brut
- **FR6 :** L'utilisateur peut modifier le texte d'une tâche existante directement dans la liste
- **FR7 :** L'utilisateur peut marquer une tâche comme faite (checkbox)
- **FR8 :** L'utilisateur peut réordonner les tâches par glisser-déposer
- **FR9 :** Les nouvelles tâches apparaissent en haut de la liste (LIFO)
- **FR10 :** Les tâches marquées comme faites disparaissent de la vue peek

### Fenêtre Peek

- **FR11 :** L'utilisateur peut voir la liste de toutes les tâches non cochées dans une fenêtre centrée
- **FR12 :** L'utilisateur peut faire défiler la liste si elle dépasse la taille de la fenêtre
- **FR13 :** L'utilisateur peut activer le mode saisie via la barre d'espace ou un bouton "+"
- **FR14 :** La fenêtre devient persistante quand le mode saisie est activé
- **FR15 :** L'utilisateur peut naviguer dans la liste avec les flèches du clavier
- **FR16 :** L'utilisateur peut cocher une tâche avec la touche Entrée après navigation clavier

### Historique

- **FR17 :** L'utilisateur peut consulter l'historique des tâches accomplies via un bouton dédié dans le peek

### Configuration système

- **FR18 :** L'utilisateur peut accéder aux paramètres via l'icône traybar
- **FR19 :** L'utilisateur peut activer/désactiver le lancement automatique au démarrage de l'OS
- **FR20 :** L'utilisateur peut configurer le raccourci global depuis la fenêtre de paramètres

### Persistance des données

- **FR21 :** Le système persiste toutes les tâches (actives et historique) dans un fichier JSON local
- **FR22 :** Le système conserve l'ordre personnalisé des tâches entre les sessions

### Mise à jour

- **FR23 :** Le système vérifie et installe les mises à jour automatiquement

## Exigences non fonctionnelles

### Performance

- **NFR1 :** La fenêtre peek apparaît en moins de 200ms après activation du raccourci
- **NFR2 :** L'app consomme moins de 50 Mo de RAM au repos
- **NFR3 :** Le binaire distribué fait moins de 10 Mo
- **NFR4 :** L'app se lance en moins de 1 seconde au démarrage de l'OS
- **NFR5 :** Les opérations sur les tâches (créer, cocher, réordonner) s'exécutent de manière instantanée (< 100ms)
- **NFR6 :** L'app ne provoque aucune latence perceptible sur les autres applications en cours d'exécution

### Compatibilité

- **NFR7 :** L'app fonctionne sur macOS (12+), Windows (10+), et Linux (distributions majeures, X11 et Wayland)
- **NFR8 :** Le raccourci global fonctionne de manière fiable quel que soit l'OS et l'application au premier plan

### Fiabilité des données

- **NFR9 :** Aucune perte de données en cas de crash ou de fermeture forcée — le fichier JSON est toujours dans un état cohérent
- **NFR10 :** Le fichier JSON reste lisible et éditable manuellement par un humain
