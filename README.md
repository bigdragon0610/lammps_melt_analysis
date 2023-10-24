# create dat
```
./target/release/dat ./melt.lammpstrj 1000000 > ./dat/temp.dat
./target/release/drop ./dat/temp.dat > ./dat/droplet.dat
```

# gnuplot
```
set view equal xyz
sp "./dat/droplet.dat" pt 6
```

媒介変数表示
```
set parametric
plot "./dat/droplet.dat" u 1:3, 8*cos(t), 8*sin(t)
```
TODO : z軸がズレたものを媒介変数表示
