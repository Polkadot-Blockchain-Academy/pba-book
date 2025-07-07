from py_ecc.bn128.bn128_curve import multiply, add, G1, G2, FQ, neg
from py_ecc.bn128.bn128_pairing import pairing


def powers_of_tau(_secret, depth):
    g_1 = [G1]
    g_2 = [G2, multiply(G2, _secret)]
    secret = _secret
    for i in range(0, depth):
        g_1.append(multiply(G1, secret))
        secret = secret * _secret
    return (g_1, g_2)


def poly_commit(poly, g):
    res = None
    for i, x in enumerate(poly):
        # deal with negitive numbers
        if (x) < 0:
            x = x * -1
            res = add(res, neg(multiply(g[i], x)))
        else:
            res = add(res, multiply(g[i], x))
    return res
