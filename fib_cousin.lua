kata = {}
function kata.solve(n) 
  a = 1
  b = 1
  for i=1,n do
    a,b = b,a+b+1
  end
  return a
end
return kata