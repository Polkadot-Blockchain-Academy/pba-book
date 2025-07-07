from scipy.interpolate import lagrange


def polynomial_eval(coef, x):
    res = []
    power = 1
    for i in coef:
        res.append(i * power)
        power = power * x
    return round(sum(res))


def polynomial_division(poly, q):
    result = []
    for i in reversed(range(0, len(poly))):
        factor = poly[i] / q[-1]
        result.append(factor)
        poly[i] = poly[i] - (factor * q[-1])
        poly[i - 1] = poly[i - 1] - (q[0] * factor)
        if sum(poly) == 0:
            return (True, result)
    return (False, result)


def gen_poly(y):
    x = range(0, len(y))
    poly = lagrange(x, y)
    poly = [float(x) for x in reversed(poly.coefficients)]
    return poly
