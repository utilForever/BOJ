#include <bits/stdc++.h>

using namespace std;
using f128 = long double;

const f128 INF = numeric_limits<double>::infinity();
const f128 GOLDEN_RATIO = (1.0 + sqrtl(5.0L)) / 2.0;

int n;
f128 x[100000], y[100000];
f128 x_square[100000];
f128 y_delta[100000];
f128 x_max = 1.0;
f128 ret;

// Calculate the value of c of the equation ax^2 + bx + c
f128 calculate_c(f128 b) {
    f128 min_val = INF;
    f128 max_val = -INF;

    for (size_t i = 0; i < n; i++) {
        f128 val = y_delta[i] - b * x[i];
        min_val = min(min_val, val);
        max_val = max(max_val, val);
    }

    ret = min(ret, max_val - min_val);
    return max_val - min_val;
}

// Calculate the value of b of the equation ax^2 + bx + c
f128 calculate_b(f128 a) {
    for (size_t i = 0; i < n; i++) {
        y_delta[i] = y[i] - a * x_square[i];
    }

    f128 left = -1e13, right = 1e13;
    f128 mid1 = (GOLDEN_RATIO * left + right) / (1.0 + GOLDEN_RATIO);
    f128 mid2 = (left + GOLDEN_RATIO * right) / (1.0 + GOLDEN_RATIO);
    f128 val_mid1 = calculate_c(mid1);
    f128 val_mid2 = calculate_c(mid2);

    for (int i = 0; i < 120; i++) {
        if (val_mid1 < val_mid2) {
            right = mid2;
            mid2 = mid1;
            mid1 = (GOLDEN_RATIO * left + right) / (1.0 + GOLDEN_RATIO);
            val_mid2 = val_mid1;
            val_mid1 = calculate_c(mid1);
        } else {
            left = mid1;
            mid1 = mid2;
            mid2 = (left + GOLDEN_RATIO * right) / (1.0 + GOLDEN_RATIO);
            val_mid1 = val_mid2;
            val_mid2 = calculate_c(mid2);
        }
    }

    return calculate_c(left);
}

// Calculate the value of a of the equation ax^2 + bx + c
f128 calculate_a() {
    f128 left = -1e7 / x_max, right = 1e7 / x_max;
    f128 mid1 = (GOLDEN_RATIO * left + right) / (1.0 + GOLDEN_RATIO);
    f128 mid2 = (left + GOLDEN_RATIO * right) / (1.0 + GOLDEN_RATIO);
    f128 val_mid1 = calculate_b(mid1);
    f128 val_mid2 = calculate_b(mid2);

    for (int i = 0; i < 90; i++) {
        if (val_mid1 < val_mid2) {
            right = mid2;
            mid2 = mid1;
            mid1 = (GOLDEN_RATIO * left + right) / (1.0 + GOLDEN_RATIO);
            val_mid2 = val_mid1;
            val_mid1 = calculate_b(mid1);
        } else {
            left = mid1;
            mid1 = mid2;
            mid2 = (left + GOLDEN_RATIO * right) / (1.0 + GOLDEN_RATIO);
            val_mid1 = val_mid2;
            val_mid2 = calculate_b(mid2);
        }
    }

    return calculate_b(left);
}

// Reference: https://github.com/dacin21/dacin21_codebook
// Reference: "2022-2023 Winter Petrozavodsk Camp, Day 2: GP of ainta" Editorial
int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t;
    cin >> t;

    while (t--) {
        cin >> n;

        f128 x_min = INF, y_min = INF;
        ret = INF;
        x_max = 1.0;

        for (size_t i = 0; i < n; i++) {
            int xi, yi;
            cin >> xi >> yi;
            x[i] = static_cast<f128>(xi);
            y[i] = static_cast<f128>(yi);

            x_min = min(x_min, x[i]);
            y_min = min(y_min, y[i]);
        }

        for (size_t i = 0; i < n; i++) {
            x[i] -= x_min;
            y[i] -= y_min;
            x_square[i] = x[i] * x[i];
            x_max = max(x_max, x[i]);
        }

        calculate_a();

        cout << fixed << setprecision(12) << ret * ret / 4.0 << '\n';
    }

    return 0;
}
