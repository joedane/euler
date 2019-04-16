#! /usr/bin/env python

# wasn't able to think of an analytical solution to this, so ...

#       Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec
days = (31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)
year = 1900
wday = 0  # Monday == 0
mday = 0
month = 0
count = 0

while year < 2001:
    print "%d / %d / %d" % (month, mday, year)
    
    if year > 1900 and mday == 0 and wday == 6:
        count = count + 1
    
    if (mday+1) >= days[month]:
        
        if month == 1 and (year % 400 == 0 or (year % 4 == 0 and year % 100 > 0)) and mday == 28:
            # leap day
            mday = mday + 1  
        elif month == 11:
            month = 0
            mday = 0
            year = year + 1
        else:
            month = month + 1
            mday = 0

        
    else:
        mday = mday + 1

    if wday == 6:
        wday = 0
    else:
        wday = wday + 1
    
print count
        
        
