# About this project

I present here a Rust command-line program inspired by the buddhist game described in "Rebirth : The Tibetan Game of Liberation", written by Mark Tatz and Jody Kent. I wholeheartedly recommend the book and hope this project motivates you to check it out.

## The original game

The game harks back to the 13th century, having been invented by a reknown Buddhist monk named Sakya Pandita. The version presented in the book and emulated in the present project evolved from it, with the basic principles remaining the same.

It is some kind of a Buddhist snakes and ladders game. Players progress on a 104-square board by rolling a six-sided dice on every turn. Every square has a list associating a dice value to a destination square somewhere on the board. The squares themselves represent fantastical places or states of mind chornicling the progress of players through the cycle of rebirth and spiritual evolution, from the lowest of hells to absolute nirvana.

## What the project does

Up to now, this is a command line interactive program simulating a one-player game of Rebirth, except that instead of simulating a dice roll every turn, the player may choose which face the dice landed on, without being presented the destination associated to each one of these values. This incorporates a modicum of randomness, mitigated by the eventual memorization of dice outcomes throughout repeated playthroughs (the results being always the same).

The game displays a textual description of each square, which is an abridged version of the one found in the book that inspired the project. I'm aware of potential copyright issues on this side. If anyone takes umbrage with it, just let me know.

To make things more interesting, the program is also configured to calculate probabilites associated with dice rolls. For now, it uses a graph algorithm and probability calculations to display, on each turn, the probability of the player following the shortest path to the goal square from his current square.

## In the future

The project isn't over yet! There's still some implementing and tidying up on the code, mostly for stuff I considered optional, like improving probability calculations for the board's trap squares. Also, I'll eventually have to finish filling up the assets/game.json data for all of the 104 squares.

Finally, if someone ever gives me 1 million dollars (more or less), I'll consider that as a stretch goal reached for implementing an online multiplayer feature.
