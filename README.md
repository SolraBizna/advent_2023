These are my solutions to the [2023 Advent of Code](https://adventofcode.com/2023) puzzles. I'm attempting to solve them as they come out, along with a small group of friends. I'll also be writing my impressions of each puzzle.

# Puzzle 1

Part 1 was not too bad. I found my code golf instincts kicking in, which is why my solution is structured the way it is. Part 2 was unexpectedly evil. It tripped up a lot of my friends. I found myself getting sucked into a trap where I thought I had failed to solve the problem, but actually I had made a typo when entering my solution. Whoops!

# Puzzle 2

This one was harder to understand than to solve. Part 1 was straightforward. On Part 2 I was tripped up by my dyslexia (did a min instead of a max), but otherwise it was straightforward too. When I saw the flavor text about "bags with colored cubes" I recognized the phrasing from some much harder mathematical problems, and thought I was in for something much harder.

(Historical note: I've done a dozen or two puzzles before now, but always a la carte. I've never actually sat there waiting for the timer to count down until this time.)

# Puzzle 3

![](sketch/day03.png)

Day 3 penance sketch: "Did Veris just lose?!"

I actually placed sort of high on the leaderboard this time. Probably because this puzzle is unusual compared to other AOC puzzles, and so folks with extensive libraries of AOC-tailored code are at a disadvantage. My result was also not entirely unmaintainable, although it is really not structured well for testing, and... ugh, that O(n²) bit, I am not proud of.

# Puzzle 4

![](sketch/day04.png)

Day 4 penance sketch: "Goose Tank"

Uneventful. I'm not thrilled about needing a mut for part 2.

# Puzzle 5

![](sketch/day05.png)

Day 5 penance sketch: "Steampunk Rock Paper Scissors"

This one was very very dense. I first implemented it without much optimization, but my spidey sense tingled and when I saw part two I saw that it was right to tingle. I ended up implementing a brute force solution first. I knew how to solve it properly, but I had to prove to my audience that rust is *fast*. So, with the aid of Rayon, some sorting, and the humble binary search, I got it chewing through all 2 132 355 834 possible seeds in just 11.843 seconds. (Then I implemented it properly, and that solution takes so little time to run that I can't measure it on my Ryzen.)

# Puzzle 6

![](sketch/day06.png)

Day 6 penance sketch: "Draygons!"

This one was confusing. I was really afraid of how small the puzzle was. I expected part 2 to be a doozy. I was not afraid for the right reasons. It ended up being confusingly fast and easy to solve. I got tripped up by ilog10, and... that was all.

?????????

I expected to have to do math...

# Puzzle 7

![](sketch/day07.png)

Day 7 penance sketch: "There are too many torches."

I've actually done some poker simulation work in the past, so I was primed for this puzzle. This ended up working against me, though, since I sorted the cards in the hand (e.g. "2KKAK" → "AKKK2"), which Camel Cards does not do.

The changes in part 2 were disruptive enough that, instead of trying to make one codebase that could solve both problems, I made another bin for part 2.

# Puzzle 8

![](sketch/day08.png)

Day 8 penance sketch: "Egg sandwiches. Plurals optional."

Kinda confused by this whole puzzle. The nature, not the puzzle itself. Not too difficult to write, though it was pretty scary until I realized that the initial and subsequent cycles were always the same in these inputs. (I was afraid I was going to have to break a cryptosystem today...)

# Puzzle 9

![](sketch/day09.png)

Day 9 penance sketch: "Someone tried to catch Mr. Resetti with a pokéball."

I was really excited by this puzzle. I don't know why, but its structure was very fun. My solution to part 2 was brute force but effective.
