[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/vqbiTdut)
# Chess, Rust edition!
## Description:
Chess is one of the oldest games in the world. It's been with us in its current form since around the 1600s, and has origins far before then. My game is an implementation of this classic game, built using Rust and the bracket_lib library. 

For those unfamiliar, I will explain the rules: Players take turns making moves. The goal of the game is to put your opponent in "checkmate." Checkmate is when you are attacking your opponent's king, and they can make no move to escape that attack. Each piece can move in a different way: Pawns can move forward one square (or two squares if it's their first move) directly forward and capture one square diagonally forward; Bishops can move as far as they want along diagonals; Knights can move in an L shape and are the only piece that can leap over other pieces; Rooks can move horizontally and vertically as far as they want; the Queen can move like Bishops and Rooks; and the King can move one square in any direction. No player may make a move that would result in their king being threatened (AKA "in check").

The game can also end in a draw, or "stalemate" if one player has no legal moves, yet their king is not in check. For my chess afficionados: the En Passant and Castling Rules are not yet implemented into this game.

## How to run
To run this game you can simply download the files, navigate to the target/debug folder, and run the Chess.exe file. The game is currently untested on MacOS and Linux, so run it there with tempered expectations. 

## How to play
To move a piece, either click a piece and drag your mouse to the square you wish to move it to, or click the piece, then click the target square. If it is a legal move, the move will be executed and you will see the piece move, otherwise the move will simply not execute. Once you move, hand the mouse to your opponent: it's their turn! They can't move your pieces, and you can't move theirs. 

Upon completion of the game, the screen will wipe and the winner will be printed on the center of the screen. At this time you may close the game, and if you wish to play again, simply run the .exe file again!

Go to this video to see how to play it: https://youtu.be/yzki7RKHz5A

## Data types
The core of the game runs on three core types: a vector representation of the board, an enum representing each piece type, and integers representing coordinates. Each Piece object (a custom class) is comprised of its type (represented as an enum), two 8 bit signed integers, a bool representing its color, a bool for whether or not it has moved, and finally a char that is printed onto the console to represent the piece itself.

I chose an enumerator for the piece type given that there was a limited number of options for each piece. With only 6 piece types, the enumerator could remain fairly simple. And, since each one has very similar operations (though not exactly the same), it can belong to the piece class. The match function in Rust provides a very helpful way to check which option the enum matches and provide a result depending on that. The match function was used to the most effectiveness in the test_move method of the Piece struct, where we check if a piece can move to a square.

I chose a vector for the board representation since I knew that I would need to loop through the entire array many times, and the size would have to be variable since pieces are often captured. A vector (which is essentially just a stack-dynamic array) is best for this since they have sequentially listed data, and for operations where I am utilizing indices (like the generate_moves and check_for_end functions) the immediate access will be fastest. The biggest necessity for iterating through the array comes from the aforementioned generate_moves function, which has to iterate through the full list every single tick.

Using integers for coordinates felt like a simple choice. Ints are a primitive type, making everything easier. We only have a small number of squares, and representing rows and columns using two ints makes tracking them space efficient and simple. Since we can specify the bits for our ints, we can use minimal space as well. The coordinates must be signed since we are performing subtractions on them while checking for legal moves, and having unsigned ints would result in an error.

