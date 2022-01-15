# Square

stores information about a piece on a square
8 bits

---

0 1 2 3 4 5 6 7

bit 0: occupied flag
0 = empty
1 = occupied

all following fields are undefined when the occupied flag is unset (0)

bit 1: color
0 = white
1 = black

bits 2,3,4: piece
000 king
001 queen
010 rook
011 bishop
100 knight
101 pawn
110 invalid
111 invalid

bit 5: boolean state flag
this bit has different meanings for different pieces
king, rook:
0 = has not previously moved
1 = has previously moved (not eligible for castling)
pawn:
0 = did not move two squares last halfmove
1 = moved two squares last halfmove (thus making it capturable via e.p.)
other pieces: undefined

bit 6: moved last half-move
0 = this piece did not move last half-move
1 = this piece did move last half-move
this is used for turn-checking.
from an initial position, any one arbitrary piece of the side whose turn it is not is chosen
and assigned this flag.

bit 7: reserved
