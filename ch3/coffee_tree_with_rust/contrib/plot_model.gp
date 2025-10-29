set terminal pngcairo size 1024,768
set output 'model_plot.png'
set title 'Model Simulation'
set xlabel 'Time'
set ylabel 'Y values'
set key left top
set datafile separator ','
plot \
'model_answer.csv' using 1:2 with lines lw 2 title 'y1', \
'model_answer.csv' using 1:3 with lines lw 2 title 'y2', \
'model_answer.csv' using 1:4 with lines lw 2 title 'y3'
