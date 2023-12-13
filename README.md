# Advent Of Code 2023

I've never done AoC before. ⭐️

I'm doing it in Go in order to learn Go!

## Day 5 Update: 🦀

### Pivot to Rust 🦀

I decided to try this one in Rust 🦀

I spent so long on learning string parsing that I got frustrated and brute forced the solution for Part 2. I parallelized it and got it to run in 15 min 36 seconds on my Mac. Not ideal! But I have enjoyed gaining some fluency in Rust.

## Day 6

Ok these are getting tough! Still using Rust 🦀

I probably could have brute forced this one and done it a lot faster but I knew there was a way to do it with quadratic equations, it just took a long time to figure out what that was.

## Day 7

Sticking with Rust 🦀. In general, the most time consuming thing is fighting with the borrow checker. I could do these a lot faster in Go (or JS or Python or PHP...).

This was a fun one to do in Rust though! Learned a ton about Enums and sorting. Part 2 wasn't a great leap at all from my Part 1 code. I just had to set the value of J to 0 and add a bunch of futzy conditional logic to determine the hand type. I am sure there are more elegant ways to accomplish that but I'm ok with this.

## Day 7 attempt No. 2 🦀

My solution was ugly (though I learned much about rust enums and structs) and after seeing a bunch of other approaches I decided to take another crack at Part 1. I wanted to see how fluent I could get with chaining methods together, and I wanted to take advantage of the functional features of rust. I much prefer this answer but of course I only reached it after seeing others' solutions.

I know it's sort of cheating 😅!
I do not care though, for I am learning good things 😌

Worth pointing out - the fugly solution with structs and enums? Runs about 5 times faster than the second crack. My guess is that it has to do with the Ord and Partial Ord implementations for HandTypes and CardList 🤔

## Day 8 🦀

This one took me a while. I needed to get a hint and that hint was "least common multiple".

I had actually thought of that early on but I discarded the idea out of hand because it seemed to me that there was no reason to think the number of instructions would line up with the total number of nodes in a given A-Z path, such that the would reliably repeat.

My brain still rebels against it - if there are 100 instructions, and it takes 98 steps to get from A to Z, then when you repeat you start back at A from the 99th instruction, which should change the entire trajectory. So I don't understand why this works and that bugs the hell out of me but I'm moving on.

## Day 9 🦀

Ok I am very happy with how quickly I was able to solve this one. I got it on the first try in both cases. I actually didn't think my pt2 answer looked right but then I plugged it in and lo and behold, it was.

I'm gaining facility in Rust as well as recursion! Woo hoo!