We can keep things extra simple for keeping track of the color of each piece (and who's turn it is). Since there are only two options, a bool is the clear choice.

We also utilize tuples occasionally. These are particularly useful since we have x y coordinates and those are easy to store in a two element tuple. We use a vector of a three element tuple to store every possible move, the first element is the index of the piece, and the latter two are where the piece is moving to. 

## Difficulties and Solutions

The biggest challenge in making this game was, surprisingly, finding checkmate. I had initially been focusing on the simpler rules (just the movement rules), and figured checkmate would be simple. Then I realized that checkmate would cause a real challenge, since I needed every possible move for a player. I had to refactor a little bit of my code to make this possible, but after taking a break and thinking about it more I realized I could implement it in a relatively easy manner. I just had to make a (slightly ugly) triple for loop that tested every possible move and then put them into a vector in the form of a three element tuple (as mentioned previously).

Another big challenge was the visuals. I had to hunt a little for a good GUI library for Rust and ran into bracket-lib. I figured bracket-lib was a good choice since it was initially designed for game development. It gave me some troubles (for a while I was struggling to make all the squares the same size), but the biggest trouble was with the lack of online information about it. It's a fairly unused library, so I had to really scour the depths of google if I wanted to find any support for anything. I mostly just relied on the documentation to guide me, but that was sparse to say the least. Ultimately, I managed to make it work and get a good enough understanding to make the game work. The library was still very helpful, even if it gave me a few hurdles. Having something designed for games made the functions well-built for the task.

## The Good, the Bad, and the Ugly

### The Good
I really enjoyed this assignment. I love chess, I love games, and learning how to code chess was a blast. It was fun to see it work, and even just testing it gave me a chance to play the game. Seeing it come to life bit by bit felt good. I also infinitely prefer assignments like this over tests. Making something that actually works is so much better than solving an arbitrary problem with code that isn't useful at all.

### The Bad
Having this and a final and multiple other finals made this week very very difficult to survive. It was a busy week and I lost many a hour of sleep over this. I would prefer just having this assignment without a final (or having this in place of the midterm, and pushing some of the other assignments to earlier in the quarter) but I understand the reason they both exist.

### The Ugly
Similar to the last point, I wish I had more time. There's a lot I wish I had time to do with this program (especially visually) but I simply did not have time to do it this week.

## Learning Experience

I learned a lot about Rust. Rust is very type sensitive, but Rust gives me a lot of control. I particularly like how it handles if elses, since I can put them pretty much anywhere and it doesn't complain (which can not be said about every language). I also learned about traits in Rust. I didn't understand them early enough to use them to their full potential, but if I ever get the chance to use Rust again I look forward to putting them to better use. Finally, I got a bit better at reading documentation, since I had to parse a lot of the bracket-lib documentation. 

# A Game in Rust

The purpose of this assignment are:
1. Learn yet another programming language
2. Practice your creativity
3. Apply the concepts of Chapter 6: Data Types from the book
4. Have an original project that you can add to your portfolio

## Specifications
You will make a program in the Rust programming language. The program will be a game of your choice. The game must not be as simple as Tic-Tac-Toe. Examples of game complexity that are welcome: Battleship, Uno, Texas Hold 'Em Poker, Dungeons and Dragons, Catan, etc. If you are not sure if your game is complex enough, or maybe too complex, feel free to ask your professor to make sure.

Before you start creating your game you will need to study *thoroughly* chapter 6 of the book. The idea is that as you code your game you consciously identify the concepts discussed in that chapter. As you find this concepts in your code you are expected to add comments in the code and make references to the section name from the book.

You will write a **VERY BEAUTIFUL** README file. This file will have the following sections:

* Description of your Game
* How to run
* How to play (You need to show screenshots of the gameplay so a person can learn how to play your game)
  * A YouTube video showing your program running and explaining how it goes.
* Data types (How you applied some concepts from the book in your code, you can reference/use the comments you wrote in your code)
* Difficulties and Solutions (What challenges you found and how you overcame them)
* The Good, the Bad and the Ugly (What you loved about this experience, what has bad, and what did you disliked)
* Learning Experience (briefly explain what you learn in this assignment)

## Rubric
These are the factors to be considered while grading

* Complexity (C), a number [0, 2], when the number is one it means your game is "complex enough," if the value is below 1 means that your program is too simple, and if it is greater than 1 up to 2 it means your program is more complex than expected.
* README (R = [0, 30]), your README file looks good, has all the sections, and it is clear.
* Video (V = [0, 30]), your video is clear, well done, and thorough.
* Data types (DT = [0, 30]), your README and code contain a complete discussion of data types.
* Data types presence (DTP = [0, 1]), you included data type discussion
* Good programming practices (G = -5 per infraction), your code demonstrates good professional programming practices. I will count (n) the number of "bad" programming practices and deduct the points accordingly.
* Subjective assessment (S = [0, 10]), a subjective assessment about the quality of your program.

Your final grade will be calculated as follows:

$Grade=C * DTP * (V+R+DT+S)-n*G$

### Example of Grade Calculation

#### Example 1
* Program: Battleship
* C = 1
* Student didn't include difficulties and learning experiences in the README R = 20
* The video is rushed but otherwise good V = 22
* The README and code included data types discussions DTP = 1
* The data types only discussed arrays DT = 5
* The program showed good programming practices n = 0
* The game looked really good and was fun to play S = 10

$Grade=C * DTP * (V+R+DT+S)-n*G$

$Grade = 1.0 * 1.0 * (22 + 20 + 5 + 10) - 0 * 5 = 57$


#### Example 2
* Program: Dungeons and Dragons
* C = 1.5
* README is complete and well organized README R = 30
* The video is good and thorough V = 30
* The README and code included data types discussions DTP = 1
* The data types discussed several data types in Rust DT = 30
* The program had 4 incidences of bad programming practices (bad indentation, poor choices of variable names, lack of comments, no use of functions) n = 4
* The game looked really good and was fun to play S = 10

$Grade=C * DTP * (V+R+DT+S)-n*G$

$Grade = 1.5 * 1.0 * (30 + 30 + 30 + 10) - 4 * 5 = 130$



