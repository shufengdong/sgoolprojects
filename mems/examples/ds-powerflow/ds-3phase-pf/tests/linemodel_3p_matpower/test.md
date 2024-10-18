在nlp.rs中写了函数test_get_pf_nlp_constraints()进行了测试，模型为一条线路，选择ieee13节点配电网模型，config601线路。
电压等级为12.47kv，具体参数见t_case2p_a_1.m文件。

用matpower进行了测试，结果如下：

```matlab
MATPOWER Version 8.0, 17-May-2024
Power Flow -- AC-polar-power formulation

 it    max residual        max ∆x
----  --------------  --------------
  0      2.375e+00           -    
  1      1.985e-02       1.293e-02
  2      3.719e-06       1.653e-04
  3      1.430e-13       3.221e-08
Newton's method converged in 3 iterations.
PF successful

PF succeeded in 0.04 seconds (0.03 setup + 0.00 solve)
================================================================================
|     System Summary                                                           |
================================================================================
  elements                on     off    total
 --------------------- ------- ------- -------
  3-ph Buses                2       -       2
  3-ph Generators           1       -       1
  3-ph Loads                1       -       1
  3-ph Lines                1       -       1

  Total 3-ph generation               5468.4 kW       2502.3 kVAr
  Total 3-ph load                     5450.0 kW       2442.6 kVAr
  Total 3-ph line loss                  18.4 kW         59.7 kVAr

================================================================================
|     3-ph Bus Data                                                            |
================================================================================
  3-ph            Phase A Voltage    Phase B Voltage    Phase C Voltage
 Bus ID   Status   (kV)     (deg)     (kV)     (deg)     (kV)     (deg)
--------  ------  -------  -------   -------  -------   -------  -------
      1      1     7.1996     0.00    7.1996  -120.00    7.1996   120.00
      2      1     7.1769    -0.08    7.1396  -120.37    7.1500   119.25

================================================================================
|     3-ph Generator Data                                                      |
================================================================================
  3-ph      3-ph             Phase A Power     Phase B Power     Phase C Power
 Gen ID    Bus ID   Status   (kW)    (KVAr)    (kW)    (kVAr)    (kW)    (kVAr)
--------  --------  ------  -------  ------   -------  ------   -------  ------
      1         1      1    1277.88  794.49   1809.44  890.72   2381.04  817.09

================================================================================
|     3-ph Load Data                                                           |
================================================================================
  3-ph      3-ph             Phase A Power     Phase B Power     Phase C Power
Load ID    Bus ID   Status   (kW)     (PF)     (kW)     (PF)     (kW)     (PF)
--------  --------  ------  -------  ------   -------  ------   -------  ------
      1         2      1    1275.00  0.8500   1800.00  0.9000   2375.00  0.9500

================================================================================
|     3-ph Line Data                                                           |
================================================================================
-->  Current Injections at "From" Bus
  3-ph    3-ph Bus  3-ph Bus          Phase A Current  Phase B Current  Phase C Current
Line ID    From ID   To ID    Status   (A)    (deg)     (A)    (deg)     (A)    (deg)
--------  --------  --------  ------  ------  ------   ------  ------   ------  ------
      1         1         2      1    209.00   -31.9   280.13  -146.2   349.65   101.1

<--  Current Injections at "To" Bus
  3-ph    3-ph Bus  3-ph Bus          Phase A Current  Phase B Current  Phase C Current
Line ID    From ID   To ID    Status   (A)    (deg)     (A)    (deg)     (A)    (deg)
--------  --------  --------  ------  ------  ------   ------  ------   ------  ------
      1         1         2      1    209.00   148.1   280.13    33.8   349.65   -78.9

-->  Power Injections at "From" Bus
  3-ph    3-ph Bus  3-ph Bus          Phase A Power    Phase B Power    Phase C Power
Line ID    From ID   To ID    Status   (kW)   (kVAr)    (kW)   (kVAr)    (kW)   (kVAr)
--------  --------  --------  ------  ------  ------   ------  ------   ------  ------
      1         1         2      1    1277.9   794.5   1809.4   890.7   2381.0   817.1

<--  Power Injections at "To" Bus
  3-ph    3-ph Bus  3-ph Bus          Phase A Power    Phase B Power    Phase C Power
Line ID    From ID   To ID    Status   (kW)   (kVAr)    (kW)   (kVAr)    (kW)   (kVAr)
--------  --------  --------  ------  ------  ------   ------  ------   ------  ------
      1         1         2      1   -1275.0  -790.2  -1800.0  -871.8  -2375.0  -780.6
```

