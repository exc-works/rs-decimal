---
locale: fr
source_revision: 637474c992943a95b792c86f50bfa04821ba9ba29fe9dab7696d42ada69cbe84
translation_provenance: en-US-french-reviewed
---
# Politique de ressources du Playground

Le Playground signale `category: "resource"` lorsqu’un calcul dépasse une des limites ci-dessous.

| Limite | Valeur actuelle |
| --- | --- |
| Entrée décimale | 8192 octets UTF-8, 4096 chiffres de mantisse ou de coefficient après expansion |
| Échelle stockée et valeur absolue de l’échelle d’entrée | 1000 décimales |
| Estimation du coefficient du résultat | 16384 bits, soit au plus 4933 chiffres décimaux |
| Entrée binaire/BSON et ByteBuffer | 65536 octets |
| Listes de règles/erreurs de validation et tables de messages | 64 entrées ; 8192 octets par chaîne ; transport JSON de 65536 octets |
| Texte après substitution de validation | estimation conservatrice de 65536 octets au total avant traduction |
| Appels entrant dans une instance Wasm | 10000, y compris accesseurs, instantanés et métadonnées |
| Degré d’une racine non triviale | 128 |
| Précision de sortie, échelle d’entrée et coefficient d’entrée de log/exp | 128 décimales, 256 décimales, 4096 bits |
| Valeur absolue de l’entrée ordinaire de exp | 64 ; le chemin rapide négatif prouvé arrondi à zéro est conservé |
| Maximum de mémoire linéaire du Wasm compilé | 64 MiB |

L’échelle d’entrée et l’expansion d’un exposant positif sont contrôlées avant l’analyse Rust.
Les conversions de configuration contrôlent le texte décimal d’origine et les chaînes JSON décodées ;
les variantes entières sur 128 bits imposent les bornes signées ou non signées exactes avant le transport.
Les adaptateurs Serde génériques conservent leurs propres erreurs et rejettent les visiteurs flottants.
Le scan SQL contrôle le texte après suppression des guillemets extérieurs selon les règles du cœur.
La validation contrôle les paramètres décimaux avant de construire les règles et limite l’expansion
des espaces réservés répétés avant traduction ; les champs et paramètres insérés restent du texte littéral.
Les décodeurs texte/paramètre/JSON limitent la source à 8192 octets et contrôlent le texte décimal
décodé avant toute modification, y compris les échappements JSON. Les octets numériques JSON d’origine
restent exacts ; le navigateur ne les convertit jamais en nombres JavaScript.
Le décodage binaire limite les bits réels du coefficient et l’échelle encodée avant l’allocation de BigInt.
La source YAML/XML est limitée à 8192 octets ; le texte décodé du scalaire ou de l’élément est contrôlé
avant l’analyse numérique du cœur, y compris les échappements, CDATA, la normalisation des traits de
soulignement YAML et l’insertion de zéros pour les flottants. L’allocation des entiers YAML selon leur base
est limitée de façon conservatrice à 4096 caractères de chiffres hexadécimaux.
Les chaînes BSON suivent les limites du texte ; l’expansion des exposants Decimal128 est contrôlée avant
l’analyse du cœur. Tout échec de contrôle préalable d’un décodeur préserve l’état du récepteur.
Les produits exacts contrôlent la somme des échelles ; les produits arrondis contrôlent l’échelle de sortie.
Tous deux limitent les bits du coefficient intermédiaire avant la multiplication. L’addition et la soustraction
contrôlent la croissance due à l’alignement des échelles et une éventuelle retenue ; la division contrôle
la mise à l’échelle du numérateur ou du dénominateur avant l’allocation. Le quotient/reste contrôle
l’alignement après la vérification du diviseur nul. Les puissances limitent l’échelle intermédiaire
normalisée et la croissance du coefficient, y compris la mise à l’échelle de l’inverse ; les chemins rapides
du cœur pour zéro et les unités, ainsi que Overflow avant allocation, sont conservés. Les opérations
qui prennent possession du récepteur le consomment même si ce contrôle préalable échoue.
La conversion rationnelle exacte contrôle les facteurs du dénominateur réduit et la croissance du
coefficient ; la conversion rationnelle arrondie contrôle la mise à l’échelle du numérateur avant
multiplication. Le dénominateur non nul est vérifié avant de construire le rationnel.
Le formatage limite la largeur à 65536 octets, la précision à 1000 et la croissance du coefficient en
notation fixe. L’ajout contrôle le tampon existant et une estimation conservatrice du texte décimal
avant modification ; même une opération qui tiendrait exactement peut être rejetée.
Les racines limitent les valeurs mises à l’échelle, les puissances de Newton et la certification exacte
du point médian avant l’appel au cœur. Les degrés non triviaux sont limités à 128 ; les chemins rapides
pour zéro, les unités et le degré un restent disponibles dans le budget d’échelle de sortie.
Log/exp imposent des budgets d’entrée et de précision plus stricts pour limiter la réduction rationnelle
et toutes les tentatives d’affinement des intervalles, y compris le maximum de 8192 bits de garde du cœur.
Exp limite également la valeur absolue des arguments ordinaires ; une entrée suffisamment négative,
dont le cœur prouve l’arrondi à zéro, conserve son chemin rapide sûr.
Une précision négative et un diviseur nul conservent les erreurs du cœur lorsque celui-ci peut les
rejeter avant allocation. Rescale contrôle l’échelle demandée et une estimation conservatrice de la
croissance avant d’allouer les puissances de dix. L’arrondi à une précision donnée utilise les mêmes
contrôles. Shift contrôle l’échelle résultante et la croissance du coefficient ; l’arrondi aux chiffres
significatifs réserve une éventuelle retenue. Le dépassement de shift du cœur est conservé lorsque Rust
peut le renvoyer avant allocation. Une suite d’opérations acceptées ne peut donc pas produire un coefficient
ou une chaîne décimale formatée de taille illimitée. Un contrôle conservateur peut rejeter une opération
dont le résultat final serait plus petit, par exemple un produit avec un opérande nul ; cette limite
n’est pas une règle de la bibliothèque principale. Les textes mal formés, précisions négatives et modes
d’arrondi inconnus conservent leurs erreurs de bibliothèque s’ils ne dépassent pas eux-mêmes une limite de ressources.

