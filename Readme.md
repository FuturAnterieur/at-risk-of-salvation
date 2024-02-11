# About this project

I present here a Rust command-line program inspired by the buddhist game described in "Rebirth : The Tibetan Game of Liberation", written by Mark Tatz and Jody Kent. I wholeheartedly recommend the book and hope this project motivates you to check it out.

## The original game

The game harks back to the 13th century, having been invented by a reknown Buddhist monk named Sakya Pandita. The version presented in the book and emulated in the present project evolved from it, with the basic principles remaining the same.

It is some kind of a Buddhist snakes and ladders game. Players progress on a 104-square board by rolling a six-sided dice on every turn. Every square has a list associating a dice value to a destination square somewhere on the board. The squares themselves represent fantastical places or states of mind chornicling the progress of players through the cycle of rebirth and spiritual evolution, from the lowest of hells to absolute nirvana.

## What the project does

Up to now, this is a command line interactive program simulating a one-player game of Rebirth, except that instead of simulating a dice roll every turn, the player may choose which face the dice landed on, without being presented the destination associated to each one of these values. This incorporates a modicum of randomness, mitigated by the eventual memorization of dice outcomes throughout repeated playthroughs (the results being always the same).

The game displays a textual description of each square, which is an abridged version of the one found in the book that inspired the project. I'm aware of potential copyright issues on this side. If anyone takes umbrage with it, just let me know.

To make things more interesting, the program is also configured to calculate probabilites associated to dice rolls. For now, it uses a graph algorithm and probability calculations to display, on each turn, the probability of the player following the shortest path to the goal square from his current square.

## In the future

The project isn't over yet! There's still some implementing and tidying up on the code, mostly for managing the two trap squares present in the game. Also, I'll eventually have to finish filling up the assets/game.json data for all of the 104 squares.

Finally, if someone ever gives me 1 million dollars (more or less), I'll consider that as a stretch goal reached for implementing an online multiplayer feature.


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

Le projet n'est pas encore fini! Il reste à peaufiner le parsing de code de probabilité et le calcul de probabilité pour mieux gérer les deux cases piège du jeu. Ensuite, il faudra évidemment finir d'inclure toutes les 104 cases du jeu dans le fichier assets/game.json.

Finalement, si quelqu'un me donne 1 million de dollars (ou à peu près), j'envisagerai d'implémenter du multiplayer online pour le jeu. Mais on verra bien.