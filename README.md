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

You can:
* prepare the assigments between the "left" and the "right" elements,
* define rankings of each group of the "left" elements assigned to a "right" element,
* compute scores from the rankings and final positions of the "left" elements,
by running:
  ```
  $ cargo run --bin rank
  ```

 
