# kmn-pairs

## Defining assignments

You can run:
  ```
  $ cargo run
  ```
to define random assignments between "left" and "right" elements, avoiding some "forbidden" assigmnents,
such that
* each "left" element is assigned to $k$ or $k+1$ "right" elements and
* each "right" element is assigned to exactly $p$ "left" elements.

## Usecase: rankigns

You can run:
  ```
  $ cargo run --bin rank
  ```
to:
* prepare the assigments between the "left" and the "right" elements,
* define ranking in each group of "left" elements assigned to "right" element, and
* get final positions of the "left" elements by scores computed from the rankings.
