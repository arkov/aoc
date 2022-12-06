with open('input.txt', 'r') as f:
    input = f.readline().strip()

i = 0
k = 14 # 4
while(i + 4 <= len(input)):
    if len( set(input[i:i+k]) ) == k:
        print(i + k)
        break
    i+=1
