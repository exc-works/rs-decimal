---
locale: fr
source_revision: 967fc6e56bfb3ade48f0faf10b0c1e0de7e21a4d9ec093d537fb469385460f6f
translation_provenance: en-US-french-reviewed
---
# Appeler les méthodes Rust dans le navigateur

Le playground exécute du JavaScript qui appelle la bibliothèque Rust compilée en Wasm.
Les noms des méthodes, l’ordre des arguments, l’arrondi et l’état décimal suivent Rust.
JavaScript organise le déroulement du programme ; il ne calcule pas les résultats décimaux.
Consultez le [catalogue des méthodes](./api/) pour les comportements individuels et le
[Playground](./playground) pour exécuter les exemples.

```javascript
const price = Decimal.new_from_string('12.350');
const total = price.mul_exact(Decimal.new(3n));
total.string_with_trailing_zeros(); // '37.050'
```

Utilisez des chaînes exactes pour les entrées décimales et `bigint` pour les arguments i64 (`3n`, pas `3`).
`Decimal.new` rejette les valeurs hors de la plage i64, les chaînes numériques et les number JavaScript.
La précision est un entier JavaScript dans la plage signée de wasm32 ; les valeurs fractionnaires,
non finies ou hors plage sont rejetées avant qu’une conversion ABI puisse les tronquer ou les faire déborder.
Une précision entière négative atteint Rust et produit son erreur InvalidPrecision d’origine.

## Erreurs et propriété

Consultez le tableau `kinds` pour identifier une erreur de calcul telle que `InvalidPrecision`. Un argument de type ou de valeur incorrecte est signalé par la catégorie `parameter`.

Après `rescale`, la valeur initiale reste disponible. `into_rescaled` la consomme même si l’arrondi échoue. Utilisez `clone` au préalable pour la conserver.

## Valeurs nullables et octets

`NullDecimal.new(decimal)` consomme son argument et définit `valid=true`, même pour un zéro non initialisé.
`NullDecimal.default()` représente l’absence. Modifier `valid` ne change que la validité et n’efface pas
la valeur interne. `get_decimal_clone()` est un adaptateur explicitement nommé qui renvoie une copie
indépendante. `set_decimal(value)` déplace une nouvelle valeur interne sans changer la validité.
Il n’y a pas d’accesseur `.decimal` laissant croire à une référence reflétant les modifications.

`coefficient()` renvoie de même un instantané bigint JavaScript immuable. JavaScript ne reproduit pas
les durées de vie des emprunts Rust ni les contrôles de traits à la compilation.
`ByteBuffer.new(Uint8Array)` copie les octets dans un tampon possédé, et `to_uint8_array()` renvoie une copie
qui reste valide après une croissance de la mémoire Wasm. Les méthodes d’ajout sont décrites dans le
[catalogue des méthodes](./api/).

`snapshot(value)` capture immédiatement l’état exact, y compris la valeur interne
masquée d’un nullable invalide. Il renvoie des données gelées ; une modification ultérieure ne change pas
une sortie antérieure. Le moteur isolé limite la sortie récursive et le temps d’exécution ; consultez la
[politique de ressources](./resource-policy).
