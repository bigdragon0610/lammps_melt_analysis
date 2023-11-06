# create dat
```
cargo run --release --bin dat ./melt.lammpstrj 1000000 > ./dat/temp.dat
cargo run --release --bin drop ./dat/temp.dat > ./dat/droplet.dat
```

# gnuplot
```
set view equal xyz
sp "./dat/droplet.dat" pt 6
```

# fit
```
cargo run --release --bin fit ./dat/droplet.dat > ./dat/fit.dat
```

# fitした結果を表示
```
set view equal xyz

Xc = 0.0
Yc = 0.0
Zc = -2.719976311391644
R = 9.901306433919977

set parametric
set urange [0:2*pi]
set vrange [-pi:pi]

splot './dat/droplet.dat' using 1:2:3 with points pt 6 lc 'blue' title 'Data Points', \
      R*cos(u)*cos(v) + Xc, R*sin(u)*cos(v) + Yc, R*sin(v) + Zc with lines lc 'red' title 'Sphere'
```
