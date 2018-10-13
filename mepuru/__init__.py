from .mepuru import ParseResult


def parse(url):
    return ParseResult(url)


__all__ = [
    'parse'
]