Une méthode qui consomme sa valeur la consomme même si Rust rejette un dépassement de ressources.
Les opérandes empruntés restent inchangés. Une fois le budget de l’instance épuisé, tous les appels exportés
continuent d’échouer, sauf la libération ; créer une autre façade JavaScript ne le réinitialise pas.
Il faut démarrer une nouvelle instance Wasm pour réinitialiser ce budget.

La façade JavaScript contrôle aussi la longueur des chaînes et des tampons avant que wasm-bindgen ne
copie les arguments. Rust contrôle à nouveau la tranche reçue avant l’analyse du cœur ou une allocation
supplémentaire. Un appel direct à l’ABI brute peut allouer la copie temporaire de l’entrée avant le contrôle
Rust, mais ne peut pas supprimer le maximum de mémoire du module compilé. Le moteur d’exécution doit
garder cette ABI brute privée. Ce n’est pas un quota strict du tas JavaScript ni de la mémoire du processus
du navigateur. Le moteur impose l’arrêt du Worker, les limites de source/sortie et l’isolation réseau.

Une mesure locale avec Node 26 a analysé et lu un coefficient de 4096 chiffres, puis formaté une valeur
d’échelle 1000, en environ 1.2 ms ; c’est une mesure indicative de développement, pas une promesse de latence.
La CI affiche la même mesure et vérifie les limites de rejet. L’artefact Wasm de développement mesuré
pesait environ 196 Ko, ou 73 Ko après gzip ; `build.json` donne la taille exacte, les empreintes et le
maximum de mémoire du commit final.
