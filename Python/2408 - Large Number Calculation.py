n = int(input())
expression = ""

for _ in range(2 * n - 1):
    text = input()

    if text == "/":
        text = "//"

    expression += text

print(eval(expression))
