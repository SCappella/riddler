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
