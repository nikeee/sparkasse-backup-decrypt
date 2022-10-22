import sys


def print_opt(*args, **kwargs):
    """
    Useful to only print something when a terminal is attached.
    We can make pipe-friendly code this way.
    """

    if sys.stdout.isatty():
        print(*args, **kwargs)
