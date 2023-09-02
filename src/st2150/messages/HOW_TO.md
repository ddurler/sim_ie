# HOW TO - Implémenter un nouveau message

Le module `st2150/messages` contient un fichier `messageXX.rs` pour chaque message IE du protocole ST2150.

Le répertoire `DOCS` contient la notice technique qui décrit les différents messages de ce protocole.

Pour ajouter un nouveau message XX pris en compte par cette application, on commence par s'appuyer sur un message TT déjà existant :

* Dans le répertoire `st2150/messages`,copier un fichier déjà existant pour un message (`TT`) dans un nouveau fichier `messageXX.rs`

* Dans ce nouveau module `messageXX.rs`, adapter son contenu :

  * Renommer la structure du message `MessageTT` du fichier pris pour modèle en `MessageXX`
  * Ne pas modifier tout de suite le comportement (on va d'abord implémenter ce nouveau message dans le projet avec le comportement du message ZZ pris pour modèle)
  * La fonction `ST2150::is_message_available` (dans `st2150/mod.rs`) utilise le numéro du message (`XX`) comme paramètre : A adapter
  * La fonction `ST2150::do_message_vacation` (dans `st2150/mod.rs`) utilise le numéro du message (`XX`) comme paramètre : A adapter

* Faire une recherche sur l'ensemble du projet pour repérer toutes les références au message TT pris comme modèle (en minuscule messageTT pour les références au module ou en majuscule pour les référence à la structure MessageTT). Pour chaque référence, la dupliquer pour le nouveau message XX :

  * `main.rs` et modules affiliés pour l'IHM
  * `st2150/mod.rs` : Ajouter `use messages::messageXX;`
  * `st2150/messages/mod.rs` : Ajouter `pub mod messageXX;`

A ce stade, le projet doit être consistant et tous les tests doivent être OK.

Il est alors possible d'adapter la construction de la requête et le décodage de la réponse pour ce nouveau message XX.

L'essentiel se fait dans le nouveau module `messageXX`.

Penser également aux différentes primitives qui utilisent le numéro de message (XX) comme paramètre :

* `st2150` pour les helpers (`message_availability` et `do_message_vacation`)
* `st2150::field::Field` pour l'encodage/décodage des trames

Pour la partie test du module du nouveau message, il faut créer une requête 'type' avec une réponse 'type' et vérifier le comportement. Pour le calcul du checksum, mettre une valeur 'bidon' pour la requête et la réponse et le test va échouer en indiquant les valeurs attendues.
