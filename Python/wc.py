#!/usr/bin/env python3
import argparse


def parse_file(filename):
    return open(filename, "rb")


def wc(fp):
    """Returns newline, word, and byte counts for the file.

    Args:
        fp: file object opened in "rb" mode.
    """
    lines, words, read_bytes = 0, 0, 0
    is_word = False

    block = fp.read1(4096)
    while block != b'':
        for i in range(len(block)):
            c = block[i:i + 1]
            if c == b'\n':
                lines += 1
            if c.isspace():
                is_word = False
            elif is_word is False:
                words += 1
                is_word = True

        read_bytes += len(block)
        block = fp.read1(4096)

    return lines, words, read_bytes


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="print newline, word, and byte counts for each file")
    parser.add_argument("filename", type=str)

    args = parser.parse_args()

    fp = parse_file(args.filename)
    lines, words, read_bytes = wc(fp)

    print("{}\t{}\t{}\t{}".format(lines, words, read_bytes, args.filename))
