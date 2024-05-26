import math


class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    @staticmethod
    def ccw(p1, p2, p3) -> int:
        x1, y1 = p1.x, p1.y
        x2, y2 = p2.x, p2.y
        x3, y3 = p3.x, p3.y

        return (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)

    @staticmethod
    def dist(p1, p2) -> float:
        return math.hypot(p1.x - p2.x, p1.y - p2.y)


def P5(A):
    n = len(A)
    heights = [Point(i + 1, int(A[i])) for i in range(n)]
    ret = 0

    for i in range(n):
        left = []
        right = []

        # Left side
        if i > 0:
            left.append(i - 1)

            for j in range(i - 2, -1, -1):
                if Point.ccw(heights[i], heights[left[-1]], heights[j]) >= 0:
                    continue

                left.append(j)

        # Right side
        if i < n - 1:
            right.append(i + 1)

            for j in range(i + 2, n):
                if Point.ccw(heights[i], heights[right[-1]], heights[j]) <= 0:
                    continue

                right.append(j)

        ret = max(ret, len(left) + len(right))

    return ret
