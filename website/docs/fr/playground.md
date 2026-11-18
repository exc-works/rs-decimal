---
locale: fr
source_revision: fbadb341fac6ded3986e830a05797cf96857f33d92f3224a113dd3d05ba9eb85
---
# Bac à sable

Appelez les méthodes Rust depuis JavaScript. Lancez le script pour examiner les valeurs exactes et leur état.

<script setup>
import Playground from "../_components/Playground.vue";
import { locales } from "../../locales.mjs";
</script>

<Playground locale="fr" :labels="locales['fr']" />

Les pages des méthodes proposent des formulaires pour les arguments littéraux de leurs exemples partagés. Les paramètres suivent l’ordre des appels JavaScript ; les noms des méthodes Rust et l’ordre des arguments sont conservés. Ouvrez le code généré dans cet éditeur pour modifier des valeurs complexes ou ajouter des appels. L’état attendu sur la page décrit l’exemple original. Les modifications du formulaire sont conservées lors des changements de langue dans cet onglet.

Le partage place au maximum 4 Kio de code UTF-8 et le commit du build de la bibliothèque dans le fragment de l’URL. Le code n’est pas envoyé à un serveur. Un lien partagé nécessite le même build, charge le code sans l’exécuter et efface les résultats précédents. Un lien incorrect conserve votre brouillon actuel. Vérifiez le code, puis cliquez sur Exécuter. Exportez les scripts plus longs dans un fichier ; l’éditeur accepte jusqu’à 64 Kio. Les liens contiennent le texte du script : vérifiez-le avant de le partager.
