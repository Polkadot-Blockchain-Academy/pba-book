from plonk.fft.fft import polynomial_eval_prime, fft, ifft
from ethsnarks.field import FQ


def test_polynomial_eval_prime():
    assert polynomial_eval_prime([3, 0, 1], 0, 5) == 3
    assert polynomial_eval_prime([3, 0, 1], 1, 5) == 4
    assert polynomial_eval_prime([3, 0, 1], 2, 5) == 2


def test_mul_poly():
    # (x + 1) **2
    a = [1, 1, 0, 0, 0, 0, 0, 0]
    b = [1, 1, 0, 0, 0, 0, 0, 0]
    domain = [FQ(i) for i in [1, 85, 148, 111, 336, 252, 189, 226]]
    p = 337

    a_fft = fft(p, domain, a)
    b_fft = fft(p, domain, b)
    c = [a * b for a, b in zip(a_fft, b_fft)]

    res = ifft(p, domain, c)
    assert res == [1, 2, 1, 0, 0, 0, 0, 0]


def test_mul():
    a = [3, 5, 2, 1, 0, 0, 0, 0]
    b = [5, 9, 8, 1, 0, 0, 0, 0]
    domain = [FQ(i) for i in [1, 85, 148, 111, 336, 252, 189, 226]]
    p = 337

    a_fft = fft(p, domain, a)
    b_fft = fft(p, domain, b)

    a_b_fft = [a * b for a, b in zip(a_fft, b_fft)]
    res = ifft(p, domain, a_b_fft)
    for i, x in enumerate(res):
        res[i] = x % 10
        if int(x / 10) != 0:
            res[i + 1] += int(x / 10)

    assert res == [5, 3, 4, 4, 7, 3, 2, 0]


def test_fft():
    p = 337
    _domain = [1, 85, 148, 111, 336, 252, 189, 226]
    domain = [FQ(i) for i in _domain]
    poly = [3, 1, 4, 1, 5, 9, 2, 6]
    result = []

    p_x = fft(p, domain, poly)

    for x in _domain:
        result.append(polynomial_eval_prime(poly, x, p, 1, 0))

    assert p_x == result


def test_ifft():
    p = 337
    domain = [FQ(i) for i in [1, 85, 148, 111, 336, 252, 189, 226]]
    poly = [3, 1, 4, 1, 5, 9, 2, 6]
    result = []

    p_x = fft(p, domain, poly)

    result = ifft(p, domain, p_x)

    assert result == poly


def test_constant_add():
    a = [1, 1, 0, 0, 0, 0, 0, 0]
    b = [1, 1, 0, 0, 0, 0, 0, 0]
    domain = [FQ(i) for i in [1, 85, 148, 111, 336, 252, 189, 226]]
    p = 337

    constant = 330

    a_fft = fft(p, domain, a)
    b_fft = fft(p, domain, b)
    c = [a * b for a, b in zip(a_fft, b_fft)]

    c = [a + constant for a in c]

    res = ifft(p, domain, c)

    assert res == [331, 2, 1, 0, 0, 0, 0, 0]


def test_poly_add():
    a = [1, 1, 0, 0, 0, 0, 0, 0]
    b = [1, 1, 0, 0, 0, 0, 0, 0]
    domain = [FQ(i) for i in [1, 85, 148, 111, 336, 252, 189, 226]]
    p = 337

    a_fft = fft(p, domain, a)
    b_fft = fft(p, domain, b)
    c = [a * b for a, b in zip(a_fft, b_fft)]

    c = [a + b for a, b in zip(c, a_fft)]

    res = ifft(p, domain, c)

    assert res == [2, 3, 1, 0, 0, 0, 0, 0]


def test_degree_reduction():
    a = [1, 0, 0, 0, 0, 0, 0, 1]
    b = [0, 1, 0, 0, 0, 0, 0, 0]
    domain = [FQ(i) for i in [1, 85, 148, 111, 336, 252, 189, 226]]
    p = 337

    a_fft = fft(p, domain, a)
    b_fft = fft(p, domain, b)
    c = [a * b for a, b in zip(a_fft, b_fft)]

    res = ifft(p, domain, c)

    assert res == [1, 1, 0, 0, 0, 0, 0, 0]
