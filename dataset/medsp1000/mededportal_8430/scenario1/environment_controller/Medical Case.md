Simulation Interest Group Scenario Template

# **Title: Gynecology Simulation Case 1 - 24 year old female with Pelvic Pain and Vaginal Bleeding**

Topic: Female with Vaginal Bleeding and Pelvic Pain

Last Modified: 11/9/2010

**Case Narrative**

Scenario Background Given to Participants

Chief complaint, triage note, medic report - 24 year old healthy, but anxious, female with vaginal bleeding. The patient complains of lower abdominal pain for the last 3 days, worse in the last 6 hours.

Past medical history
Illnesses: mild asthma
Surgeries: none

Meds and allergies
Meds: albuterol inhaler as needed
Allergies: NKDA
Habits: denies smoking and illicit drug use, drinks alcohol 4 times per week, 3-4 drinks at a time.

OB/Gyn Hx: never been pregnant, has regular periods but isn’t sure when her last period was. Has no history of sexually transmitted diseases. Never had a Pap smear.

Physical Exam
Thin, fit appearing female, anxious and in mild distress
Wt : 53 kg T : 37.2 HR : 95 BP : 100/60 RR : 20
Lungs: clear
Heart: normal

Abdomen: slightly distended, tender to palpation in bilateral lower quadrants with questionable rebound tenderness. No masses, HSM, or hernia.

PV: not done

Pelvic Ultrasound
Uterus: 7.3 cm x 3.5 cm x 4.4 cm
Endometrium 2.9 mm with no intrauterine pregnancy
Left ovary: 2.2 x 3.8 x 3.4 cm
Right ovary: 3.3 x 1.5 x 3.8 cm
Free fluid in cul-de-sac: minimal

Labs
Urine Pregnancy Test: positive
Hematocrit: 35
White blood cell count: 10.8
Electrolytes, Glucose, Blood Urea Nitrogen, creatinine : normal

NPO Status
Last ate 4 hours ago

State: Arrive
History patient gives: The patient is uncomfortable but complies with all requests.
Exam: Abdomen is slightly distended, tender to palpation in bilateral lower quadrants with questionable rebound tenderness without masses, hepatospenomegaly, or hernia.
Visual speculum exam: Nulliparous cervix with small amount of bleeding from the cervical os.
Message given if Palpation of the Uterus/Adnexa: It is not appropriate to perform a pelvic exam at this time.
Phys: HR 105, 90/ 60
What Next: Perform quick history and physical exam. Do not perform brisk palpation of the uterus/adnexa at this time. Establish IV access and administer fluids. Order pregnancy test and pelvic ultrasound.
Transitions:
To state: Diagnosis
If: ultrasound & pregnancy Test
To state: Fluids
If: tFluid > 199 and tFluid < 2001
To state: Fluids
If: tFluid > 2000
To state: Medical Management
If: methotrexate administered
To state: Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Delay
If: >300 sec elapses

State: Diagnosis
History patient gives: The patient's pain persists.
Exam: The abdomen continues to be tender to direct palpation.
Phys: BP 100/60 HR 95
What Next: Establish IV access and administer fluids.
Transitions:
To state: Diagnosis with Fluids
If: tFluid > 199 and tFluid < 2001
To state: Diagnosis with Fluids
If: Fluids > 2000 cc were administered
To state: Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Medical Management
If: methotrexate administered
To state: Delay Diagnosis
If: Fluids > 300 sec elapses

State: Fluids
History patient gives: The patient's pain persists.
Exam: The abdomen continues to be tender.
Phys: BP 100/70 HR 85
What Next: Perform pregnancy test and pelvic ultrasound.
Transitions:
To state: Diagnosis with Fluids
If: ultrasound & pregnancy Test
To state: Fluids with Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Fluids with Medical Management
If: Methotrexate administered
To state: Delay Fluids
If: >300 sec elapses

