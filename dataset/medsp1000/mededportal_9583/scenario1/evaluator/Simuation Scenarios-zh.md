# 新生儿复苏模拟场景

# 场景 #1：

母亲已用力分娩 6 小时。产科医生真空助产失败 3 次。监护仪提示胎心减速延长（在 50 次/分区间，持续 $> 5$ 分钟）。母亲被送往急诊剖宫产。产前病史其余无特殊。羊水清。婴儿胎龄 39 周，EFW= 3kg

# 参与者提示：

医生- 通过 ETT（0.5ml/kg）和 IV（0.5ml/kg）均给予 1.5ml（1:10,000）Epi；护士- 使用双指技术进行缓慢且浅表的胸外按压

初始状态：‘L1’（见下）羊水清  足月  发绀/苍白  心率：40  无自主呼吸  肌张力低下

# 场景要求：

1. 初始步骤   
2. 氧疗   
3. PPV   
4. 胸外按压   
5. $+ / -$ 插管   
6. 脐静脉导管置入   
7. IV 肾上腺素

# 改善标准：

婴儿将保持在 L1，直到给予 IV Epi。   
一旦给予 IV Epi，婴儿改善至‘L3’，场景结束。

# SimNewB 分级图：

IV Epi 后由 L1 改善至 L3

<table><tr><td rowspan=1 colspan=1>PATIENTLEVEL</td><td rowspan=1 colspan=1>CYANOSIS</td><td rowspan=1 colspan=1> HEART RATE</td><td rowspan=1 colspan=1>MuscLETONE</td><td rowspan=1 colspan=1>ReSPIR- ATIONS</td><td rowspan=1 colspan=1>VOCAL SOUNDS</td><td rowspan=1 colspan=1>LUNG SOUNDS</td><td rowspan=1 colspan=1>BLooD PRESSURE</td></tr><tr><td rowspan=1 colspan=1>L5</td><td rowspan=1 colspan=1>Off</td><td rowspan=1 colspan=1>140/min</td><td rowspan=1 colspan=1>Motion</td><td rowspan=1 colspan=1>Regular40/min</td><td rowspan=1 colspan=1>Strongcry</td><td rowspan=1 colspan=1>Normal</td><td rowspan=1 colspan=1>60/40</td></tr><tr><td rowspan=1 colspan=1>L4</td><td rowspan=1 colspan=1>On</td><td rowspan=1 colspan=1>160/min</td><td rowspan=1 colspan=1>Motion</td><td rowspan=1 colspan=1>Regular60/min</td><td rowspan=1 colspan=1>Grunting</td><td rowspan=1 colspan=1>Coarsecrackles</td><td rowspan=1 colspan=1>60/40</td></tr><tr><td rowspan=1 colspan=1>L3</td><td rowspan=1 colspan=1>On</td><td rowspan=1 colspan=1>120/min</td><td rowspan=1 colspan=1>Tone</td><td rowspan=1 colspan=1>Irregular10/min</td><td rowspan=1 colspan=1>Weakcry</td><td rowspan=1 colspan=1>Coarsecrackles</td><td rowspan=1 colspan=1>45/20</td></tr><tr><td rowspan=1 colspan=1>L2</td><td rowspan=1 colspan=1>On</td><td rowspan=1 colspan=1>70/min</td><td rowspan=1 colspan=1> Limp</td><td rowspan=1 colspan=1>Gasping 4/min</td><td rowspan=1 colspan=1></td><td rowspan=1 colspan=1>Coarsecrackles</td><td rowspan=1 colspan=1>30/20</td></tr><tr><td rowspan=1 colspan=1>LI</td><td rowspan=1 colspan=1>On</td><td rowspan=1 colspan=1>40/min</td><td rowspan=1 colspan=1> Limp</td><td rowspan=1 colspan=1>0</td><td rowspan=1 colspan=1></td><td rowspan=1 colspan=1>=</td><td rowspan=1 colspan=1>30/20</td></tr><tr><td rowspan=1 colspan=1>LO</td><td rowspan=1 colspan=1>On</td><td rowspan=1 colspan=1>0</td><td rowspan=1 colspan=1> Limp</td><td rowspan=1 colspan=1>0</td><td rowspan=1 colspan=1></td><td rowspan=1 colspan=1></td><td rowspan=1 colspan=1>0/0</td></tr></table>
