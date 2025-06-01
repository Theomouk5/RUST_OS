# Exercices VGA Buffer en no_std Rust (niveau OS kernel)

Ces exercices sont conçus pour approfondir ta maîtrise du framebuffer VGA texte dans un environnement sans `std`, comme un OS bare-metal en Rust.

---

## 🔹 Exercice 1 : Effacer tout l’écran

Implémente une fonction `clear_screen()` qui remplit tout le buffer VGA avec des espaces `' '` et une couleur par défaut.

> 💡 Utilise une double boucle `for` sur les lignes et colonnes pour écrire un `ScreenChar` vide.

---

## 🔹 Exercice 2 : Écrire du texte à une position précise

Crée une fonction `write_at(row: usize, col: usize, text: &str)` qui écrit une chaîne à un emplacement donné de l’écran.

> 💡 Gère les débordements horizontaux (`col + text.len() > BUFFER_WIDTH`).

---

## 🔹 Exercice 3 : Support de plusieurs couleurs

Modifie `write_at` pour qu’il accepte un paramètre `ColorCode`, afin d’afficher du texte en différentes couleurs.

```rust
fn write_at(row: usize, col: usize, text: &str, color: ColorCode)
```

> 💡 Tu peux créer une macro `color_println!()` pour simplifier son usage.

---

## 🔹 Exercice 4 : Curseur logiciel

Ajoute à ton `Writer` une position `(row, col)` complète (et non juste la colonne), et adapte `write_byte` pour écrire à la bonne position, avec retour à la ligne automatique.

> 🧠 Tu simules ainsi un curseur logiciel, très utile pour implémenter un shell.

---

## 🔹 Exercice 5 : Scroll configurable

Améliore la fonction `new_line()` pour faire défiler tout le texte vers le haut **d’un nombre de lignes configurable**, pas juste 1.

```rust
fn scroll_up(&mut self, lines: usize)
```

---

## 🔹 Exercice 6 : Affichage centré

Implémente une fonction `write_centered(row: usize, text: &str)` qui affiche du texte centré horizontalement à une ligne donnée.

---

## 🔹 Exercice 7 : Barre d’état

Crée une fonction `draw_status_bar(text: &str)` qui affiche une ligne colorée (ex : fond bleu, texte blanc) en bas de l’écran.

---

## 🔹 Exercice 8 : Écran de démarrage personnalisé

Combine tous les éléments précédents pour afficher un écran de démarrage comme :

```
┌────────────────────────────────────────────────────┐
│                                                    │
│           Bienvenue dans Kerber OS !               │
│                                                    │
│            Initialisation du système...            │
│                                                    │
└────────────────────────────────────────────────────┘
```

> 💡 Tu peux dessiner les bordures avec les caractères ASCII étendus : `─`, `│`, `┌`, `┘`, etc.

---

Bon codage ! 🚀