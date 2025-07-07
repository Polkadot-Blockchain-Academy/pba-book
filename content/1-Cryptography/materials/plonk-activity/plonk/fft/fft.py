#!/usr/bin/env sage -python

import pdb
from ethsnarks.numbertheory import inverse_mod

from ethsnarks import field

p = field.FQ(field.SNARK_SCALAR_FIELD).n


def roots_of_unity(order):
    a = field.FQ(5)
    p = field.FQ(field.SNARK_SCALAR_FIELD)
    return [a ** (i * (p - 1) / order) for i in range(order)]


def polynomial_eval_prime(coef, x, p, step_size=1, start_power=0):
    res = []
    power = x ** start_power
    for i in coef:
        res.append((i * power))
        power = power * x ** step_size
    return sum(res) % p


def sub_group(p, g):
    domain = [1]
    g_ = g
    while g_ != 1:
        g_ = g_ * g % p
        domain.append(g_)


def fft(p, domain, poly):

    if len(poly) == 1:
        return poly

    p_even = poly[::2]
    p_odd = poly[1::2]
    domain_positive = domain[::2]

    L = fft(p, domain_positive, p_even)

    R = fft(p, domain_positive, p_odd)

    p_x = []
    p_x_minus_1 = []

    for i, (x, y) in enumerate(zip(L, R)):

        y_times_root = y * domain[i].n

        p_x.append((x + y_times_root) % p)
        p_x_minus_1.append((x - y_times_root) % p)

    result = [str(x) + str(y) for x, y in zip(p_x, p_x_minus_1)]

    return p_x + p_x_minus_1


def ifft(p, domain, evaluation):
    vals = fft(p, domain, evaluation)

    return [x * inverse_mod(len(vals), p) % p for x in [vals[0]] + vals[1:][::-1]]


def fft_div(poly1, poly2):
    domain = roots_of_unity(4)
    print(max(len(poly1), len(poly2)))
    print(domain)
    poly1_fs = fft(p, domain, poly1)
    poly2_fs = fft(p, domain, poly2)
    res = []
    for x, y in zip(poly1_fs, poly2_fs):
        x = field.FQ(x)
        y = field.FQ(y)
        res.append((x / y).n)
    res = ifft(p, domain, res)
    return res
