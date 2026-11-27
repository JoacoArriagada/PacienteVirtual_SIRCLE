# Simulation Interest Group Scenario Template

# I. Title: Anesthesia Simulation Case 6 - 22 year old for appendectomy

Topic: Anaphylaxis Last Modified: 12/24/2008

IV. Case Narrative

Scenario Background Given to Participants (must request Preoperative
evaluation)
1. Chief complaint, triage note, medic report - 22 year old healthy, but anxious, male for appendectomy. The patient complains of lower abdominal pain for the last 12 hours.
2. Past medical history Illnesses: hay fever Surgeries: none
3. Meds and allergies Meds: none, except occasional antihistamines Allergies: penicillin caused wheal and flare but not shortness of breath or hypotension. Habits: smokes, drinks, and cusses. Denies intravenous drugs.
4. Family/social history - No anesthetic complications
5. Physical Exam Thin, fit appearing male, anxious and in mild distress Wt : 74 kg T : 37.2 HR : 108 BP : 126/60 RR : 20 Airway: normal Lungs: clear Heart: normal
6. Labs Hematocrit: 42 White blood cell count: 12.2 Electrolytes, Glucose, Blood Urea Nitrogen, creatinine : normal
7. NPO Status Last ate 13 hours ago
8. Patient and surgeon requests: The patient hates needles and requests general anesthesia. The patient received no preoperative medications. The procedure will take 30 minutes.

B. Scenario conditions initially: State: AWAKE

1. History patient gives: The patient is awake and responds to voice commands. The patient is moving spontaneously.

2. Patient’s initial exam: The breath sounds are clear bilaterally. The pulse is strong and regular. The skin is warm and dry with normal capillary refill.

3. Patient’s physiology: HR 88, BP 140/80, spontRR 16

Transitions:
c) To State: Cough5 1) If Try to place ETT

C. Scenario branch point: State: SedatedIV1

1. History patient gives: The patient has a lethargic response to voice. The patient is moving spontaneously.
2. Patient’s exam: The breath sounds are clear bilaterally. The pulse is strong and regular. The skin is warm and dry with normal capillary refill.
3. Patient’s physiology: HR 84, BP 135/80, spontRR 12

5. Transitions:
a) To State: Awake 1) If depth of anesthesia $< 0 . 1$
b) To State: SedatedIV2 1) If depth of anesthesia $> 0 . 5$
c) To State: Cough5 1) If Try to place LMA
d) To State: Cough5 1) If Try to place ETT8
e) To State: AwakePara 1) If Neuromuscular blockade $> 0 . 8$
f) To State: Hurt5 1) If Make incision

D. Scenario branch point: State: SedatedIV2

1. History patient gives: The patient has a lethargic response to voice. The patient is moving spontaneously.
2. Patient’s exam: The breath sounds are clear bilaterally. The pulse is strong and regular. The skin is warm and dry with normal capillary refill.
3. Patient’s physiology: HR 84, BP 135/80, spontRR 4

5. Transitions:
a) To State: SedatedIV1 1) If depth of anesthesia $< 0 . 5$
b) To State: AsleepIV 1) If depth of anesthesia $> 0 . 8$
c) To State: Cough4 1) If Try to place LMA
d) To State: Cough4 1) If Try to place ETT8
e) To State: Hurt4 1) If Make incision

E. Scenario branch point: State: AsleepIV

1. History patient gives: The patient is unresponsive.

2. Patient’s exam: The breath sounds are equal bilaterally. The pulse is present. The skin is warm and dry with normal capillary refill.

3. Patient’s physiology: HR 76, BP 122/68, spontRR 0

5. Transitions:
a) To State: SedatedIV2 1) If depth of anesthesia $< 0 . 8$
b) To State: awIV 1) If Try to place ETT
c) To State: awIV 1) If Try to place LMA
d) To State: Hurt3 1) If Make incision

F. Scenario branch point: State: awIV

1. History patient gives: The patient is unresponsive.

