# cupMatchingGame
Comparing different strategies for the cup matching game from tiktok in rust.
This is my first project in rust beyond simple tutorials.
On tiktok I have seen a cup matching game quite a lot, this is a simple repo for me to try to compare different strategies for the game.

## How the game works
The game is a variation on the popular board game mastermind, one player lays out a series of differently coloured cups (usually 4 or 5 cups) in a specific order, the second person has to guess the order of the cups in a certain number of attempts, and the only information they recieve between each guess is the number of cups in the correct spaces. This is different compared to Mastermind as the player does not know how many of the cups are on the board, but not in the correct location.

## The strategies
The first strategy is the brute_force function, which just guesses orders of the cups until it guesses correctly. for a game with 5 positions and 10 different cups this could be as many as 30240 guesses to get it correct.

An improved strategy, and a likely human strategy, is to only make guesses consistent with the previous information it has recieved, which is what the function my_strategy does. This is able to solve a game with 5 positions and 10 cups in 11 moves.

(currently unimplemented) A better strategy again is to only make consistent guesses, and to choose guesses from those that are possible that reduce the variance of future guesses by the most. This requires looking at each possible guess, and calculating the numer of possible guesses remaining after each possible answer, and choosing the guess with the lowest maximum number of possible guesses. This is much more computationally and memory expensive, but can help reduce the number of guesses required substantially.

(currently unimplemented) An even better strategy, even if it seems counterintuitive, is to allow the program to make guesses it knows are incorrect in order to gain more information. In certain cases, an invalid guess can reduce the total number of guesses required to find the correct answer.
