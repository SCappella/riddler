# The Problem
Originally from [FiveThirtyEight's Riddler column.](https://fivethirtyeight.com/features/work-a-shift-in-the-riddler-gift-shop/)
We have a bottle of 100 tablets.
Every day, we randomly choose one tablet from the bottle.
If we get a whole tablet, we break it half, swallow one half, then put the other half back.
How many days, on average, will it be before we pull a half tablet out of the bottle?

# Derivation
Recall that the definition of the expected value of a (discrete) random variable
`X` is `E(X) = sum(x * P(X=x))`, where `x` ranges over the possible values of `X`.

Now suppose `X` is the number of days we go without getting a single half tablet, but then get a half tablet the next day.
`P(X=k)` is the probability of *not* getting the half tablet in the first `k` days and getting a half tablet on the `k+1`th day.
Since each day is an independent draw, `P(X=k) = P(no half in k days and half on k+1) = P(no half on 1st day)*P(no half on 2nd day)*...*P(no half on kth day)*P(half on k+1th day)`.

Now on the `jth` day, there are `j-1` half tablets and if `N` is the total number of tablets, there are `N-(j-1)` whole tablets.
That means the chance of getting a whole tablet is `(N-(j-1))/N`.
Conversely, the chance of getting a half tablet is `(j-1)/N` on the `jth` day.
Substituting that into the above product, we get `P(X-k) = N/N * (N-1)/N * ... * (N-(k-1))/N * k/N`.
This can be simplified using factorials, but this is enough to write a simple program to get the answer we seek.

So now `E(X) = sum(k * N/N * (N-1)/N * ... * (N-(k-1))/N * k/N)` for `k` between `1` and `N`. `X` can't be larger than `N` since on the `N+1` day, every tablet is broken if we got that far, so it's impossible to go `N+1` days without a half tablet.
Writing that up in Rust, we get the library [`expected_value`](https://github.com/SCappella/riddler/tree/tablets/expected_value), which contains two functions: one for an approximate value,
and another that uses a rational number library to get the exact value.

Using those functions, we get `E(X) = 12.209960630215978 = ` `129277986730885202106151856642773382549665465071825090615034369359178917814747670780594211447306244001059786961886915926297133393516168977984471526892507817` `/` `10587911840678754238354031258495524525642395019531250000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000`.

# Extra Credit
We want to weight the half tablets by some proportion `p` so that when `p=0`, it's impossible to get a half tablet, and the reverse when `p=1`. Doing this in a linear way, we get `P(half) = h*p/(h*p + w*(1-p))`, where `h` is the number of half tablets and `w` is the number of whole tablets.. Note that when `p=1/2`, we get the original unweighted probability.

Since the events are still independent, we can use the same formula as before for `P(X=k)`. I'll elide the details. Suffice to say, it's nasty, but Rust handles it fine. For `p=1/4`, which makes half tablets half as likely as they were before, we have `E(X) = 20.084607007303998 = ` `56942335923098269934182644015316770399752432503864544296838360596979891` `/` `2835123231556909116034579336555902563558839715570107689103059278888960`

# Graphs
We can get some nice graphs out of this. I used the ggplot2 library for R to make these graphs. It's my first time doing something like this, but I think it turned out fine.

![proportions](https://github.com/SCappella/riddler/blob/tablets/proportion.png)
First, we let the proportion `p` vary. Note that as `p` goes to zero, `E(X)` tends toward `N`. This is because no matter how unlikely half tablets are, we'll always get one on the 101st day. As `p` goes to `1`, `E(X)` goes to `1`, since we'll always have the first day where getting a half tablet is impossible.

![tablets](https://github.com/SCappella/riddler/blob/tablets/tablets.png)
Next, we can fix `p=1/2` and let `N` vary. As you can see, the growth of `E(X)` is roughly on the order of the square root of `N`. This is related to the birthday paradox: with `N` distinct possibilities, you'll only need on the order of the square root of `N` things to get a collision.