2. Patient’s exam: The breath sounds are equal bilaterally. The pulse is present. The skin is warm and dry with normal capillary refill.

3. Patient’s physiology: HR 82, BP 132/78, spontRR 0

5. Transitions:
a) To State: GA2IV 1) If depth of anesthesia $> 2 . 2$
To State: Hurt1 1) If Make incision
c) To State: desatBrady1 1) If O2sat $< 6 3$

G. Scenario branch point: State: GA2IV

1. History patient gives: The patient is unresponsive.

2. Patient’s exam: The breath sounds are equal bilaterally. The pulse is present. The skin is warm and dry with normal capillary refill.

3. Patient’s physiology: HR 80, BP 122/68, spontRR 0

5. Transitions:
a) To State: opIV 1) If Make Incision
b) To State: desatBrady1 1) If O2sat $< 6 3$

H. Scenario branch point: State: opIV

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: The breath sounds are equal bilaterally. The pulse is present. The skin is warm and dry with normal capillary refill.
3. Patient’s physiology: HR 88, BP 134/78, spontRR 0
5. Transitions:
a) To State: tachyIV 1) If Time $> 3 0 0$ sec
b) To State: desatBrady1 1) If O2sat $< 6 3$

I. Scenario branch point: State: tachyIV

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: The breath sounds are equal bilaterally. The pulse is present. The skin is warm and dry with normal capillary refill.
3. Patient’s physiology: HR 106, BP 114/68, spontRR 0

5. Transitions:
a) To State anaph1 1) If Time $> 6 0$ sec
b) To State: desatBrady1 1) If O2sat $< 6 3$

J. Scenario branch point: State: anaph1

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There are a few scattered wheezes present. The pulse is rapid and weak. There is a red rash spreading up the arm from the IV site.
3. Patient’s physiology: HR 118, BP 88/62, spontRR 0
5. Transitions:
a) To State anaph2 1) If Time $> 6 0$ sec
b) To State: desatBrady1 1) If O2sat $< 6 3$

K. Scenario branch point: State: anaph2

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There are a few scattered wheezes present. The pulse is rapid and weak. There is a red rash spreading up the arm from the IV site.
3. Patient’s physiology: HR 134, BP 62/45, spontRR 0
5. Transitions:
a) To State anaph3 1) If Time $> 6 0$ sec
b) To State: desatBrady1 1) If O2sat $< 6 3$

L. Scenario branch point: State: anaph3

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is severe wheezing throughout. The pulse is not palpable. The red rash is now mottled and purple.
3. Patient’s physiology: HR 128, BP 52/36, spontRR 0
5. Transitions:
a) To State awSecured 1) If Tube in place and Time $> 3 0$
b) To State HardVent 1) If Tube is not in place and Time $> 3 0$
c) To State: desatBrady1 1) If O2sat $< 6 3$

M. Scenario branch point: State: HardVent

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is severe wheezing throughout. The pulse is not palpable. The red rash is now ashen gray. There is periorbital edema.
3. Patient’s physiology: HR 132, BP 48/26, spontRR 0
5. Transitions:
a) To State: desatBrady1 1) If Time $> 1 2 0$ sec

N. Scenario branch point: State: awSecured

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is severe wheezing throughout. The pulse is not palpable. The red rash is now mottled and purple. There is periorbital edema.
3. Patient’s physiology: HR 128, BP 52/36, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Fluid1 1) If tFluid $> 9 9 9$
c) To State Ep1 1) If BDRUGEpinephrine $> ~ 4 9$ and BDRUGEpinephrine $< ~ 2 0 1$
d) To State Vtach1 1) If BDRUGEpinephrine $>$ 200
e) To State: desatBrady1 1) If O2sat $< 6 3$

O. Scenario branch point: State: ep1

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is moderate wheezing throughout. The pulse is thready. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 126, BP 62/46, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Ep1fluid1 1) If tFluid $> 9 9 9$
c) To State Ep2 1) If BDRUGEpinephrine $> ~ 4 9$ and BDRUGEpinephrine $< ~ 4 0 1$
d) To State Vtach2 1) If BDRUGEpinephrine $>$ 400
e) To State: desatBrady1 1) If O2sat $< 6 3$

