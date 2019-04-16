#! /usr/bin/env python


def fmt(n):

    if n == 1000:
        return "onethousand"
    elif n == 900:
        return "ninehundred"
    elif n == 800:
        return "eighthundred"
    elif n == 700:
        return "sevenhundred"
    elif n == 600:
        return "sixhundred"
    elif n == 500:
        return "fivehundred"
    elif n == 400:
        return "fourhundred"
    elif n == 300:
        return "threehundred"
    elif n == 200:
        return "twohundred"
    elif n == 100:
        return "onehundred"
    elif n == 90:
        return "ninety"
    elif n == 80:
        return "eighty"
    elif n == 70:
        return "seventy"
    elif n == 60:
        return "sixty"
    elif n == 50:
        return "fifty"
    elif n == 40:
        return "forty"
    elif n == 30:
        return "thirty"
    elif n == 20:
        return "twenty"
    elif n == 19:
        return "nineteen"
    elif n == 18:
        return "eighteen"
    elif n == 17:
        return "seventeen"
    elif n == 16:
        return "sixteen"
    elif n == 15:
        return "fifteen"
    elif n == 14:
        return "fourteen"
    elif n == 13:
        return "thirteen"
    elif n == 12:
        return "twelve"
    elif n == 11:
        return "eleven"
    elif n == 10:
        return "ten"
    elif n == 9:
        return "nine"
    elif n == 8:
        return "eight"
    elif n == 7:
        return "seven"
    elif n == 6:
        return "six"
    elif n == 5:
        return "five"
    elif n == 4:
        return "four"
    elif n == 3:
        return "three"
    elif n == 2:
        return "two"
    elif n == 1:
        return "one"
    elif n < 100:
        return "%s%s" % (fmt(n - n % 10), fmt(n % 10))
    else:
        return "%shundredand%s" % (fmt(n // 100), fmt(n % 100))
    

def main():
    i = 0
    for n in xrange(1, 1001):
        print fmt(n)
        i = i + len(fmt(n))
    print i



if __name__ == "__main__":
    main()
    
