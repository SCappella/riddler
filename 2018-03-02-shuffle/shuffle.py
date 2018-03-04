"""
Computing lower bounds for A000375 (https://oeis.org/A000375).
"""

from itertools import permutations
from random import shuffle


def flip_shuffle(deck):
    """Flip the top part of a deck"""
    split = deck[0]
    return deck[:split][::-1] + deck[split:]


def nshuffles(deck):
    """Find the number of shuffles to get 1 on top"""
    num = 0
    while deck[0] != 1:
        deck = flip_shuffle(deck)
        num += 1
    return num


def maxshuffles(num):
    """Find the maximum shuffles for a number of cards"""
    return max(
        ((deck, nshuffles(deck)) for deck in permutations(range(1, num + 1))),
        key=lambda deck: deck[1])


def prob_max(num):
    """Generate random shuffles, keeping track of the maximum nshuffles"""
    max_shuffles = -1
    deck = list(range(1, num + 1))
    while True:
        shuffle(deck)
        shuffles = nshuffles(deck)
        if shuffles > max_shuffles:
            max_shuffles = shuffles
            print(deck, shuffles)


def mini_shuffle(deck):
    """Iterate through permutations that are one transposition away"""
    for i, _ in enumerate(deck):
        for j in range(i):
            deck[i], deck[j] = deck[j], deck[i]
            yield deck.copy()
            deck[i], deck[j] = deck[j], deck[i]


def local_maximum(start, func):
    """Find a local maximum for a function on permutations"""
    maximum = func(start)
    new = max(mini_shuffle(start), key=func, default=start)
    if func(new) <= maximum:
        return start
    return local_maximum(new, func)


def prob_max_local(num):
    """Generate random shuffles, then locally maximize nshuffles"""
    max_shuffles = -1
    deck = list(range(1, num + 1))
    while True:
        shuffle(deck)
        deck = local_maximum(deck, nshuffles)
        if nshuffles(deck) > max_shuffles:
            max_shuffles = nshuffles(deck)
            print(deck, max_shuffles)