P. Scenario branch point: State: ep2

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is mild wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 124, BP 72/48, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Ep2fluid1 1) If tFluid $> 9 9 9$
d) To State Vtach2 1) If BDRUGEpinephrine $>$ 400
e) To State: desatBrady1 1) If O2sat $< 6 3$

Q. Scenario branch point: State: Fluid1

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is mild wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 120, BP 62/46, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Fluids2 1) If tFluid $> 1 9 9 9$
c) To State Ep1fluid1 1) If BDRUGEpinephrine $>$ 49 and BDRUGEpinephrine $< ~ 2 0 1$
d) To State Vtach1 1) If BDRUGEpinephrine $>$ 400
e) To State: desatBrady1 1) If O2sat $< 6 3$

R. Scenario branch point: State: Ep1fluid1

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is moderate wheezing throughout. The pulse is thready. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 122, BP 72/50, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Ep1fluids2 1) If tFluid $> 1 9 9 9$
c) To State Ep1fluid1 1) If BDRUGEpinephrine $> ~ 4 9$ and BDRUGEpinephrine $< ~ 4 0 1$
d) To State Vtach2 1) If BDRUGEpinephrine $>$ 400
e) To State: desatBrady1 1) If O2sat $< 6 3$

S. Scenario branch point: State: Ep2fluid1

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is mild wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 118, BP 82/54, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Ep2fluids2 1) If tFluid $> 1 9 9 9$
c) To State Vtach2 1) If BDRUGEpinephrine $>$ 400
d) To State: desatBrady1 1) If O2sat $< 6 3$

T. Scenario branch point: State: Fluid2

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is severe wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 118, BP 72/54, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Ep1fluids2 1) If BDRUGEpinephrine $>$ 49 and BDRUGEpinephrine $< ~ 2 0 1$
c) To State Vtach1 1) If BDRUGEpinephrine $>$ 200
d) To State: desatBrady1 1) If O2sat $< 6 3$

U. Scenario branch point: State: Ep1fluid2

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is moderate wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 116, BP 82/58, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Ep2fluids2 1) If BDRUGEpinephrine $>$ 49 and BDRUGEpinephrine $< ~ 4 0 1$
c) To State Vtach2 1) If BDRUGEpinephrine $>$ 200
d) To State: desatBrady1 1) If O2sat $< 6 3$

V. Scenario branch point: State: Ep2fluid2

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is mild wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 114, BP 92/64, spontRR 0
5. Transitions:
a) To State HardVent 1) If Time $> 3 6 0$
b) To State Stable 1) If BDRUGDiphenhydramine $> ~ 4 9$ and BDRUGAlbuterol $> 1$
c) To State Vtach2 1) If BDRUGEpinephrine $>$ 400
d) To State: desatBrady1 1) If O2sat $< 6 3$

W. Scenario branch point: State: Stable

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is mild wheezing throughout. The pulse is present. The red rash is now mottled. There is periorbital edema.
3. Patient’s physiology: HR 114, BP 92/64, spontRR 0
5. Transitions: This is a final state.

X. Scenario branch point: State: Vtach1

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is severe wheezing throughout. The pulse is absent. The red rash is now mottled and purple. There is periorbital edema.
3. Patient’s physiology: HR 142, BP 52/36, spontRR 0
5. Transitions:
a) To State Ep1 1) If Defib $> 1 9 9$
b) To State Ep1 1) If Time $> 3 0$

Y. Scenario branch point: State: Vtach2

1. History patient gives: The patient is unresponsive.
2. Patient’s exam: There is severe wheezing throughout. The pulse is absent. The red rash is now mottled and purple. There is periorbital edema.
3. Patient’s physiology: HR 142, BP 52/36, spontRR 0
5. Transitions:
a) To State Ep1 1) If Defib $> 1 9 9$
b) To State Ep1 1) If Time $> 3 0$
