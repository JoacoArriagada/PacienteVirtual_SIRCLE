# 1. **Title**: A human-patient simulation of a patient with undiagnosed congestive heart failure from diastolic dysfunction who presents preoperatively for an elective trans-urethral resection of the prostate (TURP). 

# 3. **Overview of the scenario**:

## 3.2. <u>Simulation</u>:

### 3.2.1. *The simulated patient* (written for a Laerdal® SimMan™) is in a preoperative holding area.

## 3.3. Environment

### 3.3.1. *Lab Set Up* –A pre-operative holding area

### 3.3.2. *Manikin Set Up* – This scenario was written for a SimManTM simulator.   We do not use moulage or make up to make the mannequin appear cyanotic, but that could be tried. The mannequin is sitting up right and unmonitored.  It has a respiratory rate of 24 breaths/minute, crackles for breath sounds, a BP of 180/110, an SpO2 of 90%, and a heart rate of 110 with normal sinus rhythm and left ventricular hypertrophy QRS morphology.  We would program an S4 heart sound, if that were available.

## 3.4. Actors

### 3.4.1. *Roles* –Holding-area nurse, patient voice, urologist, OR staff, attending anesthesiologist, and primary care physician.

### 3.4.3. *Action Role* 

#### 3.4.3.2. Patient: answers questions as per scripted H & P.

# 5. **Events and Programmed Responses:**

**Miscellaneous Handler**

**Event:** <u>History performed</u>

**Actions:** Denotes the time participants begin taking the history in the simulation log. Scripted answers to history questions are attached.

**Event:** <u>Physical examination</u> 

**Actions:** Denotes the time participants begin performing a physical exam in the simulation log. Starting vital signs and physical findings are previously described.  Note that SimManTM’s breath sounds are difficult to interpret.  After auscultating the lungs the simulation’s facilitator should ask what is heard and correct any misinterpretations.

If queried, the patient does not have prominent peripheral edema, jugular venous distension, or hepatojugular reflux.

**Event:** <u>Discuss findings and plan with the patient</u>

**Action:** The event is noted in the log.  The “patient” acknowledges the participants’ care and concern.

**Attachments**

    1. History
        1. HPI
            1. Does patient complain of dyspnea?  Yes
            2. Is there dyspnea only on exertion, or at rest too?  Both
            3. What is the time course of the dyspnea?  3 months.  I have been short of breath at rest only for a few days.
            4. What is the patient’s functional capacity?  I work on a farm, and am usually able to carry a bale of hay without stopping.  However, I cannot do what I once did because of my breathing, and lately I haven’t been able to work.
        2. PMH
            1. Is there any past medical history sir?  Hypertension
            2. Anything else, such as heart attack or chest pain or diabetes?  No
            3. Emphysema or asthma or COPD?  No
            4. Ever a smoker?  As a teen, none for 60 years.
            5. Any other lung diseases (IPF, pneumonia, chemical or dust exposure, etc.)?  No.
            6. Any kidney or liver problems?  No.
        3. PSH
            1. Any surgeries in the past?  Inguinal hernia repair 20 years ago
            2. Any anesthetic complications?  No.
        4. Medications
            1. What medications do you take?  Lisinopril 25 mg PO daily
            2. When did you start?  5 years ago.
            3. And you’ve never had a cough?  No.
            4. Has your tongue every swollen on you or have you gotten all puffy?  No.
        5. Allergies
            1. Any allergies?  None
        6. FH and SH
            1. Do you smoke?  No
            2. Have you ever smoked?  No
            3. Do you drink or have you ever?  Yes.
            4. What’s occasional?  1 beer a day.
            5. Any illicit drug use?  No.
            6. Employment?  Farmer
            7. Living conditions?  Three story house
            8. Relationship status?  Married with 3 grown kids.
            9. Any family history for MI, diabetes, CHF, CP, ACS or CAD?  No.
            10. Any family history of cancer?  No.
            11. Any family history of asthma?  No.
        7. ROS
            1. Viral infection or other illness, syncopcal episodes, coughing, mucus, CP, dyspnea on exertion, PND, nocturia
                1. The patient complains of PND and sleeps on 3 pillows
                2. The patient complains of nocturia, usually getting up three times per sleeping.
                3. Dyspnea on exertion as above.
            2. No coughing, mucus production, sickness or sick contacts, CP, orthopnea, MI, or history of CHF
