from math import *

def find_the_treasure(a):
    if not a: return False
    t = [90,45,0,315,270,225,180,135]
    y=x=0
    for k,v in a:
        if k>8: return False
        y += v*sin(radians(t[k-1]))
        x += v*cos(radians(t[k-1]))
    return round(x,3),round(y,3)