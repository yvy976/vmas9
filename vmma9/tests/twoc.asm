# twoc.asm
# Manual two's complement tester.
# Stephen Marz
# COSC365: Programming Languages and Systems
# 3-February-2025

# yvy976:tesla2 ~/marz_files/tests> hexdump -C trash.v                                                         <-  8:55AM
# 00000000  de ad be ef 20 01 01 f0  75 65 3a f1 76 61 6c f1  |.... ...ue:.val.|
# 00000010  65 72 20 f1 45 6e 74 f1  00 00 00 40 14 00 00 10  |er .Ent....@....|
# 00000020  00 00 00 04 00 00 00 31  01 00 00 f0 00 00 00 20  |.......1....... |
# 00000030  20 01 01 f0 74 20 3d f1  6d 65 6e f1 70 6c 65 f1  | ...t =.men.ple.|
# 00000040  63 6f 6d f1 27 73 20 f1  54 77 6f f1 00 00 00 40  |com.'s .Two....@|
# 00000050  1c 00 00 10 00 00 00 d0  00 00 00 00 00 00 00 02  |................|
# 00000060  00 00 00 02                                       |....|
# 00000064

main:
    stpush  "Enter value: "
    stprint
    pop     20
    input
    # ~input
    not
    push    1
    # 1 + ~input
    add
    stpush  "Two's complement = "
    stprint
    pop     28
    print
    exit