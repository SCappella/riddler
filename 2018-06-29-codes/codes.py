def codes(s, k):
    for code in zip(*(s[i:] for i in range(k))):
        yield ''.join(code)

def find(k):
    code = ''
    tried = set()
    while len(tried) < 10**k:
        found = False
        for c in range(10):
            if (code + str(c))[-k:] not in tried:
                code = code + str(c)
                if len(code) >= k:
                    tried.add(code[-k:])
                found = True
                break
        if not found:
            while code[-1] == '9':
                code = code[:-1]
            c = code[-1]
            code = code[:-1] + str(int(c) + 1)
            tried = set(codes(code, k))
    return code
