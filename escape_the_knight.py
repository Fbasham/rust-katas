def f(s):
    return int(s[1])-1,int(ord(s[0])-97)
    
def g(y,x):
    return f"{chr(x+97)}{y+1}"

def choose_king_moves(k,n,m):
    ky,kx = f(k)
    nx,ny = f(n)
    if (ky+kx)%2==(ny+nx)%2: t = []
    else:
        t = [(ky-1,kx-1) if ky and kx else (ky-1,kx+1) if ky else (ky+1,kx-1) if kx else (ky+1,kx+1)]
        ky,kx = t[0]
    u = [(ky+(-1 if ky else 1),kx),(ky,kx)]
    return [g(y,x) for y,x in (t+u*m)[:m]]