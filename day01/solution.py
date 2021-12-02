import re

with open('input.txt') as f:
    content = f.readlines()

content = [int(x.strip()) for x in content]

def part_1():
    prev_value = -99
    increase_count = 0

    for n in content:
        if prev_value == -99:
            prev_value = n
            continue
        if n > prev_value:
            increase_count += 1
        prev_value = n
        
    return increase_count

def part_2():
    prev_value = -99
    increase_count = 0

    for i,n in enumerate(content):
        if prev_value == -99:
            prev_value = n + content[i+1] + content[i+2]
            continue
        if i+3 > len(content):
            break
        value = n + content[i+1] + content[i+2]
        if value > prev_value:
            increase_count += 1
        prev_value = value
        
    return increase_count

print(part_1())
print(part_2())
