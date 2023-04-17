def main():
    stockfish = open("stockfish.txt", "r")
    mine = open("mine.txt", "r")

    track = []
    track2 = []
    for line in stockfish:
        new_line = line.strip()
        track.append(new_line)

    for line in mine:
        new_line = line.strip()
        track2.append(new_line)


    track.sort()
    track2.sort()

    print("STOCKFISH VS MINE")
    for i in range(len(track)):
        if track[i] != track2[i]:
            print(track[i], track2[i], end = " ")
            print("DIFF:", int(track2[i][6:]) - int(track[i][6:]))


    print()

    print(track)
    print()
    print(track2)
if __name__ == '__main__':
    main()
