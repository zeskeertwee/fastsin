set terminal pngcairo dashed size 640,480
set output 'sin.png'
set fit logfile '/dev/null'

set xrange [0:360]
set yrange [-1:1]

set xlabel "Degrees" font ",13"
set datafile separator ","

plot 'data.csv' using 1:2 with line title "fastsin::sin" linecolor rgb "purple", \
     'data.csv' using 1:4 with line title "f64::sin" linecolor rgb "black"