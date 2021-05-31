from colorama import Fore, Style, Back, init
init()

states = {
    ' ': 0,
    'x': 1,
    'X': 1,
    '/': 2,
    '\\': 2,
    '?': 3,
    '!': 4
}

colors = {
    0: Fore.WHITE,
    1: Fore.GREEN,
    2: Fore.YELLOW,
    3: Fore.RED,
    4: Fore.RED + Style.DIM
}

code, tab = None, None
endline, line = 0, 0 
flag1, flag2 = False, False

def newState(state = None, newState = None):
    if state == newState or (newState not in states.values()):
        return [state, False]
    # if [x]
    elif state == 1:
        if newState in [0, 3, 4]:
            return [4, False]
        else:
            return [state, True]
    # if [?]
    elif state == 3:
        if newState == 4:
            return [newState, True]
        return [state, False]
    # if [ ] or [/]
    elif state == 0 or state == 2:
        return [newState, False]
    return [state, False]
            
force = lambda s: s != 0 and s != 2

def resetGlobals():
    global endline, line, code, tab, flag1, flag2

    code, tab = None, None
    endline, line = 0, 0 
    flag1, flag2 = False, False


def parsedPrint(task, state, level):
    # ignore inactive task
    global endline, line
    if (state[0] == 4 and not flag2) or (state[0] == 3 and not flag1):
        line += 1
        return

    text = code[line]
    print(colors[0], end = '')
    
    if task:
        for l in range(level + 1):
            if l == 0:
                print('└' if line == endline else ('' if line > endline else '├'), end = '')
            elif task:
                print('─', end = '')

        text = text[level:]
        # color only symbol in the checkbox
        s = lambda s: list(states.keys())[list(states.values()).index(s)]
        text = f'[{colors[state[0]]}{text[1] if state[1] else s(state[0])}{colors[0]}]{colors[state[0]]}' + text[3:]
    else:
        text = ('└' if line == endline else ('' if line > endline else '│')) + '   ' * level + colors[state[0]] + text[level:]

    print(text, end = '')

def parseFile(fileName, f1, f2):
    global endline, line, code, tab, flag1, flag2

    # reset global variables
    resetGlobals()

    file = None
    print(f'{Fore.BLACK+Back.WHITE}{fileName.upper()}{Back.RESET+Style.RESET_ALL}\n│')
    try:
        file = open(fileName, 'r')
    except FileNotFoundError:
        print(f'└{Fore.RED+Back.WHITE}not found.\n')
        return

    code = file.readlines()
    flag1, flag2 = f1, f2

    # preprocess
    while line < len(code):
        # find end line
        if code[line].strip():
            endline = line

        # define tabs
        i = 0
        try:
            i = code[line].index('[')
        except:
            pass

        c = code[line].strip()
        if len(c) and c[0] == '[' and c[2] == ']' and not tab:
            posTab = code[line][0:i]
            if not posTab.strip():
                tab = posTab
        
        line += 1

    parentStates = { }
    parentLevel = 0
    # parse
    for line in range(len(code)):
        level = 0
        task = False
        state = False

        # get branch level of the line
        while tab and code[line][level:level+len(tab)] == tab:
            code[line] = code[line][len(tab)-1:]
            level += 1

        # check if line actually contains task
        if line <= endline and code[line][level] == '[' and code[line][level+2] == ']' and code[line][level+1] in states.keys():
            task = True
            state = states[code[line][level + 1]]

        # if brancing out
        if level <= parentLevel:
            l = level
            while l in parentStates.keys():
                parentStates.__delitem__(l)
                l += 1

        # get state of the current task
        if not task or level == 0 or not force(parentStates[level-1]):
            parentStates[level] = state
            parentLevel = level
        else:
            parentStates[level] = parentStates[level-1]
            parentLevel = level - 1

        parsedPrint(task, newState(parentStates[level], state) if task else [parentStates[level]], level)

    print()
