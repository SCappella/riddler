"""
Solving https://fivethirtyeight.com/features/are-you-a-master-safecracker/
"""


import itertools


class Combo:
    """Describes the correct lock combo"""
    def __init__(self, *combo):
        if len(combo) != 3:
            raise ValueError
        self.combo = {
            'first': int(combo[0]),
            'second': int(combo[1]),
            'third': int(combo[2]),
        }

    def __getitem__(self, key):
        return self.combo[key]

    @classmethod
    def enumerate(cls):
        """
        Return a generator for all possible lock combos
        """
        for cmb in itertools.permutations(range(3)):
            yield cls(*cmb)


class State:
    """
    Describes a lock state
    """
    def __init__(self, *state):
        if len(state) != 3:
            raise ValueError
        self.state = {
            'first': bool(state[0]),
            'second': bool(state[1]),
            'third': bool(state[2]),
        }

    def __getitem__(self, key):
        return self.state[key]

    @classmethod
    def enumerate(cls):
        """
        Return a generator for all possible lock states
        """
        for stt in itertools.product([True, False], repeat=3):
            yield cls(*stt)


class Guess:
    """
    Describes an attempt to unlock the safe
    """
    def __init__(self, *guess):
        if len(guess) != 3:
            raise ValueError
        self.guess = {
            'first': int(guess[0]),
            'second': int(guess[1]),
            'third': int(guess[2]),
        }

    def __getitem__(self, key):
        return self.guess[key]

    def __repr__(self):
        return "({first}, {second}, {third})".format(**self.guess)

    @classmethod
    def enumerate(cls):
        """
        Return a generator for all possible guesses
        """
        for gss in itertools.permutations(range(3)):
            yield cls(*gss)


class Lock:
    """
    Describes all aspects of a lock
    """
    def __init__(self, combo, initial_state):
        self.combo = combo.combo
        self.state = initial_state.state

    @classmethod
    def enumerate(cls):
        """
        Enumerate all possible locks
        """
        for combo in Combo.enumerate():
            for state in State.enumerate():
                yield cls(combo, state)

    def attempt(self, guess):
        """
        Make a guess
        """
        for lock in self.combo:
            if guess[lock] == self.combo[lock]:
                self.state[lock] = not self.state[lock]

    def series_works(self, guesses):
        """
        Checks if a series of guesses unlocks this lock
        """
        if self.is_unlocked():
            return True
        for guess in guesses:
            self.attempt(guess)
            if self.is_unlocked():
                return True
        return False

    def is_unlocked(self):
        """
        Check if the lock is unlocked
        """
        return all(self.state.values())


def search_for_solution():
    """
    Find the shortest series of guesses that unlocks all possible locks
    """
    # making this ordered allows us to ensure that we get the lexographically
    # minimal solution
    strategies = [()]
    success = False
    max_unlocked = (-1, None)
    while not success:
        new_strategies = []
        for strategy in strategies:
            for guess in Guess.enumerate():
                new_strategy = strategy + (guess,)
                new_strategies.append(new_strategy)
                num_unlocked = len([lock for lock in Lock.enumerate()
                                    if lock.series_works(new_strategy)])
                if num_unlocked >= max_unlocked[0]:
                    new_strategies.append(new_strategy)
                if num_unlocked > max_unlocked[0]:
                    max_unlocked = (num_unlocked, new_strategy)
                    print(max_unlocked)
                if all(lock.series_works(new_strategy)
                       for lock in Lock.enumerate()):
                    success = True
                    break
            if success:
                break
        if success:
            break
        strategies = new_strategies
