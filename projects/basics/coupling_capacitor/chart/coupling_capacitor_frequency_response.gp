# Set the data file separator to comma for CSV files
set datafile separator ","

# Set terminal size (width, height in pixels)
set terminal png size 1600,600

set title "Coupling Capacitor Frequency Response"
set xlabel "Frequency (Hz)"
set ylabel "Vtrms (V)"

# Use logarithmic scale for frequency axis
set logscale x
set format x "10^{%L}"

# Set x-axis range (minimum and maximum)
set xrange [20:*]

# Set grid for better readability
set grid xtics ytics mxtics

# Add crosshair at 50 Hz (vertical and horizontal lines)
set arrow from 50, graph 0 to 50, 0.114 nohead dashtype 2 linecolor rgb "red" linewidth 1
set arrow from 20,0.114 to 50, 0.114 nohead dashtype 2 linecolor rgb "red" linewidth 1
set label "50 Hz (-3db)" at 40, graph 0.55 center textcolor rgb "red"

set key autotitle columnheader
set key bottom right

plot "data.csv" using 1:2 smooth csplines linewidth 1 title "Vtrms", \
     "" using 1:2 with points pointtype 3 pointsize 0 linecolor rgb "blue" notitle