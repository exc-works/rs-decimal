---
locale: fr
source_revision: f513baf539375e2a99059b44ed14e0bbb75f211d5e47c68b2215892ab1d50f56
---
# Premiers pas

Une cliente commande deux carnets à 4,50 pièce. La livraison coûte 1,25. Calculons le montant à payer pour l’imprimer sur le reçu.

## Calculer le total

Saisissez le prix avec un point dans le code. `new_from_string("4.50")` lit directement les chiffres. Multipliez le prix par deux avec `mul_exact` pour obtenir 9,00, puis ajoutez la livraison avec `add` pour obtenir 10,25.

<<< ../../../examples/web_getting_started.rs#calculate

## Afficher le montant

Pour le sous-total, `string()` donne "9" et `string_with_trailing_zeros()` donne "9.00". Le montant est identique. La seconde méthode conserve les deux décimales du prix pour le reçu.

<<< ../../../examples/web_getting_started.rs#print

[Modifier la commande dans le navigateur](./playground) — Passez la quantité de 2 à 3 et relancez le calcul. Le sous-total doit passer à 13,50 et le montant à payer à 14,75.

## Exécuter l’exemple en Rust

Avec Rust 1.85 ou une version ultérieure, téléchargez le dépôt et lancez l’exemple avec les commandes suivantes. Il affiche 10.25.

```sh
git clone https://github.com/exc-works/rs-decimal.git
cd rs-decimal
cargo run --locked --example web_getting_started
```
