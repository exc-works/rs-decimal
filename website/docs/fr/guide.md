---
locale: fr
source_revision: b0ab4b73c5319adf6d0f52ca8ad72391d3a1bf1d0a68f2ceac7763706868b54b
---
# Bien démarrer

Construisez les décimaux à partir de chaînes pour conserver une entrée exacte. `mul_exact` additionne les échelles des entrées ; `string()` omet les zéros finaux, tandis que `string_with_trailing_zeros()` les conserve.

<<< ../../../examples/web_getting_started.rs

Les paramètres de précision comptent les décimales, pas les chiffres significatifs. À mi-chemin, `HalfEven` choisit un dernier chiffre conservé pair. `Down` tronque vers zéro, pas vers moins l’infini.

`Decimal::default()` est un zéro numérique non initialisé. Un zéro initialisé est numériquement égal, mais son état diffère. `NullDecimal.valid` détermine indépendamment la présence de la valeur.

Les méthodes faillibles ordinaires renvoient `Result`. Vérifiez la catégorie d’erreur ; seules les méthodes nommées `must_*` provoquent un panic pour les erreurs d’entrée attendues. Le navigateur appellera les mêmes méthodes Rust via un bridge JavaScript dont les correspondances de types et de propriété sont documentées.
