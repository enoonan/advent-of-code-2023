# Day 2

Part 1 was easy but I did get snagged a bit on not removing all the whitespace duhhh

## Part 2 Notes

So I know this requires recursion and I want to sort of whiteboard the problem here before I try to type it out.

We need to know where the current card is (just its index), and we need to recursively calculate the cards it wins X times, where X is the value of the input array at the given index

```go
func sumCards(location int, value int, input []int) int {

  for (i := locationIterations; i < input[location], i++) {
    value += sumCards(location + i, value, input)
  }

  return value
}

```
