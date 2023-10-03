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
