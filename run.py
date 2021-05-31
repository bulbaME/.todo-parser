import sys
from parse import parser

if __name__ == '__main__':
    fileNames = [n for n in sys.argv if n[-5:] == '.todo']
    ignoreFlag = True if '-i' in sys.argv else False
    planningFlag = True if '-p' in sys.argv else False

    if not len(fileNames):
        print('no file name.')
    
    for n in fileNames:
        parser.parseFile(n, planningFlag, ignoreFlag)