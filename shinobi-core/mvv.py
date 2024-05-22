
WEIGHTS = [100, 320, 330, 500, 900, 10000];

for attacker in range(0, 6):
    print("[", end="")
    for victim in  range(0, 6):
        score = WEIGHTS[victim] - (WEIGHTS[attacker] // 100);
        #print("ATTACKER:", attacker, "VICTIM:", victim, "SCORE:", score, "\t", end="")
        print(score, ", ", end="")
    print("]")
    print()