State: Delay
History patient gives: The patient's pain persists and she is tiring of the Emergency Room
Exam: The abdomen is tenderness to direct palpation. There is no rebound tenderness.
Phys: BP 95/65 HR 90
What Next: Perform pregnancy test and pelvic ultrasound. Establish IV access and administer fluids
Transitions:
To state: Delay & Diagnosis
If: ultrasound & pregnancy test
To state: Delay & Fluids
If: tFluid > 199 and tFluid < 2001
To state: Delay & Fluids
If: tFluid > 2000
To state: Medical Management
If: methotrexate administered
To state: Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Critical Delay
If: >300 seconds elapse

State: Diagnosis with Fluids
History patient gives: The patient's pain continues.
Exam: The abdomen remains tender to direct palpation.
Phys: BP 100/70 HR 95
What Next: Consider the treatment options for the ectopic pregnancy.
Transitions:
To state: Fluids with Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Fluids with Medical Management
If: Methotrexate administered
To state: Delay & Fluids
If: > 300 sec elapses

State: Delay Diagnosis
History patient gives: The patient's pain persists and she is very tired of the Emergency Room.
Exam: The abdomen has continued tenderness to direct palpation, but is stable.
Phys: BP 95 100/70
What Next: Consider intravenous fluids and management options.
Transitions:
To state: Diagnosis with Fluids
If: tFluid > 199 and tFluid < 2001
To state: Diagnosis with Fluids
If: Fluids > 2000 cc were administered
To state: Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Medical Management
If: methotrexate administered
To state: Critical Delay & Diagnosis
If: >300 sec elapse

State: DELAY with FLUIDS
History patient gives: The patient's pain persists.
Exam:
Phys: BP 100/60 HR 85
What Next: Perform pregnancy test and pelvic ultrasound.
Transitions: The abdomen is tender to direct palpation.
To state: Diagnosis with Fluids
If: perform pelvic ultrasound and pregnancy test
To state: Fluids with Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Fluids with Medical Management
If: Methotrexate administered
To state: CRITICAL DELAY
If: >300 seconds elapse

EndState: Surgical Management
History patient gives: Patient is recovering from anesthesia.
Exam: Patient is recovering from anesthesia.
Phys: BP 90/60 HR 90
What Next: End of case simulation.

EndState: Fluids with Surgical Management
History patient gives: Patient is recovering from anesthesia.
Exam: Patient is recovering from anesthesia.
Phys: BP 110/80 HR 92
What Next: End of case simulation.

State: Critical Delay
History patient gives: The patient's pain persists.
Phys: BP 100/80 HR 90
What Next: Establish IV access and administer fluids. Perform pregnancy test and pelvic ultrasound.
Transitions:
To state: Critical Delay then Diagnosis
If: perform pelvic ultrasound and pregnancy test
To state: Delay with Fluids
If: Fluids >199cc and <2001cc
To state: Delay with Fluids
If: Fluids >2000cc
To state: Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Medical Management
If: methotrexate administered
To state: Inappropriate Delay
If: >300 seconds elapse

State: Critical Delay & Diagnosis
History patient gives: The patient's pain persists.
Exam: The abdomen remains tender to direct palpation.
Phys: BP 95/65 HR 85
What Next: Establish IV access and administer fluids.
Transitions:
To state: Diagnosis with Fluids
If: tFluid > 199 and tFluid < 2001
To state: Diagnosis with Fluids
If: Fluids > 2000 cc were administered
To state: Surgical Management
If: salpingectomy/salpingostomy ordered
To state: Medical Management
If: methotrexate administered
To state: Inappropriate Delay
If: >300 seconds elapse

END State: Inappropriate Delay
History patient gives: The patient is leaving the Emergency Room because of excessive delay. She still has significant abdominal pain.
Exam: Unchanged.
Phys: BP 95/65 HR 100

EndState: Medical Management
History patient gives: Patient has received treatment and understands the risks and warnings with this treatment. She will follow in the clinic to monitor her response to treatment.
Exam: Unchanged
Phys: BP 90/60 HR 90
What Next: End of case simulation.

EndState: Fluids with Medical Management
History patient gives: Patient has received treatment and understands the risks and warnings with this treatment. She will follow in the clinic to monitor her response to treatment.
Exam: Unchanged
Phys: BP 90/60 HR 90
What Next: End of case simulation.
