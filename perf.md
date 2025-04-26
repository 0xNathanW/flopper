cargo run --bin equity -r -- "77+, A9s+, KTs+, AJo+" "44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+" --lookup-path ./data/lookup_table.bin
+-------------------------------------------------------------+--------+--------+-------+
| Range                                                       | Equity | Win %  | Tie % |
+-------------------------------------------------------------+--------+--------+-------+
| 77+, A9s+, KTs+, AJo+                                       | 57.66% | 55.77% | 1.90% |
+-------------------------------------------------------------+--------+--------+-------+
| 44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+ | 42.34% | 40.44% | 1.90% |
+-------------------------------------------------------------+--------+--------+-------+
Performance counter stats for 'cargo run --bin equity -r -- 77+, A9s+, KTs+, AJo+ 44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+ --lookup-path ./data/lookup_table.bin':

      2,681,868.17 msec task-clock                       #   10.645 CPUs utilized             
           401,911      context-switches                 #  149.862 /sec                      
            12,056      cpu-migrations                   #    4.495 /sec                      
            41,175      page-faults                      #   15.353 /sec                      
10,787,950,431,679      cycles                           #    4.023 GHz                       
   394,263,313,145      stalled-cycles-frontend          #    3.65% frontend cycles idle      
18,548,000,304,078      instructions                     #    1.72  insn per cycle            
                                                  #    0.02  stalled cycles per insn   
 3,355,117,532,036      branches                         #    1.251 G/sec                     
    20,661,297,244      branch-misses                    #    0.62% of all branches           

     251.940431188 seconds time elapsed

    2675.145615000 seconds user
       5.430031000 seconds sys

enumerate_hands changed from vec to preallocated array.
Performance counter stats for 'cargo run --bin equity -r -- 77+, A9s+, KTs+, AJo+ 44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+ --lookup-path ./data/lookup_table.bin':

      1,410,490.82 msec task-clock                       #   10.892 CPUs utilized             
           192,240      context-switches                 #  136.293 /sec                      
             1,983      cpu-migrations                   #    1.406 /sec                      
           112,732      page-faults                      #   79.924 /sec                      
 5,752,522,340,891      cycles                           #    4.078 GHz                       
   230,412,484,021      stalled-cycles-frontend          #    4.01% frontend cycles idle      
 8,817,510,966,308      instructions                     #    1.53  insn per cycle            
                                                  #    0.03  stalled cycles per insn   
 1,451,723,966,139      branches                         #    1.029 G/sec                     
    16,465,277,506      branch-misses                    #    1.13% of all branches           

     129.502987155 seconds time elapsed

    1407.255328000 seconds user
       2.637371000 seconds sys


Added some progress counters and whatnot.
Performance counter stats for 'cargo run --bin equity -r -- 77+, A9s+, KTs+, AJo+ 44+, A2s+, K9s+, Q9s+, J9s+, T9s, 98s, 87s, 76s, ATo+, KJo+ --lookup ./data/lookup_table.bin':

      1,343,377.08 msec task-clock                       #    9.421 CPUs utilized             
           306,922      context-switches                 #  228.470 /sec                      
             4,615      cpu-migrations                   #    3.435 /sec                      
            38,556      page-faults                      #   28.701 /sec                      
 5,431,124,034,988      cycles                           #    4.043 GHz                       
   234,200,536,698      stalled-cycles-frontend          #    4.31% frontend cycles idle      
 8,589,135,375,092      instructions                     #    1.58  insn per cycle            
                                                  #    0.03  stalled cycles per insn   
 1,374,915,214,127      branches                         #    1.023 G/sec                     
    16,827,526,548      branch-misses                    #    1.22% of all branches           

     142.591406106 seconds time elapsed

    1337.714120000 seconds user
       4.628992000 seconds sys