
"""
OKAY! Grand plans here!
Here's what we need:

if my turn:
    Parse input

    first function in rust:
        update FEN
        Zobrist hash it
        put current position in first index in TT
        put opponent position in last index in TT

    recursively call function in rust:
        generate legal moves:
            use magic bitboard to find legal moves
        find best move:
            look at captures first
            then suggested moves
            then previous best
            then rest
        Zobrist hash it
        check if in TT.
            if so, retrieve values.
        if not
            call eval calculator
            store it if index is free
            if index is filled and depth is greater, overwrite it
        return best move

if not my turn:
    do recursive rust function:
        increase depth search for best values

white
None 10.0 10.0 FEN
<move>
A 9.6
<opp move> 9.6 9.1 FEN
<move>
A 8.9

black
<opp move> 10.0 9.1 FEN
<move>
A 9.4
<opp move> 9.4 7.2 FEN
<move>
A 8.9
"""
import sys
import chessbot
import threading
import time

#essential items
bestMove = None
currentFEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

# general status
previousMove = None
myTime: int = 0
oppTime: int = 0
boardFen = None
isMyTurn: bool = False
game_on: bool = True
myColor = None

def beginning():
    global isMyTurn
    global previousMove
    global myTime
    global oppTime
    global boardFen
    global myColor

    who_starts: str = input()
    myColor = who_starts
    if who_starts == "white":
        isMyTurn = True
    elif who_starts == "black":
        isMyTurn = False

    if isMyTurn:
        status_string: str = input()
        parts = status_string.split()
        previousMove = parts[0]
        myTime = int(parts[1])
        oppTime = int(parts[2])
        boardFen = parts[3]

        if len(parts) != 4:
            print("Initial problem reading opening stats")

        move_decision()

        judge: str = input()
        judge_parts = judge.split()
        acceptance = judge_parts[0]
        if acceptance == "D":
            print("Judge denied move")
        myTime = judge_parts[1]


def control_flow():
    global currentFEN
    global previousMove
    global myTime
    global oppTime
    global boardFen
    global isMyTurn
    while game_on:
        status_string = sys.stdin.readline().strip()

        if status_string:
            isMyTurn = True

            parts = status_string.split(maxsplit=3)
            if len(parts) != 4:
                print("Problem with the status input", len(parts))

            previousMove = parts[0]
            myTime = int(parts[1])
            oppTime = int(parts[2])
            boardFen = parts[3]

            currentFEN = chessbot.update_FEN(currentFEN, previousMove)
            if currentFEN != boardFen:
                print("BoardFEN and currentFEN do not match")
            print(currentFEN)

            move_decision()

            judge: str = input()
            judge_parts = judge.split()
            acceptance = judge_parts[0]
            if acceptance == "D":
                print("Judge denied move")
            myTime = judge_parts[1]

def move_decision():
    global currentFEN
    print(myColor)
    best_move = chessbot.find_best_move(currentFEN, myTime, game_on, myColor)
    currentFEN = chessbot.update_FEN(currentFEN, best_move);
    print(best_move)

def main():
    beginning()
    control_flow()

if __name__ == "__main__":
    main()
