

# À propos de ce projet

Je propose ici un programme command-line en Rust inspiré du jeu bouddhiste décrit dans le livre "Rebirth : The Tibetan Game of Liberation" par Mark Tatz et Jody Kent. C'est un livre que je recommande fortement; j'espère que ce projet vous inspirera à y jeter un coup d'oeil.

## Le jeu originel 

Le jeu en question tirerait ses origines au 13e siècle, par l'invention d'un moine tibétain réputé nommé Sakya Pandita. La version présentée dans le livre et reproduite dans le présent projet en constitue une évolution, bien que le principe de base reste le même.

Il s'agit d'une sorte de serpents et échelles bouddhiste. Les joueurs avancent (ou reculent) sur un plateau de 104 cases en tirant à chaque tour un dé à six faces. Chaque case présente une liste associant chaque résultat de dé à une case destination ailleurs sur le plateau. Les cases représentent des lieux ou bien des états d'esprit qui forment des points de repère jalonnant le parcours des joueurs à travers le cycle des réincarnations et de l'évolution spirituelle, du plus bas des enfers jusqu'au nirvana absolu.

## Ce que le projet fait

Jusqu'à présent, et pour l'instant, ce projet propose un programme en ligne de commande qui simule une partie de jeu pour un joueur, excepté qu'au lieu de simuler un lancé de dé aléatoire à chaque tour, il est proposé au joueur de choisir le chiffre résultant du lancé de dés, sans toutefois lui indiquer sur quelle case chaque chiffre le fait aboutir. Ceci incorpore donc un certain élément de hasard, mitigé par l'éventuelle mémorisation des résultats de lancé de dés selon la case de départ (ces résultats restant toujours les mêmes).

Le jeu affiche pour chaque case une description dont le texte constitue une version abrégée de celle du livre ayant inspiré le projet. Je suis au courant de problèmes de copyright potentiels de ce côté. Si quelqu'un en prend ombrage, il n'a qu'à m'en aviser.

Pour rendre cela plus intéressant, le programme est aussi configuré pour calculer des statistiques basées sur les probabilités associées aux lancés de dés. Pour l'instant, il utilise un algorithme de graphe ainsi que des calculs de probabilité pour afficher à chaque tour la probabilité que le joueur emprunte le plus court chemin menant à la destination finale à partir de la case où il se trouve présentement. 

## Pour le futur

Le projet n'est pas encore fini! Il reste quelques features optionnelles, comme l'amélioration des calculs de probabilité pour les cases piège du jeu. Ensuite, il faudra évidemment finir d'inclure toutes les 104 cases du jeu dans le fichier assets/game.json.

Finalement, si quelqu'un me donne 1 million de dollars (ou à peu près), j'envisagerai d'implémenter du multiplayer online pour le jeu. Mais on verra bien.