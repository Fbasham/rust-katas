function f(n) {
  let s = n + "",
    t = 0;
  for (let i = 0; i < s.length - 1; i++) {
    let x = s[i],
      y = s[i + 1];
    t += (x <= y ? 1 : -1) * x ** Math.abs(x - y);
  }
  return t + +s[s.length - 1];
}

let a = [];
for (let i = 100; i < 1000000; i++) {
  if (f(i) === 0) a.push(i);
}

function prevNext(n) {
  let x = a.filter((e) => e < n);
  let y = a.filter((e) => e > n);
  return [...(x.slice(-1) || []), ...(a.includes(n) ? [n] : []), y[0]];
}
