---
locale: fr
source_revision: 60b71a4e750d3835abdec3e2e546f11a77759013f3f84ad31d530c433bd67b11
---
# Playground

Lancez le calcul pour deux carnets à 4,50 pièce et 1,25 de livraison. Le montant à payer est de 10,25 et la monnaie sur 20,00 est de 9,75. Passez la quantité de `2n` à `3n` et relancez le calcul pour obtenir 14,75.

Si votre brouillon est ouvert, exportez-le avant de choisir **Réinitialiser** pour charger le reçu.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="fr" :labels="locales['fr']" />

::: details Exemples de méthodes et partage
Vous pouvez modifier les arguments et exécuter l’exemple sur la page de la méthode. Ouvrez-le dans le Playground pour modifier le script ou le partager.

Le partage place au maximum 4 Kio de code UTF-8 et le commit du build de la bibliothèque dans le fragment de l’URL. Le code n’est pas envoyé à un serveur. Un lien partagé nécessite le même build, charge le code sans l’exécuter et efface les résultats précédents. Un lien incorrect conserve votre brouillon actuel. Vérifiez le code, puis cliquez sur Exécuter. Exportez les scripts plus longs dans un fichier ; l’éditeur accepte jusqu’à 64 Kio. Les liens contiennent le texte du script : vérifiez-le avant de le partager.
:::

[Politique de ressources du Playground](./resource-policy)
