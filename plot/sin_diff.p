set terminal pngcairo dashed size 640,480
set output 'sin_diff.png'
set fit logfile '/dev/null'
set title "fastsin::sin - f64::sin (precision 2)"

set xrange [0:360]

set xlabel "Degrees" font ",13"
set datafile separator ","

plot 'data.csv' using 1:6 with line notitle linecolor rgb "purple"