# create dat
```
dat ./melt.lammpstrj 1000000 > ./dat/temp.dat
drop ./dat/temp.dat 3.0 > ./dat/droplet.dat
```

# gnuplot
```
set view equal xyz
sp "./dat/droplet.dat" pt 6
```

# fit
```
fit ./dat/droplet.dat 100.0 0.0 > ./dat/fit.dat
min_v ./dat/fit.dat > ./dat/min_v.dat
```

# fitした結果を表示
```
set view equal xyz

Xc = 0.0
Yc = 0.0
Zc = -0.054928819964232795
R = 24.857257463632966

set parametric
set urange [0:2*pi]
set vrange [-pi:pi]

splot './dat/droplet.dat' using 1:2:3 with points pt 6 lc 'blue' title 'Data Points', \
      R*cos(u)*cos(v) + Xc, R*sin(u)*cos(v) + Yc, R*sin(v) + Zc with lines lc 'red' title 'Sphere'
```
