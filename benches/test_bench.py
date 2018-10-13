u = 'https://zh.wikipedia.org/w/index.php?title=%E9%9D%92%E6%98%A5%E6%9C%9F%E8%B1%AC%E9%A0%AD&action=edit'


def test_urlparse(benchmark):
    from urllib.parse import urlparse
    benchmark(urlparse, u)


def test_mepuru(benchmark):
    from mepuru import parse as urlparse
    benchmark(urlparse, u)
