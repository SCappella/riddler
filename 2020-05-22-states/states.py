STATE_LIST_FILE = './states.txt'
WORD_LIST_FILE = './words.txt'

def read_file(path):
    with open(path, 'r') as file:
        return file.read().splitlines()

def read_states_words():
    states = read_file(STATE_LIST_FILE)
    words = read_file(WORD_LIST_FILE)
    return (states, words)

def mackerel_state(states, word):
    mackerel_states = [state for state in states if not (set(state) & set(word))]
    if len(mackerel_states) == 1:
        return mackerel_states[0]

def longest_mackerel(states, words):
    # casefold words/states
    states = [state.casefold() for state in states]
    words = [word.casefold() for word in words]

    words = sorted(words, key=lambda word: len(word), reverse=True)

    for word in words:
        state = mackerel_state(states, word)
        if state is not None:
            return (word, state)

def longest_mackerel_num_letters(states, words):
    # casefold words/states
    states = [state.casefold() for state in states]
    words = [word.casefold() for word in words]

    words = sorted(words, key=lambda word: len(set(word)), reverse=True)

    for word in words:
        state = mackerel_state(states, word)
        if state is not None:
            return (word, state)

def longest_mackerel_ties(states, words, length):
    # casefold words/states
    states = [state.casefold() for state in states]
    words = [word.casefold() for word in words]

    words = [word for word in words if len(word) == length]

    for word in words:
        state = mackerel_state(states, word)
        if state is not None:
            yield (word, state)

def longest_mackerel_num_letters_ties(states, words, length):
    # casefold words/states
    states = [state.casefold() for state in states]
    words = [word.casefold() for word in words]

    words = [word for word in words if len(set(word)) == length]

    for word in words:
        state = mackerel_state(states, word)
        if state is not None:
            yield (word, state)

def count_mackerels(states, words):
    # casefold words/states
    states = [state.casefold() for state in states]
    words = [word.casefold() for word in words]

    mackerels = {state: 0 for state in states}
    for word in words:
        state = mackerel_state(states, word)
        if state is not None:
            mackerels[state] += 1
    return mackerels

def find_all_mackerels(states, words):
    # casefold words/states
    states = [state.casefold() for state in states]
    words = [word.casefold() for word in words]

    mackerels = {state: [] for state in states}
    for word in words:
        state = mackerel_state(states, word)
        if state is not None:
            mackerels[state].append(word)
    return mackerels

def find_longest_mackerel():
    states = read_file(STATE_LIST_FILE)
    words = read_file(WORD_LIST_FILE)

    long_mack = longest_mackerel(states, words)
    print(f'longest mackerel: {long_mack[0]}, state: {long_mack[1]}')

def find_most_mackerels():
    states = read_file(STATE_LIST_FILE)
    words = read_file(WORD_LIST_FILE)

    mackerels = count_mackerels(states, words)
    print(sorted(mackerels.items(), key=lambda s_w: s_w[1]))

if __name__ == '__main__':
    find_longest_mackerel()
    find_most_mackerels()