函数test_get_pf_nlp_constraints()输出结果为：

```text
P_1_A+V_1_A*V_1_A*1.1451-V_1_A*V_2_A*(1.1451*cos(D_1_A-D_2_A)+-3.3006*sin(D_1_A-D_2_A))+V_1_A*V_1_B*(-0.4859*cos(D_1_A-D_1_B)+1.2203*sin(D_1_A-D_1_B))-V_1_A*V_2_B*(-0.4859*cos(D_1_A-D_2_B)+1.2203*sin(D_1_A-D_2_B))+V_1_A*V_1_C*(-0.2661*cos(D_1_A-D_1_C)+0.9122*sin(D_1_A-D_1_C))-V_1_A*V_2_C*(-0.2661*cos(D_1_A-D_2_C)+0.9122*sin(D_1_A-D_2_C)):[0/0]
Q_1_A+-V_1_A*V_1_A*-3.3006+V_1_A*V_2_A*(-3.3006*cos(D_1_A-D_2_A)-1.1451*sin(D_1_A-D_2_A))+V_1_A*V_1_B*(-0.4859*sin(D_1_A-D_1_B)-1.2203*cos(D_1_A-D_1_B))+V_1_A*V_2_B*(1.2203*cos(D_1_A-D_2_B)--0.4859*sin(D_1_A-D_2_B))+V_1_A*V_1_C*(-0.2661*sin(D_1_A-D_1_C)-0.9122*cos(D_1_A-D_1_C))+V_1_A*V_2_C*(0.9122*cos(D_1_A-D_2_C)--0.2661*sin(D_1_A-D_2_C)):[0/0]
P_1_B+V_1_B*V_1_B*1.0027-V_1_B*V_2_B*(1.0027*cos(D_1_B-D_2_B)+-3.1276*sin(D_1_B-D_2_B))+V_1_B*V_1_A*(-0.4859*cos(D_1_B-D_1_A)+1.2203*sin(D_1_B-D_1_A))-V_1_B*V_2_A*(-0.4859*cos(D_1_B-D_2_A)+1.2203*sin(D_1_B-D_2_A))+V_1_B*V_1_C*(-0.1263*cos(D_1_B-D_1_C)+0.6967*sin(D_1_B-D_1_C))-V_1_B*V_2_C*(-0.1263*cos(D_1_B-D_2_C)+0.6967*sin(D_1_B-D_2_C)):[0/0]
Q_1_B+-V_1_B*V_1_B*-3.1276+V_1_B*V_2_B*(-3.1276*cos(D_1_B-D_2_B)-1.0027*sin(D_1_B-D_2_B))+V_1_B*V_1_A*(-0.4859*sin(D_1_B-D_1_A)-1.2203*cos(D_1_B-D_1_A))+V_1_B*V_2_A*(1.2203*cos(D_1_B-D_2_A)--0.4859*sin(D_1_B-D_2_A))+V_1_B*V_1_C*(-0.1263*sin(D_1_B-D_1_C)-0.6967*cos(D_1_B-D_1_C))+V_1_B*V_2_C*(0.6967*cos(D_1_B-D_2_C)--0.1263*sin(D_1_B-D_2_C)):[0/0]
P_1_C+V_1_C*V_1_C*0.8867-V_1_C*V_2_C*(0.8867*cos(D_1_C-D_2_C)+-2.9506*sin(D_1_C-D_2_C))+V_1_C*V_1_A*(-0.2661*cos(D_1_C-D_1_A)+0.9122*sin(D_1_C-D_1_A))-V_1_C*V_2_A*(-0.2661*cos(D_1_C-D_2_A)+0.9122*sin(D_1_C-D_2_A))+V_1_C*V_1_B*(-0.1263*cos(D_1_C-D_1_B)+0.6967*sin(D_1_C-D_1_B))-V_1_C*V_2_B*(-0.1263*cos(D_1_C-D_2_B)+0.6967*sin(D_1_C-D_2_B)):[0/0]
Q_1_C+-V_1_C*V_1_C*-2.9506+V_1_C*V_2_C*(-2.9506*cos(D_1_C-D_2_C)-0.8867*sin(D_1_C-D_2_C))+V_1_C*V_1_A*(-0.2661*sin(D_1_C-D_1_A)-0.9122*cos(D_1_C-D_1_A))+V_1_C*V_2_A*(0.9122*cos(D_1_C-D_2_A)--0.2661*sin(D_1_C-D_2_A))+V_1_C*V_1_B*(-0.1263*sin(D_1_C-D_1_B)-0.6967*cos(D_1_C-D_1_B))+V_1_C*V_2_B*(0.6967*cos(D_1_C-D_2_B)--0.1263*sin(D_1_C-D_2_B)):[0/0]
P_2_A+V_2_A*V_2_A*1.1451-V_2_A*V_1_A*(1.1451*cos(D_2_A-D_1_A)+-3.3006*sin(D_2_A-D_1_A))+V_2_A*V_2_B*(-0.4859*cos(D_2_A-D_2_B)+1.2203*sin(D_2_A-D_2_B))-V_2_A*V_1_B*(-0.4859*cos(D_2_A-D_1_B)+1.2203*sin(D_2_A-D_1_B))+V_2_A*V_2_C*(-0.2661*cos(D_2_A-D_2_C)+0.9122*sin(D_2_A-D_2_C))-V_2_A*V_1_C*(-0.2661*cos(D_2_A-D_1_C)+0.9122*sin(D_2_A-D_1_C)):[0/0]
Q_2_A+-V_2_A*V_2_A*-3.3006+V_2_A*V_1_A*(-3.3006*cos(D_2_A-D_1_A)-1.1451*sin(D_2_A-D_1_A))+V_2_A*V_2_B*(-0.4859*sin(D_2_A-D_2_B)-1.2203*cos(D_2_A-D_2_B))+V_2_A*V_1_B*(1.2203*cos(D_2_A-D_1_B)--0.4859*sin(D_2_A-D_1_B))+V_2_A*V_2_C*(-0.2661*sin(D_2_A-D_2_C)-0.9122*cos(D_2_A-D_2_C))+V_2_A*V_1_C*(0.9122*cos(D_2_A-D_1_C)--0.2661*sin(D_2_A-D_1_C)):[0/0]
P_2_B+V_2_B*V_2_B*1.0027-V_2_B*V_1_B*(1.0027*cos(D_2_B-D_1_B)+-3.1276*sin(D_2_B-D_1_B))+V_2_B*V_2_A*(-0.4859*cos(D_2_B-D_2_A)+1.2203*sin(D_2_B-D_2_A))-V_2_B*V_1_A*(-0.4859*cos(D_2_B-D_1_A)+1.2203*sin(D_2_B-D_1_A))+V_2_B*V_2_C*(-0.1263*cos(D_2_B-D_2_C)+0.6967*sin(D_2_B-D_2_C))-V_2_B*V_1_C*(-0.1263*cos(D_2_B-D_1_C)+0.6967*sin(D_2_B-D_1_C)):[0/0]
Q_2_B+-V_2_B*V_2_B*-3.1276+V_2_B*V_1_B*(-3.1276*cos(D_2_B-D_1_B)-1.0027*sin(D_2_B-D_1_B))+V_2_B*V_2_A*(-0.4859*sin(D_2_B-D_2_A)-1.2203*cos(D_2_B-D_2_A))+V_2_B*V_1_A*(1.2203*cos(D_2_B-D_1_A)--0.4859*sin(D_2_B-D_1_A))+V_2_B*V_2_C*(-0.1263*sin(D_2_B-D_2_C)-0.6967*cos(D_2_B-D_2_C))+V_2_B*V_1_C*(0.6967*cos(D_2_B-D_1_C)--0.1263*sin(D_2_B-D_1_C)):[0/0]
P_2_C+V_2_C*V_2_C*0.8867-V_2_C*V_1_C*(0.8867*cos(D_2_C-D_1_C)+-2.9506*sin(D_2_C-D_1_C))+V_2_C*V_2_A*(-0.2661*cos(D_2_C-D_2_A)+0.9122*sin(D_2_C-D_2_A))-V_2_C*V_1_A*(-0.2661*cos(D_2_C-D_1_A)+0.9122*sin(D_2_C-D_1_A))+V_2_C*V_2_B*(-0.1263*cos(D_2_C-D_2_B)+0.6967*sin(D_2_C-D_2_B))-V_2_C*V_1_B*(-0.1263*cos(D_2_C-D_1_B)+0.6967*sin(D_2_C-D_1_B)):[0/0]
Q_2_C+-V_2_C*V_2_C*-2.9506+V_2_C*V_1_C*(-2.9506*cos(D_2_C-D_1_C)-0.8867*sin(D_2_C-D_1_C))+V_2_C*V_2_A*(-0.2661*sin(D_2_C-D_2_A)-0.9122*cos(D_2_C-D_2_A))+V_2_C*V_1_A*(0.9122*cos(D_2_C-D_1_A)--0.2661*sin(D_2_C-D_1_A))+V_2_C*V_2_B*(-0.1263*sin(D_2_C-D_2_B)-0.6967*cos(D_2_C-D_2_B))+V_2_C*V_1_B*(0.6967*cos(D_2_C-D_1_B)--0.1263*sin(D_2_C-D_1_B)):[0/0]
V_1_A-7199.5579:[0/0]
D_1_A-0:[0/0]
V_1_B-7199.5579:[0/0]
D_1_B--2/3*pi:[0/0]
V_1_C-7199.5579:[0/0]
D_1_C-2/3*pi:[0/0]
P_2_A-1275000.0000:[0/0]
Q_2_A-790174.0000:[0/0]
P_2_B-1800000.0000:[0/0]
Q_2_B-871779.8000:[0/0]
P_2_C-2375000.0000:[0/0]
Q_2_C-780624.7000:[0/0]
V_1_A:[0/99999999],D_1_A:[-3.2/3.2],V_1_B:[0/99999999],D_1_B:[-3.2/3.2],V_1_C:[0/99999999],D_1_C:[-3.2/3.2],P_1_A:[-99999999/99999999],P_1_B:[-99999999/99999999],P_1_C:[-99999999/99999999],Q_1_A:[-99999999/99999999],Q_1_B:[-99999999/99999999],Q_1_C:[-99999999/99999999],V_2_A:[0/99999999],D_2_A:[-3.2/3.2],V_2_B:[0/99999999],D_2_B:[-3.2/3.2],V_2_C:[0/99999999],D_2_C:[-3.2/3.2],P_2_A:[-99999999/99999999],P_2_B:[-99999999/99999999],P_2_C:[-99999999/99999999],Q_2_A:[-99999999/99999999],Q_2_B:[-99999999/99999999],Q_2_C:[-99999999/99999999]
```

输入至easyslove，求解得到：

```text
V_1_A : 7199.5579
D_1_A : 0
V_1_B : 7199.5579
D_1_B : -2.0943951023931953
V_1_C : 7199.5579
D_1_C : 2.0943951023931953
P_1_A : -1277881.0174413954
P_1_B : -1809439.2939235603
P_1_C : -2381044.287784358
Q_1_A : -794494.3206242201
Q_1_B : -890721.1548199678
Q_1_C : -817091.6107552513
V_2_A : 7176.944412608694
D_2_A : -0.001431890501404535
V_2_B : 7139.609540861016
D_2_B : -2.1008076662494224
V_2_C : 7149.960093956684
D_2_C : 2.0813825033502016
P_2_A : 1275000
P_2_B : 1800000
P_2_C : 2375000
Q_2_A : 790174
Q_2_B : 871779.8
Q_2_C : 780624.7
```

二者结果相同。