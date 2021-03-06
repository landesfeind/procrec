set autoscale

set ylabel "CPU %"  tc lt 1 font ",12"
set y2label "Memory (kB)"  tc lt 20 font ",12" offset 1,0
set xlabel "Times (s)" font ",12"

set ytic auto
set y2tic auto

set yr[0:]
set y2r[0:]
set xtics auto

set key right center # legend placement
unset key 
plot    filename using 1:5 title "CPU" with l lt 1 lw 2, \
        "" using 1:7 title "RSS" with l lt 20 lw 2 axes x1y2
